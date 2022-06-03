use crate::manager::platform::Platform;

use super::download::Download;

#[derive(Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub platforms: Vec<ProjectPlatform>,
}

#[derive(Debug)]
pub struct ProjectPlatform {
    pub platform: Platform,
    pub binary: Download,
    pub priority: i32
}

impl Project {
    pub fn new(id: i32, name: &str) -> Project {
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