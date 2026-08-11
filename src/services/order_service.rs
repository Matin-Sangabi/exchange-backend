use std::sync::Arc;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    api::dto::wallet,
    domain::orders::{Order, OrderSide},
    errors::AppError,
    repositories::{market::MarketRepository, orders::OrderRepository, wallet::WalletRepository},
    services::market_service::normalize_market_symbol,
};

#[derive(Clone)]
pub struct OrderService {
    order_repository: Arc<dyn OrderRepository>,
    wallet_repository: Arc<dyn WalletRepository>,
    market_repository: Arc<dyn MarketRepository>,
}

impl OrderService {
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        wallet_repository: Arc<dyn WalletRepository>,
        market_repository: Arc<dyn MarketRepository>,
    ) -> Self {
        Self {
            order_repository,
            wallet_repository,
            market_repository,
        }
    }

    pub async fn create_order(
        &self,
        user_id: Uuid,
        wallet_id: Uuid,
        market_symbol: String,
        side: OrderSide,
        quantity: Decimal,
    ) -> Result<Order, AppError> {
        let market_symbol = normalize_market_symbol(market_symbol)?;

        let wallet = self
            .wallet_repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        if wallet.user_id() != user_id {
            return Err(AppError::WalletNotFound);
        }

        let market = self
            .market_repository
            .find_by_symbol(&market_symbol)
            .await?
            .ok_or(AppError::MarketNotFound)?;

        let price = market.price()?;

        let order = Order::new(user_id, wallet_id, market_symbol, side, quantity, price)?;

        self.order_repository.create(&order).await
    }

    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, AppError> {
        self.order_repository
            .find_by_id(order_id)
            .await?
            .ok_or(AppError::OrderNotFound)
    }

    pub async fn get_user_order(
        &self,
        user_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<Order>, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);

        let offset = u64::from(page - 1)
            .checked_mul(u64::from(per_page))
            .and_then(|value| i64::try_from(value).ok())
            .unwrap_or(i64::MAX);

        self.order_repository
            .find_by_user_id(user_id, i64::from(per_page), offset)
            .await
    }
}
