use anyhow::{Context, Result, anyhow};
use tracing_subscriber::EnvFilter;

pub fn init_tracing() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("exchange_backend=debug,tower_http=debug"))
        .context("Failed to create tracing environment filter")?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .compact()
        .try_init()
        .map_err(|error| anyhow!("Failed to initialize tracing subscriber: {error}"))?;

    Ok(())
}
