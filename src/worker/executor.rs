use std::{ ops::Deref, sync::Arc, thread::sleep, time::Duration };

use async_trait::async_trait;
use tokio::{ runtime::Runtime, sync::watch };

use crate::{ common::enums::StatusEnum, network::traits::task_poller::TaskPoller };
use reqwest::Error;

use super::worker::ConductorWorker;

#[async_trait]
pub trait TaskExecutor {
    async fn poll_and_work(
        &self,
        worker: &(dyn ConductorWorker + Send + Sync)
    ) -> Result<(), Error>;
    fn start(&self) -> Result<(), Error>;
    fn start_in_thread(
        self: Arc<Self>,
        stop_tx: watch::Receiver<()>
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct WorkerManager {
    pub task_provider: Box<dyn TaskPoller + Send + Sync>,
    pub workers: Vec<Arc<dyn ConductorWorker + Send + Sync>>,
    pub poll_interval: u64,
}

impl WorkerManager {
    pub fn new(
        task_provider: Box<dyn TaskPoller + Send + Sync>,
        workers: Vec<Arc<dyn ConductorWorker + Send + Sync>>,
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

    // Ignore coverage for this blocking function
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

    fn start_in_thread(
        self: Arc<Self>,
        stop_tx: watch::Receiver<()>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let self_clone = std::sync::Arc::clone(&self);

        tokio::task::spawn(async move {
            let stop_rx = stop_tx.clone();
            loop {
                if stop_rx.has_changed().unwrap() {
                    break;
                }
                for worker in self_clone.workers.iter() {
                    let dereferenced_worker: &(dyn ConductorWorker + Send + Sync) = worker.deref();
                    self.poll_and_work(dereferenced_worker).await.unwrap();
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(self.poll_interval)).await;
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use std::{ collections::HashMap, ops::Deref, sync::Arc };

    use async_trait::async_trait;
    use serde_json::Value;

    use crate::{
        network::client::ConductorClient,
        worker::{ executor::{ TaskExecutor, WorkerManager }, worker::ConductorWorker },
    };

    #[derive(Debug)]
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
    #[test]
    fn test_worker_manager_new() {
        let mock_worker = MockWorker::new("simple_task_0");

        let task_provider = Box::new(ConductorClient::new_with_url("http://localhost:8080", None));
        let poll_interval = 1000;
        let worker_manager = WorkerManager::new(
            task_provider,
            vec![Arc::new(mock_worker)],
            poll_interval
        );

        assert_eq!(worker_manager.poll_interval, 1000);
    }

    #[tokio::test]
    async fn test_worker_manager_poll_and_work() {
        let mut server = mockito::Server::new_async().await;

        let task_input =
            r#"{
            "taskDefName": "simple_task_1",
            "inputData": {
                "input_1": "value_1"
            },
            "taskId": "task_id_1"
        }"#;

        let task_input_url = "/api/tasks/poll/simple_task_1";
        let task_input_response = server
            .mock("GET", task_input_url)
            .with_status(200)
            .with_body(task_input)
            .create();

        let task_output =
            r#"{
            "taskDefName": "simple_task_1",
            "status": "COMPLETED",
            "outputData": {
                "output_1": "value_1"
            },
            "taskId": "task_id_1"
        }"#;

        let task_output_url = "/api/tasks";
        let task_output_response = server
            .mock("POST", task_output_url)
            .with_status(200)
            .with_body(task_output)
            .create();

        let mock_worker = MockWorker::new("simple_task_1");

        let task_provider = Box::new(
            ConductorClient::new_with_url(
                format!("http://{}", server.host_with_port()).as_str(),
                None
            )
        );
        let poll_interval = 1000;
        let worker_manager = WorkerManager::new(
            task_provider,
            vec![Arc::new(mock_worker)],
            poll_interval
        );

        worker_manager.poll_and_work(worker_manager.workers[0].deref()).await.unwrap();

        task_input_response.assert();
        task_output_response.assert();
    }

    #[tokio::test]
    async fn test_worker_manager_poll_and_work_no_task() {
        let mut server = mockito::Server::new_async().await;

        let task_input_url = "/api/tasks/poll/simple_task_2";
        let task_input_response = server
            .mock("GET", task_input_url)
            .with_status(200)
            .with_body("null")
            .create();

        let mock_worker = MockWorker::new("simple_task_2");

        let task_provider = Box::new(
            ConductorClient::new_with_url(
                format!("http://{}", server.host_with_port()).as_str(),
                None
            )
        );
        let poll_interval = 1000;
        let worker_manager = WorkerManager::new(
            task_provider,
            vec![Arc::new(mock_worker)],
            poll_interval
        );

        worker_manager.poll_and_work(worker_manager.workers[0].deref()).await.unwrap();

        task_input_response.assert();
    }

    #[tokio::test]
    async fn test_worker_manager_poll_and_work_failed_task() {
        let mut server = mockito::Server::new_async().await;

        let task_input =
            r#"{
            "taskDefName": "simple_task_3",
            "inputData": {
                "input_1": "value_1"
            },
            "taskId": "task_id_3"
        }"#;

        let task_input_url = "/api/tasks/poll/simple_task_3";
        let task_input_response = server
            .mock("GET", task_input_url)
            .with_status(200)
            .with_body(task_input)
            .create();

        let task_output_url = "/api/tasks";
        let task_output_response = server.mock("POST", task_output_url).with_status(500).create();

        let mock_worker = MockWorker::new("simple_task_3");

        let task_provider = Box::new(
            ConductorClient::new_with_url(
                format!("http://{}", server.host_with_port()).as_str(),
                None
            )
        );
        let poll_interval = 1000;
        let worker_manager = WorkerManager::new(
            task_provider,
            vec![Arc::new(mock_worker)],
            poll_interval
        );

        worker_manager.poll_and_work(worker_manager.workers[0].deref()).await.unwrap();

        task_input_response.assert();
        task_output_response.assert();
    }

    #[tokio::test]
    async fn test_worker_manager_start_in_thread() {
        let mut server = mockito::Server::new_async().await;

        let task_input =
            r#"{
            "taskDefName": "simple_task_4",
            "inputData": {
                "input_1": "value_1"
            },
            "taskId": "task_id_4"
        }"#;

        let task_input_url = "/api/tasks/poll/simple_task_4";
        let task_input_response = server
            .mock("GET", task_input_url)
            .with_status(200)
            .with_body(task_input)
            .create();

        let task_output =
            r#"{
            "taskDefName": "simple_task_4",
            "status": "COMPLETED",
            "outputData": {
                "output_1": "value_1"
            },
            "taskId": "task_id_4"
        }"#;

        let task_output_url = "/api/tasks";
        let task_output_response = server
            .mock("POST", task_output_url)
            .with_status(200)
            .with_body(task_output)
            .create();

        let mock_worker = MockWorker::new("simple_task_4");

        let task_provider = Box::new(
            ConductorClient::new_with_url(
                format!("http://{}", server.host_with_port()).as_str(),
                None
            )
        );
        let poll_interval = 1000;
        let worker_manager = WorkerManager::new(
            task_provider,
            vec![Arc::new(mock_worker)],
            poll_interval
        );

        let (stop_tx, stop_rx) = tokio::sync::watch::channel(());
        let worker_manager_clone = Arc::new(worker_manager);
        worker_manager_clone.start_in_thread(stop_rx).unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        task_input_response.assert();
        task_output_response.assert();
        stop_tx.send(()).unwrap();
    }
}
#[cfg(test)]
mod integration_tests {
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
