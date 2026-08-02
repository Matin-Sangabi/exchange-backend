use std::time::Duration;

use crate::config::database::DatabaseConfig;
use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

pub mod migrations;

pub async fn create_database_pool(config: &DatabaseConfig) -> Result<PgPool> {
    info!(
        max_connections = config.max_connections,
        "Connection to postgres sql"
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.url)
        .await
        .context("Failed to connect to Postgres sql")?;

    info!("PostgreSQL connection pool created");

    Ok(pool)
}

pub async fn check_database_health(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .context("PostgreSQL health check failed")?;

    info!("PostgreSQL health check passed");

    Ok(())
}
