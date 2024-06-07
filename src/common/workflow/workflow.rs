use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use crate::common::enums::TimeoutPolicyEnum;

use super::workflow_task::WorkflowTask;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WorkflowDef {
    #[serde(rename = "timeoutPolicy")]
    pub timeout_policy: Option<TimeoutPolicyEnum>,
    #[serde(rename = "createTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "failureWorkflow")]
    pub failure_workflow: Option<String>,
    #[serde(rename = "inputParameters")]
    pub input_parameters: Option<Vec<Option<String>>>,
    #[serde(rename = "inputTemplate")]
    pub input_template: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outputParameters")]
    pub output_parameters: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "ownerApp")]
    pub owner_app: Option<String>,
    #[serde(rename = "ownerEmail")]
    pub owner_email: Option<String>,
    #[serde(rename = "restartable")]
    pub restartable: Option<bool>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: Option<i32>,
    #[serde(rename = "tasks")]
    pub tasks: Option<Vec<WorkflowTask>>,
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,
    #[serde(rename = "variables")]
    pub variables: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(rename = "workflowStatusListenerEnabled")]
    pub workflow_status_listener_enabled: Option<bool>,
}
