use std::thread;
use std::time::Duration;

use clap::Parser;
use simplelog::{ColorChoice, Config, info, TerminalMode, TermLogger};
use tokio::time::Instant;

use crate::api::mcathome::api::MCAtHomeAPI;
use crate::manager::worker::WorkerThread;

pub mod api;
pub mod data;
pub mod manager;

#[derive(Parser, Debug)]
#[clap(author = "Koding", version = "0.1.0", about = "DICC Client")]
struct Opts {
    /// Minecraft@Home API key
    #[clap(short, long)]
    api_key: String,

    /// Worker count
    #[clap(short, long, default_value_t = 0)]
    workers: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    // Parse command line arguments
    let mut opts = Opts::parse();

    if opts.workers == 0 {
        opts.workers = num_cpus::get() / 2;
    }

    info!("");
    info!("<bold><blue>DICC Client</>");
    info!("<bold><blue>Version: 0.1.0</>");
    info!("<bold><blue>Using {} workers</>", opts.workers);
    info!("");

    // Fetch platforms
    info!("<green><bold>Fetching platforms...</>");
    let api = MCAtHomeAPI::new(opts.api_key.as_str());
    let platforms = api.list_platforms().await?;

    let mut manager = manager::platform::PlatformManager::new();
    for platform in platforms {
        manager.add(platform);
    }

    let mut ts = Instant::now();
    info!("<green><bold>Detecting platforms...</>");

    let valid_platforms = manager.detect().await;
    info!("<green><bold>Detected in {}ms. Found {} platform(s).</>", ts.elapsed().as_millis(), valid_platforms.len());

    // Find projects
    ts = Instant::now();
    info!("<green><bold>Fetching projects...</>");
    let projects = api.get_projects_for_platforms(&valid_platforms).await?;
    info!("<green><bold>Found {} project(s) in {}ms.</>", projects.len(), ts.elapsed().as_millis());

    for project in &projects {
        info!("<bold>{} - {}</>", project.id, project.name);
        for (_, platform) in &project.platforms {
            info!(" - <bright-black>{}</>", platform.platform.name);
        }
    }

    info!("<green><bold>Creating threads...</>");
    let platform_ids = valid_platforms.keys().cloned().collect::<Vec<i64>>();

    for i in 0..opts.workers {
        let worker = WorkerThread::new(i as i32, &api, &projects, &platform_ids);
        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            runtime.block_on(worker.run());
        });
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
