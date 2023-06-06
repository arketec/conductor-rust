use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use crate::common::timeout_policy::TimeoutPolicyEnum;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RetryLogicEnum {
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "EXPONENTIAL_BACK_OFF")]
    ExponentialBackOff,
    #[serde(rename = "LINEAR_BACK_OFF")]
    LinearBackOff,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDef {
    #[serde(rename = "retryLogic")]
    pub retry_logic: Option<RetryLogicEnum>,
    #[serde(rename = "timeoutPolicy")]
    pub timeout_policy: Option<TimeoutPolicyEnum>,
    #[serde(rename = "backoffScaleFactor")]
    pub backoff_scale_factor: Option<i32>,
    #[serde(rename = "concurrentExecLimit")]
    pub concurrent_exec_limit: Option<i32>,
    #[serde(rename = "createTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "executionNameSpace")]
    pub execution_name_space: Option<String>,
    #[serde(rename = "inputKeys")]
    pub input_keys: Option<Vec<Option<String>>>,
    #[serde(rename = "inputTemplate")]
    pub input_template: Option<HashMap<String, Option<Value>>>,
    #[serde(rename = "isolationGroupId")]
    pub isolation_group_id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outputKeys")]
    pub output_keys: Option<Vec<Option<String>>>,
    #[serde(rename = "ownerApp")]
    pub owner_app: Option<String>,
    #[serde(rename = "ownerEmail")]
    pub owner_email: Option<String>,
    #[serde(rename = "pollTimeoutSeconds")]
    pub poll_timeout_seconds: Option<i32>,
    #[serde(rename = "rateLimitFrequencyInSeconds")]
    pub rate_limit_frequency_in_seconds: Option<i32>,
    #[serde(rename = "rateLimitPerFrequency")]
    pub rate_limit_per_frequency: Option<i32>,
    #[serde(rename = "responseTimeoutSeconds")]
    pub response_timeout_seconds: Option<i64>,
    #[serde(rename = "retryCount")]
    pub retry_count: Option<i32>,
    #[serde(rename = "retryDelaySeconds")]
    pub retry_delay_seconds: Option<i32>,
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,
}
