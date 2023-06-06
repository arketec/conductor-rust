use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ConductorWorker {
    async fn execute(
        &self,
        task_input: &Option<HashMap<String, Option<Value>>>
    ) -> Result<Option<HashMap<String, Option<Value>>>, String>;
    fn get_task_type(&self) -> String;
}
