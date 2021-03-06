use std::collections::hash_map::Entry;
use std::collections::HashMap;

use reqwest::Error;

use crate::{
    api::mcathome::platforms::PlatformListResponse,
    manager::platform::Platform,
};
use crate::api::mcathome::assignments::{RetrieveTaskOfProjectsRequest, RetrieveTaskOfProjectsResponse};
use crate::api::mcathome::projects::{
    GetProjectsForPlatformsRequest, GetProjectsForPlatformsResponse,
};
use crate::api::mcathome::results::{SubmitResultRequest, SubmitResultResponse};
use crate::data::assignment::{Assignment, AssignmentResult};
use crate::data::project::{Project, ProjectPlatform};

#[derive(Debug, Clone)]
pub struct MCAtHomeAPI {
    client: reqwest::Client,
    api_key: String,
}

impl MCAtHomeAPI {
    const BASE_URL: &'static str = "https://api.microboinc.com";

    pub fn new(api_key: &str) -> MCAtHomeAPI {
        MCAtHomeAPI {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn list_platforms(&self) -> Result<Vec<Platform>, Error> {
        let url = format!("{}/platforms/list", MCAtHomeAPI::BASE_URL);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", self.api_key.to_string())
            .send()
            .await?
            .json::<PlatformListResponse>()
            .await?;

        let mut platforms: Vec<Platform> = Vec::new();
        for platform in resp {
            let platform = Platform::new(
                platform.id,
                platform.name.as_str(),
                platform.detector_binary.as_download(),
            );

            platforms.push(platform);
        }
        Ok(platforms)
    }

    pub async fn get_projects_for_platforms(
        &self,
        platforms: &HashMap<i64, Platform>,
    ) -> Result<Vec<Project>, Error> {
        let platform_ids = platforms
            .iter()
            .map(|(_, p)| p.id)
            .collect::<Vec<i64>>();

        let url = format!("{}/projects/compatible", MCAtHomeAPI::BASE_URL);
        let body = GetProjectsForPlatformsRequest { platform_ids };

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.api_key.to_string())
            .json(&body)
            .send()
            .await?
            .json::<GetProjectsForPlatformsResponse>()
            .await?;

        let mut projects: HashMap<i64, Project> = HashMap::new();
        for binary in response.project_binaries {
            let project = match projects.entry(binary.project.id) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    entry.insert(Project::new(binary.project.id, &binary.project.name))
                }
            };

            let platform = platforms
                .get(&binary.platform_id)
                .expect("Platform not found");

            project.add_platform(ProjectPlatform::new(
                platform.clone(),
                binary.binary.as_download(),
                binary.priority,
            ));
        }

        let mut projects: Vec<Project> = projects.into_iter().map(|(_, v)| v).collect();
        projects.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(projects)
    }

    pub async fn get_assignments(&self, projects: &Vec<Project>) -> Result<Vec<Assignment>, Error> {
        let project_ids = projects
            .iter()
            .map(|p| p.id)
            .collect::<Vec<i64>>();

        let url = format!("{}/feeder/ofprojects", MCAtHomeAPI::BASE_URL);
        let body = RetrieveTaskOfProjectsRequest { task_count: 1, project_ids };

        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.api_key.to_string())
            .json(&body)
            .send()
            .await?
            .json::<RetrieveTaskOfProjectsResponse>()
            .await?;

        let assignments = resp
            .assignments
            .into_iter()
            .map(|assignment| {
                let project = projects
                    .iter()
                    .find(|p| p.id == assignment.task.project_id)
                    .expect("Project not found");

                Assignment::new(assignment.id, project.to_owned(), assignment.task.input_data)
            })
            .collect::<Vec<Assignment>>();

        Ok(assignments)
    }

    pub async fn submit_result(&self, result: &AssignmentResult) -> Result<SubmitResultResponse, Error> {
        let url = format!("{}/results/submit", MCAtHomeAPI::BASE_URL);
        let body = SubmitResultRequest {
            execution_time: result.execution_time,
            assignment_id: result.id,
            std_err: result.error.to_string(),
            std_out: result.output.to_string(),
            exit_code: result.status as i64,
        };

        Ok(
            self
                .client
                .post(&url)
                .header("Authorization", self.api_key.to_string())
                .json(&body)
                .send()
                .await?
                .json::<SubmitResultResponse>()
                .await?
        )
    }
}
