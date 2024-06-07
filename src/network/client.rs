use async_trait::async_trait;
use reqwest::{ Client, Error, Response };
use serde::Serialize;

use crate::common::{
    event::event_handler::EventHandler,
    task::{ task::Task, task_metadata::TaskDef },
    workflow::{ workflow::WorkflowDef, workflow_metadata::Workflow },
};

use super::traits::{
    event_handler_mutator::EventHandlerMutator,
    event_handler_provider::EventHandlerProvider,
    task_metadata_mutator::TaskMetadataMutator,
    task_metadata_provider::TaskMetadataProvider,
    task_poller::TaskPoller,
    workflow_metadata_mutator::WorkflowMetadataMutator,
    workflow_metadata_provider::WorkflowMetadataProvider,
    workflow_provider::WorkflowProvider,
    workflow_executor::WorkflowExecutor,
};

pub struct ConductorClient {
    client: Client,
    base_url: String,
}

impl ConductorClient {
    pub fn new(conductor_host: &str, conductor_port: &u32, client: Option<Client>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: format!("http://{}:{}/api", conductor_host, conductor_port),
        }
    }

    pub fn new_with_url(conductor_url: &str, client: Option<Client>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: format!("{}/api", conductor_url),
        }
    }
}

#[async_trait]
impl TaskMetadataProvider for ConductorClient {
    async fn get_task_metadata(&self, task_type: &str) -> Result<Option<TaskDef>, Error> {
        let url = format!("{}/metadata/taskdefs/{}", self.base_url, task_type);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(None);
        }
        let task_metadata = response.json::<TaskDef>().await?;
        Ok(Some(task_metadata))
    }
    async fn get_all_task_metadata(&self) -> Result<Vec<TaskDef>, Error> {
        let url = format!("{}/metadata/taskdefs", self.base_url);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(vec![]);
        }
        let task_metadata = response.json::<Vec<TaskDef>>().await?;
        Ok(task_metadata)
    }
}

#[async_trait]
impl TaskMetadataMutator for ConductorClient {
    async fn update_task_metadata(&self, task: TaskDef) -> Result<bool, Error> {
        let url = format!("{}/metadata/taskdefs", self.base_url);
        let response: Response = self.client.put(&url).json(&task).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }
    async fn save_task_metadata(&self, tasks: Vec<TaskDef>) -> Result<bool, Error> {
        let url = format!("{}/metadata/taskdefs", self.base_url);
        let response: Response = self.client.post(&url).json(&tasks).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }
}

#[async_trait]
impl WorkflowMetadataProvider for ConductorClient {
    async fn get_workflow_metadata(
        &self,
        workflow_name: &str
    ) -> Result<Option<WorkflowDef>, Error> {
        let url = format!("{}/metadata/workflow/{}", self.base_url, workflow_name);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(None);
        }
        let workflow_metadata = response.json::<WorkflowDef>().await?;
        Ok(Some(workflow_metadata))
    }
    async fn get_all_workflow_metadata(&self) -> Result<Vec<WorkflowDef>, Error> {
        let url = format!("{}/metadata/workflow", self.base_url);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(vec![]);
        }
        let workflow_metadata = response.json::<Vec<WorkflowDef>>().await?;
        Ok(workflow_metadata)
    }
}

#[async_trait]
impl WorkflowMetadataMutator for ConductorClient {
    async fn update_workflow_metadata(&self, workflow: WorkflowDef) -> Result<bool, Error> {
        let url = format!("{}/metadata/workflow", self.base_url);
        let response: Response = self.client.put(&url).json(&vec![workflow]).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }

        Ok(true)
    }
    async fn save_workflow_metadata(&self, workflows: Vec<WorkflowDef>) -> Result<bool, Error> {
        let url = format!("{}/metadata/workflow", self.base_url);
        let response: Response = self.client.put(&url).json(&workflows).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }
}

