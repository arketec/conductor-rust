use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use crate::common::task::task_metadata::TaskDef;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowTaskTypeEnum {
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "DYNAMIC")]
    Dynamic,
    #[serde(rename = "FORK_JOIN")]
    ForkJoin,
    #[serde(rename = "FORK_JOIN_DYNAMIC")]
    ForkJoinDynamic,
    #[serde(rename = "DECISION")]
    Decision,
    #[serde(rename = "SWITCH")]
    Switch,
    #[serde(rename = "JOIN")]
    Join,
    #[serde(rename = "DO_WHILE")]
    DoWhile,
    #[serde(rename = "SUB_WORKFLOW")]
    SubWorkflow,
    #[serde(rename = "START_WORKFLOW")]
    StartWorkflow,
    #[serde(rename = "EVENT")]
    Event,
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "HUMAN")]
    Human,
    #[serde(rename = "USER_DEFINED")]
    UserDefined,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "LAMBDA")]
    Lambda,
    #[serde(rename = "INLINE")]
    Inline,
    #[serde(rename = "EXCLUSIVE_JOIN")]
    ExclusiveJoin,
    #[serde(rename = "TERMINATE")]
    Terminate,
    #[serde(rename = "KAFKA_PUBLISH")]
    KafkaPublish,
    #[serde(rename = "JSON_JQ_TRANSFORM")]
    JsonJqTransform,
    #[serde(rename = "SET_VARIABLE")]
    SetVariable,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowTask {
    #[serde(rename = "type")]
    pub workflow_task_type: Option<WorkflowTaskTypeEnum>,
    #[serde(rename = "asyncComplete")]
    pub async_complete: Option<bool>,
    #[serde(rename = "caseExpression")]
    pub case_expression: Option<String>,
    #[serde(rename = "caseValueParam")]
    pub case_value_param: Option<String>,
    #[serde(rename = "decisionCases")]
    pub decision_cases: Option<HashMap<String, Option<Vec<Option<Value>>>>>,
    #[serde(rename = "defaultCase")]
    pub default_case: Option<Vec<Option<Value>>>,
    #[serde(rename = "defaultExclusiveJoinTask")]
    pub default_exclusive_join_task: Option<Vec<Option<String>>>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "dynamicForkJoinTasksParam")]
    pub dynamic_fork_join_tasks_param: Option<String>,
    #[serde(rename = "dynamicForkTasksInputParamName")]
    pub dynamic_fork_tasks_input_param_name: Option<String>,
    #[serde(rename = "dynamicForkTasksParam")]
    pub dynamic_fork_tasks_param: Option<String>,
    #[serde(rename = "dynamicTaskNameParam")]
    pub dynamic_task_name_param: Option<String>,
    #[serde(rename = "evaluatorType")]
    pub evaluator_type: Option<String>,
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    #[serde(rename = "forkTasks")]
    pub fork_tasks: Option<Vec<Vec<Option<Value>>>>,
    #[serde(rename = "inputParameters")]
    pub input_parameters: HashMap<String, Option<Value>>,
    #[serde(rename = "joinOn")]
    pub join_on: Option<Vec<Option<String>>>,
    #[serde(rename = "loopCondition")]
    pub loop_condition: Option<String>,
    #[serde(rename = "loopOver")]
    pub loop_over: Option<Vec<Option<Value>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "optional")]
    pub optional: Option<bool>,
    #[serde(rename = "rateLimited")]
    pub rate_limited: Option<bool>,
    #[serde(rename = "retryCount")]
    pub retry_count: Option<i32>,
    #[serde(rename = "scriptExpression")]
    pub script_expression: Option<String>,
    #[serde(rename = "sink")]
    pub sink: Option<String>,
    #[serde(rename = "startDelay")]
    pub start_delay: Option<i32>,
    #[serde(rename = "subWorkflowParam")]
    pub sub_workflow_param: Option<Value>,
    #[serde(rename = "taskDefinition")]
    pub task_definition: Option<TaskDef>,
    #[serde(rename = "taskReferenceName")]
    pub task_reference_name: String,
}
