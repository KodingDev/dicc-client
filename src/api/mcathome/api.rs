use std::collections::hash_map::Entry;
use std::collections::HashMap;

use reqwest::Error;

use crate::api::mcathome::projects::{
    GetProjectsForPlatformsRequest, GetProjectsForPlatformsResponse,
};
use crate::data::project::{Project, ProjectPlatform};
use crate::{
    api::mcathome::platforms::PlatformListResponse,
    manager::platform::Platform,
};

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
                format!("{}", platform.id).as_str(),
                platform.name.as_str(),
                platform.detector_binary.as_download(),
            );

            platforms.push(platform);
        }
        Ok(platforms)
    }

    pub async fn get_projects_for_platforms(
        &self,
        platforms: HashMap<String, Platform>,
    ) -> Result<Vec<Project>, Error> {
        let platform_ids = platforms
            .iter()
            .map(|(_, p)| p.id.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

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

        let mut projects: HashMap<i32, Project> = HashMap::new();
        for binary in response.project_binaries {
            let project = match projects.entry(binary.project.id) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    entry.insert(Project::new(binary.project.id, &binary.project.name))
                }
            };

            let platform = platforms
                .get(&binary.platform_id.to_string())
                .expect("Platform not found");

            project.add_platform(ProjectPlatform::new(
                platform.clone(),
                binary.binary.as_download(),
                binary.priority,
            ));
        }
        Ok(projects.into_iter().map(|(_, v)| v).collect())
    }
}
