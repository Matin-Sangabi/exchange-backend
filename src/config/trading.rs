use std::str::FromStr;

use anyhow::{Context, Result};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct TradingConfig {
    pub fee_percent: Decimal,
}

impl TradingConfig {
    pub fn from_env() -> Result<Self> {
        let fee_percent =
            std::env::var("TRADING_FEE_PERCENT").unwrap_or_else(|_| "0.2".to_string());

        let fee_percent = Decimal::from_str(&fee_percent)
            .context("TRADING_FEE_PERCENT must be a valid decimal")?;
        if fee_percent < Decimal::ZERO {
            anyhow::bail!("TRADING_FEE_PERCENT cannot be negative")
        }
        Ok(Self { fee_percent })
    }
}
