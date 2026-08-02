mod api;
mod app;
mod config;
mod database;
mod domain;
mod errors;
mod middleware;
mod repositories;
mod services;
mod utils;

use anyhow::Result;
use tracing::{error, info};

use crate::{app::Application, config::AppConfig, utils::tracing::init_tracing, };

async fn run() -> Result<()> {
    let config = AppConfig::load()?;

    init_tracing()?;

    info!("Starting exchange backend");

    let application = Application::build(config).await?;

    application.run().await
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        error!(
            error = ?error,
            "Application failed"
        );

        eprintln!("Application failed:\n{error:#}");

        std::process::exit(1);
    }
}
