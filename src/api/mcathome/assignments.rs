use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RetrieveTaskOfProjectsRequest {
    #[serde(rename = "acceptedProjectsIDs")]
    pub project_ids: Vec<i64>,

    #[serde(rename = "taskCount")]
    pub task_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveTaskOfProjectsResponse {
    pub assignments: Vec<AssignmentInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AssignmentInfo {
    pub id: i64,
    pub task: TaskInfo,
}

#[derive(Debug, Deserialize)]
pub struct TaskInfo {
    pub id: i64,
    #[serde(rename = "groupID")]
    pub group_id: i64,
    #[serde(rename = "projectID")]
    pub project_id: i64,
    #[serde(rename = "inputData")]
    pub input_data: String,
}