use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::market::Market;

#[derive(Debug, Deserialize)]
pub struct CreateMarketRequest {
    pub base_asset: String,
    pub quote_asset: String,
}
#[derive(Debug, Deserialize)]
pub struct SetMarketPriceRequest {
    pub price: String,
}

#[derive(Debug, Serialize)]
pub struct MarketResponse {
    pub id: Uuid,
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub status: String,
    pub current_price: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Market> for MarketResponse {
    fn from(value: Market) -> Self {
        Self {
            id: value.id(),
            symbol: value.symbol().to_owned(),
            base_asset: value.base_asset().to_owned(),
            quote_asset: value.quote_asset().to_owned(),
            status: value.status().as_str().to_owned(),
            current_price: value.current_price().map(|price| price.to_string()),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MarketListResponse {
    pub items: Vec<MarketResponse>,
}

#[derive(Debug , Serialize)]
pub struct MarketPriceResponse {
    pub symbol: String,
    pub price: String,
    pub updated_at: DateTime<Utc>,
}
