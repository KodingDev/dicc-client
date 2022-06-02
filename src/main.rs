use clap::Parser;
use simplelog::{ColorChoice, Config, info, TerminalMode, TermLogger};
use tokio::time::Instant;

use crate::api::mcathome::api::MCAtHomeAPI;

pub mod api;
pub mod data;
pub mod manager;

#[derive(Parser, Debug)]
#[clap(author = "Koding", version = "0.1.0", about = "DICC Client")]
struct Opts {
    /// Minecraft@Home API key
    #[clap(short, long)]
    api_key: String,
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
    let opts = Opts::parse();

    info!("");
    info!("<bold><blue>DICC Client</>");
    info!("<bold><blue>Version: 0.1.0</>");
    info!("");

    // Fetch platforms
    info!("<green><bold>Fetching platforms...</>");
    let api = MCAtHomeAPI::new(opts.api_key.as_str());
    let platforms = api.list_platforms().await?;

    let mut manager = manager::platform::PlatformManager::new();
    for platform in platforms {
        manager.add(platform);
    }

    let ts = Instant::now();
    info!("<green><bold>Detecting platforms...</>");

    let valid = manager.detect().await;
    info!("<green><bold>Detected in {}ms. Found {} platform(s).</>", ts.elapsed().as_millis(), valid.len());

    // Find projects
    info!("<green><bold>Fetching projects...</>");
    let platform_ids = valid.iter().map(|&p| p.id.as_str().to_string()).collect::<Vec<String>>();
    let projects = api.get_projects_for_platforms(platform_ids).await?;
    println!("{:#?}", projects);

    Ok(())
}
