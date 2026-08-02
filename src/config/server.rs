use std::net::{IpAddr, SocketAddr};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn from_env() -> Result<Self> {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string())
            .parse::<IpAddr>()
            .context("Host must be valid Ip Address")?;

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("PORT must be valid number between 0 and 65365")?;
        Ok(Self { host, port })
    }

    
}
