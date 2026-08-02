use anyhow::{Context, Result};
use sqlx::PgPool;
use tracing::info;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("Running database migrations");

    MIGRATOR
        .run(pool)
        .await
        .context("Failed to run database migrations")?;

    info!("Database migrations completed");

    Ok(())
}
