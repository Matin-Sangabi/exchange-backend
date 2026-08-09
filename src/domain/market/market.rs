use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{domain::wallet_asset::normalize_symbol, errors::AppError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketStatus {
    Active,
    Inactive,
}

impl MarketStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }

    pub fn from_str(value: &str) -> Result<Self, AppError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err(AppError::InvalidMarketStatus),
        }
    }
}

pub struct Market {
    id: Uuid,
    symbol: String,
    base_asset: String,
    quote_asset: String,
    status: MarketStatus,
    current_price: Option<Decimal>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Market {
    pub fn new(
        base_asset: impl Into<String>,
        quote_asset: impl Into<String>,
    ) -> Result<Self, AppError> {
        let base_asset = normalize_symbol(base_asset.into())?;

        let quote_asset = normalize_symbol(quote_asset.into())?;

        if base_asset == quote_asset {
            return Err(AppError::SameMarketAssets);
        }

        let symbol = format!("{base_asset}-{quote_asset}");

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            symbol,
            base_asset,
            quote_asset,
            status: MarketStatus::Active,
            current_price: None,
            created_at: now,
            updated_at: now,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restore(
        id: Uuid,
        symbol: String,
        base_asset: String,
        quote_asset: String,
        status: MarketStatus,
        current_price: Option<Decimal>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            symbol,
            base_asset,
            quote_asset,
            status,
            current_price,
            created_at,
            updated_at,
        }
    }

    pub fn set_price(&mut self, price: Decimal) -> Result<(), AppError> {
        if price <= Decimal::ZERO {
            return Err(AppError::InvalidMarketPrice);
        }

        self.current_price = Some(price);
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn activate(&mut self) {
        self.status = MarketStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = MarketStatus::Inactive;
        self.updated_at = Utc::now();
    }

    pub fn price(&self) -> Result<Decimal, AppError> {
        self.current_price.ok_or(AppError::MarketNotFound)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn base_asset(&self) -> &str {
        &self.base_asset
    }

    pub fn quote_asset(&self) -> &str {
        &self.quote_asset
    }

    pub fn status(&self) -> MarketStatus {
        self.status
    }

    pub fn current_price(&self) -> Option<Decimal> {
        self.current_price
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
