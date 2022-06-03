use crate::manager::platform::Platform;

use super::download::Download;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub platforms: Vec<ProjectPlatform>,
}

#[derive(Debug, Clone)]
pub struct ProjectPlatform {
    pub platform: Platform,
    pub binary: Download,
    pub priority: i32,
}

impl Project {
    pub fn new(id: i64, name: &str) -> Project {
        Project {
            id,
            name: name.to_string(),
            platforms: Vec::new(),
        }
    }

    pub fn add_platform(&mut self, platform: ProjectPlatform) {
        self.platforms.push(platform);
    }
}

impl ProjectPlatform {
    pub fn new(platform: Platform, binary: Download, priority: i32) -> ProjectPlatform {
        ProjectPlatform {
            platform,
            binary,
            priority,
        }
    }
}