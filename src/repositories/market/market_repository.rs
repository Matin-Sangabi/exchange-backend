use async_trait::async_trait;

use crate::{domain::market::Market, errors::AppError};

#[async_trait]
pub trait MarketRepository: Send + Sync {
    async fn create(&self, market: &Market) -> Result<Market, AppError>;

    async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Market>, AppError>;

    async fn find_all(&self) -> Result<Vec<Market>, AppError>;

    async fn update(&self, market: &Market) -> Result<Market, AppError>;

    
}
