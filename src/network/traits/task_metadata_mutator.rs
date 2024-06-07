use async_trait::async_trait;
use reqwest::Error;

use crate::common::task::task_metadata::TaskDef;
#[async_trait]
pub trait TaskMetadataMutator {
    async fn update_task_metadata(&self, task: TaskDef) -> Result<bool, Error>;
    async fn save_task_metadata(&self, tasks: Vec<TaskDef>) -> Result<bool, Error>;
}
