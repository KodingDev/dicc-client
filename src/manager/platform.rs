use simplelog::info;
use std::{fs::Permissions, path::Path};
use tokio::{
    fs::{self},
    process::Command,
};

use crate::data::download::Download;

#[derive(Debug)]
pub struct Platform {
    id: String,
    name: String,
    detector: Download,
}

impl Platform {
    pub fn new(id: &str, name: &str, detector: Download) -> Platform {
        Platform {
            id: id.to_string(),
            name: name.to_string(),
            detector,
        }
    }

    pub async fn detect(&self, path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        let status = Command::new(&path).output().await?.status.code().unwrap();
        Ok(status == 0)
    }
}

pub struct PlatformManager {
    platforms: Vec<Platform>,
}

impl PlatformManager {
    pub fn new() -> PlatformManager {
        PlatformManager {
            platforms: Vec::new(),
        }
    }

    pub fn add(&mut self, platform: Platform) {
        self.platforms.push(platform);
    }

    pub async fn detect(&self) -> Vec<&Platform> {
        let dir = Path::new("platforms");
        if !dir.exists() {
            fs::create_dir_all(dir)
                .await
                .expect("failed to create platforms directory");
        }

        let mut platforms: Vec<&Platform> = Vec::new();
        for platform in &self.platforms {
            let path = dir.join(format!("{}.bin", platform.id));

            platform
                .detector
                .download_to_file(&path.as_path())
                .await
                .expect("failed to download platform");

            #[cfg(not(target_os = "windows"))]
            set_permissions(&path).await;

            if platform.detect(&path).await.unwrap() {
                platforms.push(platform);
                info!("{}: {}", platform.name, "<green>OK</>");
            } else {
                info!("{}: {}", platform.name, "<red>FAILED</>");
            }
        }
        platforms
    }
}

#[cfg(not(target_os = "windows"))]
pub async fn set_permissions(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let perms = Permissions::from_mode(0o755);
    fs::set_permissions(path, perms).await.expect("failed to set permissions");
}