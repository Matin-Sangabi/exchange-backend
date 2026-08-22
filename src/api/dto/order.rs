use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::orders::Order, services::order_service::OrderStats};

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
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total_items: i64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
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
    fn from(order: Order) -> Self {
        Self {
            id: order.id(),
            user_id: order.user_id(),
            wallet_id: order.wallet_id(),
            market_symbol: order.market_symbol().to_owned(),
            side: order.side().as_str().to_owned(),
            status: order.status().as_str().to_owned(),
            quantity: order.quantity().to_string(),
            price: order.price().to_string(),
            total_value: order.total_value().to_string(),
            fee_percent: order.fee_percent().map(|value| value.to_string()),
            fee_amount: order.fee_amount().map(|value| value.to_string()),
            executed_at: order.executed_at(),
            created_at: order.created_at(),
            updated_at: order.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrderListResponse {
    pub items: Vec<OrderResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct OrderStatsResponse {
    pub total_orders: i64,
    pub pending_orders: i64,
    pub filled_orders: i64,
    pub cancelled_orders: i64,

    pub buy_orders: i64,
    pub sell_orders: i64,

    pub total_trade_volume: String,
    pub total_fees: String,
}

impl From<OrderStats> for OrderStatsResponse {
    fn from(stats: OrderStats) -> Self {
        Self {
            total_orders: stats.total_orders,
            pending_orders: stats.pending_orders,
            filled_orders: stats.filled_orders,
            cancelled_orders: stats.cancelled_order,

            buy_orders: stats.buy_orders,
            sell_orders: stats.sell_orders,

            total_trade_volume: stats.total_trade_volume.to_string(),

            total_fees: stats.total_fees.to_string(),
        }
    }
}
