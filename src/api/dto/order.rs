use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::orders::Order;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub market_symbol: String,
    pub side: String,
    pub quantity: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,

    pub status: Option<String>,
    pub side: Option<String>,
    pub market: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,

    pub market_symbol: String,
    pub side: String,
    pub status: String,

    pub quantity: String,
    pub price: String,
    pub total_value: String,

    pub fee_percent: Option<String>,
    pub fee_amount: Option<String>,
    pub executed_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Order> for OrderResponse {
    fn from(value: Order) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            wallet_id: value.wallet_id(),
            market_symbol: value.market_symbol().to_owned(),
            side: value.side().as_str().to_owned(),
            status: value.status().as_str().to_owned(),
            quantity: value.quantity().to_string(),
            price: value.price().to_string(),
            total_value: value.total_value().to_string(),
            fee_percent: value.fee_percent().map(|value| value.to_string()),
            fee_amount: value.fee_amount().map(|value| value.to_string()),
            executed_at: value.executed_at(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrderListResponse {
    pub items: Vec<OrderResponse>,
}
