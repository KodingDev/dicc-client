use crate::{
    api::mcathome::platforms::PlatformListResponse,
    data::download::{Checksum, Download},
    manager::platform::Platform,
};
use reqwest::Error;

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
            let download = Download::new(
                platform.detector_binary.download_url.as_str(),
                vec![Checksum::new(
                    "sha256",
                    platform.detector_binary.checksum.as_str(),
                )],
            );


            let platform = Platform::new(
                format!("{}", platform.id).as_str(),
                platform.name.as_str(),
                download,
            );

            platforms.push(platform);
        }
        Ok(platforms)
    }
}