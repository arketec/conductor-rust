use async_trait::async_trait;
use reqwest::Error;

use crate::common::task::task_metadata::TaskDef;

#[async_trait]
pub trait TaskMetadataProvider {
    async fn get_task_metadata(&self, name: &str) -> Result<Option<TaskDef>, Error>;
    async fn get_all_task_metadata(&self) -> Result<Vec<TaskDef>, Error>;
}
