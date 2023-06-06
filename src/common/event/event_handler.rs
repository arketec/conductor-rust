use serde::{ Deserialize, Serialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvaluatorType {
    #[serde(rename = "case-value")]
    CaseValue,
    #[serde(rename = "javascript")]
    Javascript,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "complete_task")]
    CompleteTask,
    #[serde(rename = "fail_task")]
    FailTask,
    #[serde(rename = "start_workflow")]
    StartWorkflow,
}

#[derive(Serialize, Deserialize)]
pub struct StartWorkflow {
    pub name: String,
    pub version: Option<i32>,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
    pub input: Option<serde_json::value::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTask {
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    #[serde(rename = "taskRefName")]
    pub task_ref_name: String,
    #[serde(rename = "Output")]
    pub output: Option<serde_json::value::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub action: ActionType,
    pub start_workflow: Option<StartWorkflow>,
    pub complete_task: Option<UpdateTask>,
    pub fail_task: Option<UpdateTask>,
}

#[derive(Serialize, Deserialize)]
pub struct EventHandler {
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "active")]
    pub active: Option<bool>,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "evaluatorType")]
    pub evaluator_type: EvaluatorType,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "name")]
    pub name: String,
}
