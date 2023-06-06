use std::{ thread::sleep, time::Duration };

use async_trait::async_trait;

use crate::{ network::client::{ TaskPoller }, common::status::StatusEnum };
use reqwest::{ Error };

use super::worker::ConductorWorker;

#[async_trait]
pub trait Executor {
    async fn poll_and_work(&self) -> Result<(), Error>;
    async fn start(&self) -> Result<(), Error> {
        loop {
            self.poll_and_work().await?;
        }
    }
}

pub struct TaskExecutor {
    pub task_provider: Box<dyn TaskPoller + Send + Sync>,
    pub workers: Vec<Box<dyn ConductorWorker + Send + Sync>>,
    pub poll_interval: u64,
}

impl TaskExecutor {
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
impl Executor for TaskExecutor {
    async fn poll_and_work(&self) -> Result<(), Error> {
        for worker in self.workers.iter() {
            let task = self.task_provider.poll(worker.get_task_type().as_str()).await?;
            if task.is_none() {
                continue;
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
                    task.reason_for_incompletion = Some(err.to_string());
                    self.task_provider.update(task).await?;
                }
            }
        }

        Ok(())
    }

    async fn start(&self) -> Result<(), Error> {
        loop {
            self.poll_and_work().await?;
            sleep(Duration::from_millis(self.poll_interval));
        }
    }
}
#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use async_trait::async_trait;
    use serde_json::Value;

    use crate::{
        worker::{ worker::ConductorWorker, executor::{ TaskExecutor, Executor } },
        network::client::ConductorClient,
    };

    use tokio::runtime::Runtime;

    #[test]
    fn test_task_executor_poll_and_work() {
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
        let executor = TaskExecutor::new(client, vec![Box::new(mock_worker)], 1000);
        let rt = Runtime::new().unwrap();
        let error = rt.block_on(executor.poll_and_work()).err();
        assert!(error.is_none());
    }
    fn build_test_conductor_client() -> ConductorClient {
        let host = Box::new("localhost");
        let port = Box::new(8080_u32);
        ConductorClient::new(&host, &port, None)
    }
}
