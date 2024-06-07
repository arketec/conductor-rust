use async_trait::async_trait;
use reqwest::Error;

use crate::common::workflow::workflow::WorkflowDef;
#[async_trait]
pub trait WorkflowMetadataProvider {
    async fn get_workflow_metadata(&self, name: &str) -> Result<Option<WorkflowDef>, Error>;
    async fn get_all_workflow_metadata(&self) -> Result<Vec<WorkflowDef>, Error>;
}
