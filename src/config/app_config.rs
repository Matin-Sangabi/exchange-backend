use std::str::FromStr;

use anyhow::{Context, Result};

use crate::config::trading::TradingConfig;

use super::{database::DatabaseConfig, env::AppEnvironment, server::ServerConfig};

#[derive(Debug, Clone)]

pub struct AppConfig {
    pub app_name: String,
    pub env: AppEnvironment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub trading: TradingConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        dotenvy::dotenv().context("Failed to load or parse the .env file")?;

        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "Exchange Backend".to_string());

        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let environment =
            AppEnvironment::from_str(&environment).context("Failed to parse APP_ENV")?;

        let server = ServerConfig::from_env().context("Failed to load server configuration")?;

        let database =
            DatabaseConfig::from_env().context("Failed to load database configuration")?;

        let trading = TradingConfig::from_env().context("Failed to load trading configuration")?;

        Ok(Self {
            app_name,
            env: environment,
            server,
            database,
            trading,
        })
    }
}
