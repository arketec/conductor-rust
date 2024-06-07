use async_trait::async_trait;
use reqwest::Error;

use crate::common::workflow::workflow_metadata::Workflow;
#[async_trait]
pub trait WorkflowProvider {
    async fn get_workflow_execution(&self, workflow_id: &str) -> Result<Option<Workflow>, Error>;
}
