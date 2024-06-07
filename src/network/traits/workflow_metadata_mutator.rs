use async_trait::async_trait;
use reqwest::Error;

use crate::common::workflow::workflow::WorkflowDef;

#[async_trait]
pub trait WorkflowMetadataMutator {
    async fn update_workflow_metadata(&self, workflow: WorkflowDef) -> Result<bool, Error>;
    async fn save_workflow_metadata(&self, workflows: Vec<WorkflowDef>) -> Result<bool, Error>;
}