#[async_trait]
impl TaskPoller for ConductorClient {
    async fn poll(&self, task_type: &str) -> Result<Option<Task>, Error> {
        println!("Polling for taskType: '{}'", task_type);
        let url = format!("{}/tasks/poll/{}", self.base_url, task_type);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            println!("taskType: '{}' returned no results", task_type);
            return Ok(None);
        }
        if response.status().as_u16() == 204 {
            println!("taskType: '{}' returned no results", task_type);
            return Ok(None);
        }
        let task = response.json::<Task>().await;
        if task.is_err() {
            println!("Error: {}", task.err().unwrap());
            return Ok(None);
        }
        let task = task.ok().unwrap();
        println!("Found task id: {}", task.task_id.as_ref().unwrap());
        Ok(Some(task))
    }
    async fn update(&self, task: Task) -> Result<bool, Error> {
        let url = format!("{}/tasks", self.base_url);
        let response: Response = self.client.post(&url).json(&task).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }
}

#[async_trait]
impl WorkflowProvider for ConductorClient {
    async fn get_workflow_execution(&self, workflow_id: &str) -> Result<Option<Workflow>, Error> {
        let url = format!("{}/workflow/{}", self.base_url, workflow_id);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("workflow_id: '{}' returned no results", workflow_id);
            return Ok(None);
        }
        let workflow = response.json::<Workflow>().await?;
        Ok(Some(workflow))
    }
}

#[async_trait]
impl WorkflowExecutor for ConductorClient {
    async fn start_workflow<T>(
        &self,
        workflow_name: &str,
        workflow_input: T
    ) -> Result<Option<String>, Error>
        where T: Serialize + Send + Sync
    {
        let url = format!("{}/workflow/{}", self.base_url, workflow_name);
        let response: Response = self.client.post(&url).json(&workflow_input).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(None);
        }
        let workflow_id = response.text().await?;
        Ok(Some(workflow_id))
    }
}

#[async_trait]
impl EventHandlerProvider for ConductorClient {
    async fn get_event_handler(&self, event_type: &str) -> Result<Option<EventHandler>, Error> {
        let url = format!("{}/event/{}", self.base_url, event_type);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(None);
        }
        let event_handler = response.json::<EventHandler>().await?;
        Ok(Some(event_handler))
    }

    async fn get_all_event_handlers(&self) -> Result<Vec<EventHandler>, Error> {
        let url = format!("{}/event", self.base_url);
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(vec![]);
        }
        let event_handlers = response.json::<Vec<EventHandler>>().await?;
        Ok(event_handlers)
    }
}

#[async_trait]
impl EventHandlerMutator for ConductorClient {
    async fn update_event_handler(&self, event_handler: EventHandler) -> Result<bool, Error> {
        let url = format!("{}/event", self.base_url);
        let response: Response = self.client.put(&url).json(&event_handler).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }

