use std::sync::Arc;

use rust_decimal::Decimal;

use crate::{
    domain::{market::Market, wallet_asset::normalize_symbol},
    errors::AppError,
    repositories::market::MarketRepository,
};

#[derive(Clone)]
pub struct MarketService {
    repository: Arc<dyn MarketRepository>,
}

impl MarketService {
    pub fn new(repository: Arc<dyn MarketRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_market(
        &self,
        base_asset: String,
        quote_asset: String,
    ) -> Result<Market, AppError> {
        let market = Market::new(base_asset, quote_asset)?;

        self.repository.create(&market).await
    }

    pub async fn get_market(&self, symbol: String) -> Result<Market, AppError> {
        let symbol = normalize_market_symbol(symbol)?;

        self.repository
            .find_by_symbol(&symbol)
            .await?
            .ok_or(AppError::MarketNotFound)
    }

    pub async fn get_markets(&self) -> Result<Vec<Market>, AppError> {
        self.repository.find_all().await
    }

    pub async fn set_price(&self, symbol: String, price: Decimal) -> Result<Market, AppError> {
        let symbol = normalize_market_symbol(symbol)?;

        let mut market = self
            .repository
            .find_by_symbol(&symbol)
            .await?
            .ok_or(AppError::MarketNotFound)?;

        market.set_price(price)?;

        self.repository.update(&market).await
    }

    pub async fn get_price(&self, symbol: String) -> Result<Decimal, AppError> {
        let symbol = normalize_market_symbol(symbol)?;
        let market = self.get_market(symbol).await?;
        market.price()
    }
}

pub fn normalize_market_symbol(symbol: impl AsRef<str>) -> Result<String, AppError> {
    let symbol = symbol.as_ref().trim().to_uppercase();

    let mut parts = symbol.split("-");

    let base = parts.next().ok_or(AppError::InvalidMarketSymbol)?;

    let quote = parts.next().ok_or(AppError::InvalidMarketSymbol)?;

    if parts.next().is_some() {
        return Err(AppError::InvalidMarketSymbol);
    }

    let base = normalize_symbol(base)?;
    let quote = normalize_symbol(quote)?;

    if base == quote {
        return Err(AppError::SameMarketAssets);
    }

    Ok(format!("{base}-{quote}"))
}
