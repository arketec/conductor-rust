use async_trait::async_trait;
use reqwest::{ Client, Error, Response };
use serde::Serialize;

use crate::common::{
    task::{ task_metadata::TaskDef, task::Task },
    workflow::{ workflow::WorkflowDef, workflow_metadata::Workflow },
};

#[async_trait]
pub trait TaskMetadataProvider {
    async fn get_task_metadata(&self, task_id: &str) -> Result<Option<TaskDef>, Error>;
    async fn get_all_task_metadata(&self) -> Result<Vec<TaskDef>, Error>;
}

#[async_trait]
pub trait TaskMetadataMutator {
    async fn update_task_metadata(&self, task: TaskDef) -> Result<bool, Error>;
    async fn save_task_metadata(&self, tasks: Vec<TaskDef>) -> Result<bool, Error>;
}

#[async_trait]
pub trait WorkflowMetadataProvider {
    async fn get_workflow_metadata(&self, task_id: &str) -> Result<Option<WorkflowDef>, Error>;
    async fn get_all_workflow_metadata(&self) -> Result<Vec<WorkflowDef>, Error>;
}

#[async_trait]
pub trait WorkflowMetadataMutator {
    async fn update_workflow_metadata(&self, task: WorkflowDef) -> Result<bool, Error>;
    async fn save_workflow_metadata(&self, tasks: Vec<WorkflowDef>) -> Result<bool, Error>;
}

#[async_trait]
pub trait TaskPoller {
    async fn poll(&self, task_type: &str) -> Result<Option<Task>, Error>;
    async fn update(&self, task: Task) -> Result<bool, Error>;
}

#[async_trait]
pub trait WorkflowProvider {
    async fn get_workflow_execution(&self, workflow_id: &str) -> Result<Option<Workflow>, Error>;
}

#[async_trait]
pub trait WorkflowExecutor {
    async fn start_workflow<T>(
        &self,
        workflow_name: &str,
        workflow_input: T
    ) -> Result<Option<String>, Error>
        where T: Serialize + Send + Sync;
}

pub struct ConductorClient {
    client: Client,
    base_url: String,
}

impl ConductorClient {
    pub fn new(
        conductor_host: &Box<&str>,
        conductor_port: &Box<u32>,
        client: Option<Client>
    ) -> Self {
        Self {
            client: client.unwrap_or(Client::new()),
            base_url: format!("http://{}:{}/api", conductor_host, conductor_port),
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
        let response: Response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            println!("taskType: '{}' returned no results", task_type);
            return Ok(None);
        }
        if response.status().as_u16() == 204 {
            println!("taskType: '{}' returned no results", task_type);
            return Ok(None);
        }
        let task = response.json::<Task>().await?;
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

#[cfg(test)]
mod unit_tests {
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