    async fn save_event_handler(&self, event_handler: EventHandler) -> Result<bool, Error> {
        let url = format!("{}/event", self.base_url);
        let response: Response = self.client.post(&url).json(&event_handler).send().await?;
        if !response.status().is_success() {
            println!("Status: {}", response.status());
            println!("Error: {}", response.text().await?);
            return Ok(false);
        }
        Ok(true)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_conductor_client_new() {
        let host = Box::new("localhost");
        let port = Box::new(8080_u32);
        let client = ConductorClient::new(&host, &port, None);
        assert_eq!(client.base_url, "http://localhost:8080/api");
    }
    #[test]
    fn test_conductor_client_new_with_existing_http_client() {
        let host = Box::new("localhost");
        let port = Box::new(8080_u32);
        let client = ConductorClient::new(&host, &port, Some(Client::new()));
        assert_eq!(client.base_url, "http://localhost:8080/api");
    }
    #[test]
    fn test_conductor_client_new_with_url() {
        let client = ConductorClient::new_with_url("http://localhost:8080", None);
        assert_eq!(client.base_url, "http://localhost:8080/api");
    }
    #[test]
    fn test_conductor_client_new_with_url_with_existing_http_client() {
        let client = ConductorClient::new_with_url("http://localhost:8080", Some(Client::new()));
        assert_eq!(client.base_url, "http://localhost:8080/api");
    }

    #[tokio::test]
    async fn test_get_task_metadata() {
        let mut server = mockito::Server::new_async().await;
        let task_metadata =
            r#"{
            "name": "simple_task_1",
            "retryCount": 3
        }"#;

        server
            .mock("GET", "/api/metadata/taskdefs/simple_task_1")
            .with_status(200)
            .with_body(task_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = client.get_task_metadata("simple_task_1").await.unwrap();
        assert!(task_metadata.is_some());
        let task_metadata = task_metadata.unwrap();
        assert_eq!(task_metadata.name, "simple_task_1");
        assert_eq!(task_metadata.retry_count.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_get_all_task_metadata() {
        let mut server = mockito::Server::new_async().await;
        let task_metadata =
            r#"[{
            "name": "simple_task_1",
            "retryCount": 3
        }]"#;

        server
            .mock("GET", "/api/metadata/taskdefs")
            .with_status(200)
            .with_body(task_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = client.get_all_task_metadata().await.unwrap();
        assert_eq!(task_metadata.len(), 1);
        assert_eq!(task_metadata[0].name, "simple_task_1");
        assert_eq!(task_metadata[0].retry_count.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_get_workflow_metadata() {
        let mut server = mockito::Server::new_async().await;
        let workflow_metadata =
            r#"{
            "name": "load_test",
            "timeoutSeconds": 300
        }"#;

        server
            .mock("GET", "/api/metadata/workflow/load_test")
            .with_status(200)
            .with_body(workflow_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = client.get_workflow_metadata("load_test").await.unwrap();
        assert!(workflow_metadata.is_some());
        let workflow_metadata = workflow_metadata.unwrap();
        assert_eq!(workflow_metadata.name, "load_test");
        assert_eq!(workflow_metadata.timeout_seconds.unwrap(), 300);
    }

    #[tokio::test]
    async fn test_get_all_workflow_metadata() {
        let mut server = mockito::Server::new_async().await;
        let workflow_metadata =
            r#"[{
            "name": "load_test",
            "timeoutSeconds": 300
        }]"#;

        server
            .mock("GET", "/api/metadata/workflow")
            .with_status(200)
            .with_body(workflow_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = client.get_all_workflow_metadata().await.unwrap();
        assert_eq!(workflow_metadata.len(), 1);
        assert_eq!(workflow_metadata[0].name, "load_test");
        assert_eq!(workflow_metadata[0].timeout_seconds.unwrap(), 300);
    }

    #[tokio::test]
    async fn test_poll() {
        let mut server = mockito::Server::new_async().await;
        let task =
            r#"{
            "taskType": "simple_task_1",
            "taskId": "1234"
        }"#;

        server
            .mock("GET", "/api/tasks/poll/simple_task_1")
            .with_status(200)
            .with_body(task)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = client.poll("simple_task_1").await.unwrap();
        assert!(task.is_some());
        let task = task.unwrap();
        assert_eq!(task.task_type.unwrap(), "simple_task_1");
        assert_eq!(task.task_id.unwrap(), "1234");
    }

    #[tokio::test]
    async fn test_update() {
        let mut server = mockito::Server::new_async().await;
        let task =
            r#"{
            "taskType": "simple_task_1",
            "taskId": "1234"
        }"#;

        server.mock("POST", "/api/tasks").with_status(200).with_body(task).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = Task {
            task_type: Some("simple_task_1".to_string()),
            task_id: Some("1234".to_string()),
            ..Default::default()
        };
        let result = client.update(task).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_get_workflow_execution() {
        let mut server = mockito::Server::new_async().await;
        let workflow =
            r#"{
            "workflowName": "load_test",
            "workflowId": "1234"
        }"#;

        server.mock("GET", "/api/workflow/1234").with_status(200).with_body(workflow).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow = client.get_workflow_execution("1234").await.unwrap();
        assert!(workflow.is_some());
        let workflow = workflow.unwrap();
        assert_eq!(workflow.workflow_name.unwrap(), "load_test");
        assert_eq!(workflow.workflow_id.unwrap(), "1234");
    }

    #[tokio::test]
    async fn test_start_workflow() {
        let mut server = mockito::Server::new_async().await;
        let workflow_id = "1234";

        server
            .mock("POST", "/api/workflow/load_test")
            .with_status(200)
            .with_body(workflow_id)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_id = client.start_workflow("load_test", "").await.unwrap();
        assert!(workflow_id.is_some());
        let workflow_id = workflow_id.unwrap();
        assert_eq!(workflow_id, "1234");
    }

    #[tokio::test]
    async fn test_update_task_metadata() {
        let mut server = mockito::Server::new_async().await;
        let task_metadata =
            r#"{
            "name": "simple_task_1",
            "retryCount": 3
        }"#;

        server
            .mock("PUT", "/api/metadata/taskdefs")
            .with_status(200)
            .with_body(task_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        };
        let result = client.update_task_metadata(task_metadata).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_save_task_metadata() {
        let mut server = mockito::Server::new_async().await;
        let task_metadata =
            r#"[{
            "name": "simple_task_1",
            "retryCount": 3
        }]"#;

