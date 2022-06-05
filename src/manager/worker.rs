use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use simplelog::{error, info};
use tokio::fs;
use tokio::process::Command;

use crate::data::assignment::{Assignment, AssignmentResult};
use crate::data::project::{Project, ProjectPlatform};
use crate::MCAtHomeAPI;

pub struct ProjectWorker {
    pub assignment: Assignment,
}

pub struct WorkerThread {
    pub id: i32,
    pub api: MCAtHomeAPI,
    pub projects: Vec<Project>,
    pub platform_ids: Vec<i64>,
}

impl WorkerThread {
    pub fn new(id: i32, api: &MCAtHomeAPI, projects: &Vec<Project>, platform_ids: &Vec<i64>) -> WorkerThread {
        WorkerThread {
            id,
            api: api.clone(),
            projects: projects.clone(),
            platform_ids: platform_ids.clone(),
        }
    }

    pub async fn run(&self) {
        info!("Starting worker thread #{}", self.id);
        loop {
            self.run_loop().await.expect("Failed to run worker thread");
        }
    }

    async fn run_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ts = Instant::now();
        let assignments = self.api.get_assignments(&self.projects).await?;
        info!("<green><bold>Assigned {} task(s) in {}ms.</>", assignments.len(), ts.elapsed().as_millis());

        if assignments.len() == 0 {
            info!("<red><bold>No tasks to do. Sleeping for 1 minute.</>");
            tokio::time::sleep(Duration::from_secs(60)).await;
            return Ok(());
        }

        for assignment in &assignments {
            let worker = assignment.create_worker();
            let output = worker.run(&self.platform_ids).await?;
            self.api.submit_result(&output).await?;
            info!("<green><bold>Submitted result for assignment {}.</>", assignment.id);
        }
        Ok(())
    }
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

        let start = Instant::now();
        let output = command.output().await?;
        if output.status.success() {
            info!("Assignment {} finished successfully", self.assignment.id);
            Ok(AssignmentResult::new(
                self.assignment.id,
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
                output.status.code().unwrap(),
                start.elapsed().as_nanos(),
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