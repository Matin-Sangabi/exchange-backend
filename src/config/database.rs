use anyhow::{Context, Result};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("DATABASE_URL").context("DATABASE URL variable is required")?;
        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .context("DATABASE_MAX_CONNECTIONS must be a valid number")?;

        if max_connections == 0 {
            anyhow::bail!("DATABASE_MAX_CONNECTIONS must be greater than 0");
        }

        Ok(Self {
            url,
            max_connections,
        })
    }
}

impl std::fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("url", &"[REDACTED]")
            .field("max_connections", &self.max_connections)
            .finish()
    }
}
