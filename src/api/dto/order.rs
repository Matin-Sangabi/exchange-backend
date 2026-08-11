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
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrderListResponse {
    pub items: Vec<OrderResponse>,
}
