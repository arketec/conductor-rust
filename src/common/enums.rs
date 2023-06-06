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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeoutPolicyEnum {
    #[serde(rename = "RETRY")]
    Retry,
    #[serde(rename = "TIME_OUT_WF")]
    TimeoutWorkflow,
    #[serde(rename = "ALERT_ONLY")]
    AlertOnly,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RetryLogicEnum {
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "EXPONENTIAL_BACK_OFF")]
    ExponentialBackOff,
    #[serde(rename = "LINEAR_BACK_OFF")]
    LinearBackOff,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvaluatorType {
    #[serde(rename = "case-value")]
    CaseValue,
    #[serde(rename = "javascript")]
    Javascript,
}
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
