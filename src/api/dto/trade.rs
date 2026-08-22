use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::trades::trade::Trade;

#[derive(Debug, Deserialize)]
pub struct TradeListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
#[derive(Debug, Serialize)]
pub struct TradeResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,

    pub market_symbol: String,
    pub side: String,

    pub price: String,
    pub quantity: String,
    pub quote_amount: String,

    pub fee_amount: String,
    pub fee_percent: String,

    pub executed_at: DateTime<Utc>,
}

impl From<Trade> for TradeResponse {
    fn from(trade: Trade) -> Self {
        Self {
            id: trade.id(),
            order_id: trade.order_id(),
            user_id: trade.user_id(),
            wallet_id: trade.wallet_id(),

            market_symbol: trade.market_symbol().to_owned(),

            side: trade.side().as_str().to_owned(),

            price: trade.price().to_string(),
            quantity: trade.quantity().to_string(),
            quote_amount: trade.quote_amount().to_string(),

            fee_amount: trade.fee_amount().to_string(),

            fee_percent: trade.fee_percent().to_string(),

            executed_at: trade.executed_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TradeListResponse {
    pub page: u32,
    pub per_page: u32,
    pub items: Vec<TradeResponse>,
}