        server
            .mock("POST", "/api/metadata/taskdefs")
            .with_status(200)
            .with_body(task_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = vec![TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        }];
        let result = client.save_task_metadata(task_metadata).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_update_workflow_metadata() {
        let mut server = mockito::Server::new_async().await;
        let workflow_metadata =
            r#"{
            "name": "load_test",
            "timeoutSeconds": 300
        }"#;

        server
            .mock("PUT", "/api/metadata/workflow")
            .with_status(200)
            .with_body(workflow_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        };
        let result = client.update_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_save_workflow_metadata() {
        let mut server = mockito::Server::new_async().await;
        let workflow_metadata =
            r#"[{
            "name": "load_test",
            "timeoutSeconds": 300
        }]"#;

        server
            .mock("PUT", "/api/metadata/workflow")
            .with_status(200)
            .with_body(workflow_metadata)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = vec![WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        }];
        let result = client.save_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_poll_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/tasks/poll/simple_task_1").with_status(204).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = client.poll("simple_task_1").await.unwrap();
        assert!(task.is_none());
    }

    #[tokio::test]
    async fn test_update_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/tasks").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = Task {
            task_type: Some("simple_task_1".to_string()),
            task_id: Some("1234".to_string()),
            ..Default::default()
        };
        let result = client.update(task).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_get_workflow_execution_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/workflow/1234").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow = client.get_workflow_execution("1234").await.unwrap();
        assert!(workflow.is_none());
    }

    #[tokio::test]
    async fn test_start_workflow_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/workflow/load_test").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_id = client.start_workflow("load_test", "").await.unwrap();
        assert!(workflow_id.is_none());
    }

    #[tokio::test]
    async fn test_get_all_task_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/metadata/taskdefs").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = client.get_all_task_metadata().await.unwrap();
        assert!(task_metadata.is_empty());
    }

    #[tokio::test]
    async fn test_get_task_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/metadata/taskdefs/simple_task_1").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = client.get_task_metadata("simple_task_1").await.unwrap();
        assert!(task_metadata.is_none());
    }

    #[tokio::test]
    async fn test_get_all_workflow_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/metadata/workflow").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = client.get_all_workflow_metadata().await.unwrap();
        assert!(workflow_metadata.is_empty());
    }

    #[tokio::test]
    async fn test_get_workflow_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/metadata/workflow/load_test").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = client.get_workflow_metadata("load_test").await.unwrap();
        assert!(workflow_metadata.is_none());
    }

    #[tokio::test]
    async fn test_update_task_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/taskdefs").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        };
        let result = client.update_task_metadata(task_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_save_task_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/metadata/taskdefs").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = vec![TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        }];
        let result = client.save_task_metadata(task_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_update_workflow_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/workflow").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        };
        let result = client.update_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_save_workflow_metadata_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/workflow").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = vec![WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        }];
        let result = client.save_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_poll_no_results_204() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/tasks/poll/simple_task_1").with_status(204).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = client.poll("simple_task_1").await.unwrap();
        assert!(task.is_none());
    }
    #[tokio::test]
    async fn test_poll_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server
            .mock("GET", "/api/tasks/poll/simple_task_1")
            .with_status(404)
            .with_body("text")
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = client.poll("simple_task_1").await.unwrap();
        assert!(task.is_none());
    }

    #[tokio::test]
    async fn test_update_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/tasks").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task = Task {
            task_type: Some("simple_task_1".to_string()),
            task_id: Some("1234".to_string()),
            ..Default::default()
        };
        let result = client.update(task).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_get_workflow_execution_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/workflow/1234").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow = client.get_workflow_execution("1234").await.unwrap();

        assert!(workflow.is_none());
    }

    #[tokio::test]
    async fn test_start_workflow_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/workflow/load_test").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_id = client.start_workflow("load_test", "").await.unwrap();
        assert!(workflow_id.is_none());
    }

    #[tokio::test]
    async fn test_update_task_metadata_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/taskdefs").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        };
        let result = client.update_task_metadata(task_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_save_task_metadata_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("POST", "/api/metadata/taskdefs").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let task_metadata = vec![TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        }];
        let result = client.save_task_metadata(task_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_update_workflow_metadata_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/workflow").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        };
        let result = client.update_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_save_workflow_metadata_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("PUT", "/api/metadata/workflow").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let workflow_metadata = vec![WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        }];
        let result = client.save_workflow_metadata(workflow_metadata).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_poll_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task = client.poll("simple_task_1").await;
        assert!(task.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_update_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task = Task {
            task_type: Some("simple_task_1".to_string()),
            task_id: Some("1234".to_string()),
            ..Default::default()
        };
        let result = client.update(task).await;
        assert!(result.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_workflow_execution_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow = client.get_workflow_execution("1234").await;
        assert!(workflow.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_start_workflow_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow_id = client.start_workflow("load_test", "").await;
        assert!(workflow_id.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_update_task_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task_metadata = TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        };
        let result = client.update_task_metadata(task_metadata).await;
        assert!(result.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_save_task_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task_metadata = vec![TaskDef {
            name: "simple_task_1".to_string(),
            retry_count: Some(3),
            ..Default::default()
        }];
        let result = client.save_task_metadata(task_metadata).await;
        assert!(result.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_update_workflow_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow_metadata = WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        };
        let result = client.update_workflow_metadata(workflow_metadata).await;
        assert!(result.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_save_workflow_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow_metadata = vec![WorkflowDef {
            name: "load_test".to_string(),
            timeout_seconds: Some(300),
            ..Default::default()
        }];
        let result = client.save_workflow_metadata(workflow_metadata).await;
        assert!(result.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_all_task_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task_metadata = client.get_all_task_metadata().await;
        assert!(task_metadata.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_task_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let task_metadata = client.get_task_metadata("simple_task_1").await;
        assert!(task_metadata.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_all_workflow_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow_metadata = client.get_all_workflow_metadata().await;
        assert!(workflow_metadata.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_workflow_metadata_network_error() {
        let invalid_url = "http://invalid.domain";
        let client = ConductorClient::new_with_url(invalid_url, None);

        let workflow_metadata = client.get_workflow_metadata("load_test").await;
        assert!(workflow_metadata.is_err(), "Expected an error due to invalid domain");
    }

    #[tokio::test]
    async fn test_get_event_handler() {
        let mut server = mockito::Server::new_async().await;
        let event_handler =
            r#"{
            "name": "simple_event_1",
            "event": "test",
            "actions": [
                {
                    "action": "start_workflow",
                    "start_workflow": {
                        "name": "simple_action_1"
                    }
                }
            ],
            "evaluatorType": "javascript",
            "active": true,
            "condition": "true"
        }"#;

        server
            .mock("GET", "/api/event/simple_event_1")
            .with_status(200)
            .with_body(event_handler)
            .create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let event_handlers = client.get_event_handler("simple_event_1").await.unwrap().unwrap();

        assert_eq!(event_handlers.name, "simple_event_1");
        assert_eq!(event_handlers.event, "test");
    }

    #[tokio::test]
    async fn test_get_all_event_handlers() {
        let mut server = mockito::Server::new_async().await;
        let event_handlers =
            r#"[{
                "name": "simple_event_1",
                "event": "test",
                "actions": [
                    {
                        "action": "start_workflow",
                        "start_workflow": {
                            "name": "simple_action_1"
                        }
                    }
                ],
                "evaluatorType": "javascript",
                "active": true,
                "condition": "true"
            }]"#;

        server.mock("GET", "/api/event").with_status(200).with_body(event_handlers).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let event_handlers = client.get_all_event_handlers().await.unwrap();

        assert_eq!(event_handlers.len(), 1);
        assert_eq!(event_handlers[0].name, "simple_event_1");
        assert_eq!(event_handlers[0].event, "test");
    }

    #[tokio::test]
    async fn test_save_event_handler() {
        let mut server = mockito::Server::new_async().await;
        let event_handler =
            r#"{
            "name": "simple_event_1",
            "event": "test"
        }"#;

        server.mock("POST", "/api/event").with_status(200).with_body(event_handler).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let event_handler = EventHandler {
            name: "simple_event_1".to_string(),
            event: "test".to_string(),
            ..Default::default()
        };
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let result = client.save_event_handler(event_handler).await.unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn test_update_event_handler() {
        let mut server = mockito::Server::new_async().await;
        let event_handler =
            r#"{
            "name": "simple_event_1",
            "event": "test"
        }"#;

        server.mock("PUT", "/api/event").with_status(200).with_body(event_handler).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let event_handler = EventHandler {
            name: "simple_event_1".to_string(),
            event: "test".to_string(),
            ..Default::default()
        };
        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let result = client.update_event_handler(event_handler).await.unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn test_get_event_handler_no_results() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/event/simple_event_1").with_status(404).create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let result = client.get_event_handler("simple_event_1").await.unwrap();

        assert!(result.is_none());
    }
    #[tokio::test]
    async fn test_get_event_handler_no_results_404() {
        let mut server = mockito::Server::new_async().await;

        server.mock("GET", "/api/event/simple_event_1").with_status(404).with_body("text").create();

        let url = server.host_with_port();
        let host = url.split(':').collect::<Vec<&str>>()[0];
        let port = url.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let client = ConductorClient::new(&Box::new(host), &Box::new(port), None);

        let result = client.get_event_handler("simple_event_1").await.unwrap();

        assert!(result.is_none());
    }
}

#[cfg(test)]
mod integration_tests {
    #[cfg(all(test, feature = "integration_test"))]
    use crate::{
        network::client::{ ConductorClient, TaskMetadataProvider, WorkflowMetadataProvider },
        common::enums::{ RetryLogicEnum, TimeoutPolicyEnum },
    };
    #[cfg(all(test, feature = "integration_test"))]
    use tokio::runtime::Runtime;

    #[test]
    #[cfg(all(test, feature = "integration_test"))]
    fn test_get_task_metadata() {
        let rt = Runtime::new().unwrap();
        let client = build_test_conductor_client();
        let task_metadata = rt.block_on(client.get_task_metadata("simple_task_1")).unwrap();
        assert!(task_metadata.is_some());
        let task_metadata = task_metadata.unwrap();
        assert_eq!(task_metadata.name, "simple_task_1");
        assert_eq!(task_metadata.retry_logic.unwrap(), RetryLogicEnum::Fixed);
        assert_eq!(task_metadata.timeout_policy.unwrap(), TimeoutPolicyEnum::TimeoutWorkflow);
    }

    #[test]
    #[cfg(all(test, feature = "integration_test"))]
    fn test_get_all_task_metadata() {
        let rt = Runtime::new().unwrap();
        let client = build_test_conductor_client();
        let task_metadata = rt.block_on(client.get_all_task_metadata()).unwrap();
        assert!(task_metadata.len() > 0);
    }

    #[test]
    #[cfg(all(test, feature = "integration_test"))]
    fn test_get_workflow_metadata() {
        let rt = Runtime::new().unwrap();
        let client = build_test_conductor_client();
        let workflow_metadata = rt.block_on(client.get_workflow_metadata("load_test")).unwrap();
        assert!(workflow_metadata.is_some());
        let workflow_metadata = workflow_metadata.unwrap();
        assert_eq!(workflow_metadata.name, "load_test");
        assert_eq!(workflow_metadata.timeout_policy.unwrap(), TimeoutPolicyEnum::AlertOnly);
    }

    #[test]
    #[cfg(all(test, feature = "integration_test"))]
    fn test_get_all_workflow_metadata() {
        let rt = Runtime::new().unwrap();
        let client = build_test_conductor_client();
        let workflow_metadata = rt.block_on(client.get_all_workflow_metadata()).unwrap();
        assert!(workflow_metadata.len() > 0);
    }

    #[cfg(all(test, feature = "integration_test"))]
    fn build_test_conductor_client() -> ConductorClient {
        let host = Box::new("localhost");
        let port = Box::new(8080_u32);
        ConductorClient::new(&host, &port, None)
    }
}
