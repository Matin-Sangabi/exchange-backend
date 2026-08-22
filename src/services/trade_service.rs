use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::trades::trade::Trade, errors::AppError, repositories::trades::TradeRepository,
    services::market_service::normalize_market_symbol,
};

#[derive(Clone)]
pub struct TradeService {
    repository: Arc<dyn TradeRepository>,
}

impl TradeService {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_trade(&self, trade_id: Uuid) -> Result<Trade, AppError> {
        self.repository
            .find_by_id(trade_id)
            .await?
            .ok_or(AppError::TradeNotFound)
    }

    pub async fn get_user_trades(
        &self,
        user_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<Trade>, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);

        let offset = i64::from(page - 1).saturating_mul(i64::from(per_page));

        self.repository
            .find_by_user_id(user_id, i64::from(per_page), offset)
            .await
    }

    pub async fn get_market_trades(
        &self,
        market_symbol: String,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<Trade>, AppError> {
        let market_symbol = normalize_market_symbol(market_symbol)?;

        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);

        let offset = i64::from(page - 1).saturating_mul(i64::from(per_page));

        self.repository
            .find_by_market(&market_symbol, i64::from(per_page), offset)
            .await
    }
}
