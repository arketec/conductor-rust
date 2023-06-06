use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use crate::common::{ enums::StatusEnum, task::task::Task };

use super::workflow::WorkflowDef;

#[derive(Serialize, Deserialize)]
pub struct Workflow {
    #[serde(rename = "status")]
    pub status: Option<StatusEnum>,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "event")]
    pub event: Option<String>,
    #[serde(rename = "externalInputPayloadStoragePath")]
    pub external_input_payload_storage_path: Option<String>,
    #[serde(rename = "externalOutputPayloadStoragePath")]
    pub external_output_payload_storage_path: Option<String>,
    #[serde(rename = "failedReferenceTaskNames")]
    pub failed_reference_task_names: Option<Vec<Option<String>>>,
    #[serde(rename = "input")]
    pub input: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "lastRetriedTime")]
    pub last_retried_time: Option<i64>,
    #[serde(rename = "output")]
    pub output: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "ownerApp")]
    pub owner_app: Option<String>,
    #[serde(rename = "parentWorkflowId")]
    pub parent_workflow_id: Option<String>,
    #[serde(rename = "parentWorkflowTaskId")]
    pub parent_workflow_task_id: Option<String>,
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    #[serde(rename = "reRunFromWorkflowId")]
    pub re_run_from_workflow_id: Option<String>,
    #[serde(rename = "reasonForIncompletion")]
    pub reason_for_incompletion: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "taskToDomain")]
    pub task_to_domain: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "tasks")]
    pub tasks: Option<Vec<Task>>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,
    #[serde(rename = "variables")]
    pub variables: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "workflowDefinition")]
    pub workflow_definition: Option<WorkflowDef>,
    #[serde(rename = "workflowId")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowName")]
    pub workflow_name: Option<String>,
    #[serde(rename = "workflowVersion")]
    pub workflow_version: Option<i32>,
}
