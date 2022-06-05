use serde::{Deserialize, Serialize};

use crate::api::mcathome::platforms::BinaryInfo;

#[derive(Debug, Serialize)]
pub struct GetProjectsForPlatformsRequest {
    #[serde(rename = "PlatformsIDs")]
    pub platform_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GetProjectsForPlatformsResponse {
    #[serde(rename = "projectsIDs")]
    pub project_ids: Vec<i32>,

    #[serde(rename = "projectsBinaries")]
    pub project_binaries: Vec<ProjectBinary>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectBinary {
    pub id: i64,
    pub priority: i32,

    #[serde(rename = "platformID")]
    pub platform_id: i64,

    pub binary: BinaryInfo,
    pub project: ProjectInfo,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub id: i64,
    pub name: String,
}