use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::trades::trade::Trade, errors::AppError};

#[async_trait]
pub trait TradeRepository: Send + Sync {
    async fn find_by_id(&self, trade_id: Uuid) -> Result<Option<Trade>, AppError>;

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Trade>, AppError>;

    async fn find_by_market(
        &self,
        market_symbol: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Trade>, AppError>;
}
