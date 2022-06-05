use std::{collections::HashMap, path::Path};
#[cfg(not(target_os = "windows"))]
use std::fs::Permissions;

use simplelog::info;
use tokio::{
    fs::{self}
};

use crate::data::download::Download;

#[derive(Debug, Clone)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    pub detector: Download,
}

impl Platform {
    pub fn new(id: i64, name: &str, detector: Download) -> Platform {
        Platform {
            id,
            name: name.to_string(),
            detector,
        }
    }

    pub async fn detect(&self, path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.detector.get_command(path).output().await;
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

    pub async fn detect(&self) -> HashMap<i64, Platform> {
        let dir = Path::new("platforms");
        if !dir.exists() {
            fs::create_dir_all(dir)
                .await
                .expect("failed to create platforms directory");
        }

        let mut platforms: HashMap<i64, Platform> = HashMap::new();
        for platform in &self.platforms {
            let path = dir.join(format!("{}.bin", platform.id));

            platform
                .detector
                .download_to_file(&path.as_path())
                .await
                .expect("failed to download platform");

            set_permissions(&path).await;
            if platform.detect(&path).await.unwrap() {
                platforms.insert(platform.id, platform.to_owned());
                info!("{}: {}", platform.name, "<green>OK</>");
            } else {
                info!("{}: {}", platform.name, "<red>FAILED</>");
            }
        }
        platforms
    }
}

#[allow(unused_variables)]
pub async fn set_permissions(path: &Path) {
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = Permissions::from_mode(0o755);
        fs::set_permissions(path, perms).await.expect("failed to set permissions");
    }
}