use serde::{ Deserialize, Serialize };
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeoutPolicyEnum {
    #[serde(rename = "RETRY")]
    Retry,
    #[serde(rename = "TIME_OUT_WF")]
    TimeoutWorkflow,
    #[serde(rename = "ALERT_ONLY")]
    AlertOnly,
}
