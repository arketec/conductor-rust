use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use crate::common::{ workflow::workflow_task::WorkflowTask, enums::StatusEnum };

use super::task_metadata::TaskDef;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Task {
    #[serde(rename = "status")]
    pub status: Option<StatusEnum>,
    #[serde(rename = "callbackAfterSeconds")]
    pub callback_after_seconds: Option<i64>,
    #[serde(rename = "callbackFromWorker")]
    pub callback_from_worker: Option<bool>,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "executed")]
    pub executed: Option<bool>,
    #[serde(rename = "executionNameSpace")]
    pub execution_name_space: Option<String>,
    #[serde(rename = "externalInputPayloadStoragePath")]
    pub external_input_payload_storage_path: Option<String>,
    #[serde(rename = "externalOutputPayloadStoragePath")]
    pub external_output_payload_storage_path: Option<String>,
    #[serde(rename = "inputData")]
    pub input_data: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "isolationGroupId")]
    pub isolation_group_id: Option<String>,
    #[serde(rename = "iteration")]
    pub iteration: Option<i32>,
    #[serde(rename = "loopOverTask")]
    pub loop_over_task: Option<bool>,
    #[serde(rename = "outputData")]
    pub output_data: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "pollCount")]
    pub poll_count: Option<i32>,
    #[serde(rename = "queueWaitTime")]
    pub queue_wait_time: Option<i64>,
    #[serde(rename = "rateLimitFrequencyInSeconds")]
    pub rate_limit_frequency_in_seconds: Option<i32>,
    #[serde(rename = "rateLimitPerFrequency")]
    pub rate_limit_per_frequency: Option<i32>,
    #[serde(rename = "reasonForIncompletion")]
    pub reason_for_incompletion: Option<String>,
    #[serde(rename = "referenceTaskName")]
    pub reference_task_name: Option<String>,
    #[serde(rename = "responseTimeoutSeconds")]
    pub response_timeout_seconds: Option<i64>,
    #[serde(rename = "retried")]
    pub retried: Option<bool>,
    #[serde(rename = "retriedTaskId")]
    pub retried_task_id: Option<String>,
    #[serde(rename = "retryCount")]
    pub retry_count: Option<i32>,
    #[serde(rename = "scheduledTime")]
    pub scheduled_time: Option<i64>,
    #[serde(rename = "seq")]
    pub seq: Option<i32>,
    #[serde(rename = "startDelayInSeconds")]
    pub start_delay_in_seconds: Option<i32>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "subWorkflowId")]
    pub sub_workflow_id: Option<String>,
    #[serde(rename = "subWorkflowChanged")]
    pub sub_workflow_changed: Option<bool>,
    #[serde(rename = "taskDefName")]
    pub task_def_name: Option<String>,
    #[serde(rename = "taskDefinition")]
    pub task_definition: Option<TaskDef>,
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
    #[serde(rename = "taskType")]
    pub task_type: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    #[serde(rename = "workerId")]
    pub worker_id: Option<String>,
    #[serde(rename = "workflowInstanceId")]
    pub workflow_instance_id: Option<String>,
    #[serde(rename = "workflowPriority")]
    pub workflow_priority: Option<i32>,
    #[serde(rename = "workflowTask")]
    pub workflow_task: Option<WorkflowTask>,
    #[serde(rename = "workflowType")]
    pub workflow_type: Option<String>,
}
