use async_trait::async_trait;
use reqwest::Error;

use crate::common::task::task::Task;
#[async_trait]
pub trait TaskPoller {
    async fn poll(&self, task_type: &str) -> Result<Option<Task>, Error>;
    async fn update(&self, task: Task) -> Result<bool, Error>;
}
