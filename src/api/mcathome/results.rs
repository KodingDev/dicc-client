use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SubmitResultRequest {
    #[serde(rename = "executionTime")]
    pub execution_time: u128,

    #[serde(rename = "assignmentID")]
    pub assignment_id: i64,

    #[serde(rename = "stdErr")]
    pub std_err: String,

    #[serde(rename = "stdOut")]
    pub std_out: String,

    #[serde(rename = "exitCode")]
    pub exit_code: i64,
}

#[derive(Debug, Deserialize)]
pub struct SubmitResultResponse {
    pub id: i64,
}