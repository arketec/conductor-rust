use async_trait::async_trait;
use reqwest::Error;
use serde::Serialize;
#[async_trait]
pub trait WorkflowExecutor {
    async fn start_workflow<T>(
        &self,
        workflow_name: &str,
        workflow_input: T
    ) -> Result<Option<String>, Error>
        where T: Serialize + Send + Sync;
}
