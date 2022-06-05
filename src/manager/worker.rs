use std::path::{Path, PathBuf};

use simplelog::{error, info};
use tokio::fs;
use tokio::process::Command;

use crate::data::assignment::{Assignment, AssignmentResult};
use crate::data::project::ProjectPlatform;

pub struct ProjectWorker {
    pub assignment: Assignment,
}

impl ProjectWorker {
    fn get_platform(&self, platforms: &Vec<i64>) -> Result<&ProjectPlatform, Box<dyn std::error::Error>> {
        for platform in platforms {
            if let Some(platform) = self.assignment.project.platforms.get(platform) {
                return Ok(platform);
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "platform not found",
        )))
    }

    pub async fn prepare_binary(&self, platform: &ProjectPlatform) -> Result<Command, Box<dyn std::error::Error>> {
        let dir = Path::new("projects")
            .join(&self.assignment.project.name)
            .join("bin");

        if !&dir.exists() {
            fs::create_dir_all(&dir)
                .await
                .expect("failed to create platforms directory");
        }

        let path = dir.join(platform.binary.get_filename());
        platform.binary.download_to_file(&path).await.unwrap();
        Ok(platform.binary.get_command(&path))
    }

    pub async fn prepare_input(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        info!("Preparing input for assignment {}", self.assignment.id);

        let dir = Path::new("projects")
            .join(&self.assignment.project.name)
            .join("inputs");

        if !&dir.exists() {
            fs::create_dir_all(&dir)
                .await
                .expect("failed to create platforms directory");
        }

        let path = dir.join(format!("{}.bin", self.assignment.id));
        fs::write(&path, &self.assignment.input_data).await.unwrap();
        Ok(path)
    }

    pub async fn run(&self, platform_ids: &Vec<i64>) -> Result<AssignmentResult, Box<dyn std::error::Error>> {
        info!("Running assignment {}", self.assignment.id);
        let platform = self.get_platform(platform_ids)?;
        let mut command = self.prepare_binary(platform).await?;
        let input_path = self.prepare_input().await?;

        command.arg("--input");
        command.arg(&input_path.canonicalize()?.to_str().unwrap());

        let start = std::time::Instant::now();
        let output = command.output().await?;
        if output.status.success() {
            info!("Assignment {} finished successfully", self.assignment.id);
            Ok(AssignmentResult::new(
                self.assignment.id,
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
                output.status.code().unwrap(),
                start.elapsed().as_nanos()
            ))
        } else {
            error!("assignment {} failed", self.assignment.id);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to run binary",
            )))
        }
    }
}