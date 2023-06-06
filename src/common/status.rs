use serde::{ Deserialize, Serialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "CANCELED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "FAILED_WITH_TERMINAL_ERROR")]
    FailedWithTerminalError,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "COMPLETED_WITH_ERRORS")]
    CompletedWithErrors,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "TIMED_OUT")]
    TimedOut,
    #[serde(rename = "SKIPPED")]
    Skipped,
}
