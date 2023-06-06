use std::{ thread::sleep, time::Duration, ops::Deref };

use async_trait::async_trait;
use tokio::runtime::Runtime;

use crate::{ network::client::TaskPoller, common::enums::StatusEnum };
use reqwest::{ Error };

use super::worker::ConductorWorker;

#[async_trait]
pub trait TaskExecutor {
    async fn poll_and_work(
        &self,
        worker: &(dyn ConductorWorker + Send + Sync)
    ) -> Result<(), Error>;
    fn start(&self) -> Result<(), Error>;
}

pub struct WorkerManager {
    pub task_provider: Box<dyn TaskPoller + Send + Sync>,
    pub workers: Vec<Box<dyn ConductorWorker + Send + Sync>>,
    pub poll_interval: u64,
}

impl WorkerManager {
    pub fn new(
        task_provider: Box<dyn TaskPoller + Send + Sync>,
        workers: Vec<Box<dyn ConductorWorker + Send + Sync>>,
        poll_interval: u64
    ) -> Self {
        Self {
            task_provider,
            workers,
            poll_interval,
        }
    }
}

#[async_trait]
impl TaskExecutor for WorkerManager {
    async fn poll_and_work(
        &self,
        worker: &(dyn ConductorWorker + Send + Sync)
    ) -> Result<(), Error> {
        let task = self.task_provider.poll(worker.get_task_type().as_str()).await?;
        if task.is_none() {
            return Ok(());
        }
        let mut task = task.unwrap();
        let output = worker.execute(&task.input_data).await;
        match output {
            Ok(value) => {
                println!("Task '{}' completed", task.task_id.clone().unwrap());
                task.output_data = value;
                task.status = Some(StatusEnum::Completed);
                self.task_provider.update(task).await?;
            }
            Err(err) => {
                println!("Task failed with error: {}", err);
                task.status = Some(StatusEnum::Failed);
                task.reason_for_incompletion = Some(err);
                self.task_provider.update(task).await?;
            }
        }
        Ok(())
    }

    fn start(&self) -> Result<(), Error> {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                for worker in self.workers.iter() {
                    let dereferenced_worker: &(dyn ConductorWorker + Send + Sync) = worker.deref();
                    self.poll_and_work(dereferenced_worker).await?;
                }
                sleep(Duration::from_millis(self.poll_interval));
            }
        })
    }
}
#[cfg(test)]
mod unit_tests {
    #[test]
    #[cfg(all(test, feature = "integration_test"))]
    fn test_task_executor_poll_and_work() {
        use std::collections::HashMap;

        use async_trait::async_trait;
        use serde_json::Value;

        use crate::{
            worker::{ worker::ConductorWorker, executor::{ WorkerManager, TaskExecutor } },
            network::client::ConductorClient,
        };

        fn build_test_conductor_client() -> ConductorClient {
            let host = Box::new("localhost");
            let port = Box::new(8080_u32);
            ConductorClient::new(&host, &port, None)
        }
        struct MockWorker {
            task_type: String,
        }
        impl MockWorker {
            fn new(task_type: &str) -> Self {
                Self {
                    task_type: task_type.to_string(),
                }
            }
        }
        #[async_trait]
        impl ConductorWorker for MockWorker {
            fn get_task_type(&self) -> String {
                self.task_type.clone()
            }
            async fn execute(
                &self,
                _task_input: &Option<HashMap<String, Option<Value>>>
            ) -> Result<Option<HashMap<String, Option<Value>>>, String> {
                Ok(None)
            }
        }
        let mock_worker = MockWorker::new("simple_task_0");
        let client = Box::new(build_test_conductor_client());
        let executor = WorkerManager::new(client, vec![Box::new(mock_worker)], 1000);

        executor.start()
    }
}
