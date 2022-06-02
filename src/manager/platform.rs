#[cfg(not(target_os = "windows"))]
use std::fs::Permissions;
use std::path::Path;

use simplelog::info;
use tokio::{
    fs::{self},
    process::Command,
};

use crate::data::download::Download;

#[derive(Debug)]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub detector: Download,
}

impl Platform {
    pub fn new(id: &str, name: &str, detector: Download) -> Platform {
        Platform {
            id: id.to_string(),
            name: name.to_string(),
            detector,
        }
    }

    fn get_command(&self, path: &Path) -> Command {
        let mut command = Command::new(path);

        if self.detector.get_filename().ends_with(".jar") {
            command = Command::new("java");
            command.arg("-jar");
            command.arg(path);
        }

        command
    }

    pub async fn detect(&self, path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.get_command(path).output().await;
        if result.is_err() {
            return Ok(false);
        }

        Ok(result.unwrap().status.code() == Some(0))
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

pub async fn set_permissions(path: &Path) {
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = Permissions::from_mode(0o755);
        fs::set_permissions(path, perms).await.expect("failed to set permissions");
    }
}