use std::sync::Arc;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::orders::{Order, OrderSide, OrderStatus},
    errors::AppError,
    repositories::{market::MarketRepository, orders::OrderRepository, wallet::WalletRepository},
    services::market_service::normalize_market_symbol,
};

#[derive(Clone, Debug)]
pub struct OrderFilter {
    pub status: Option<OrderStatus>,
    pub side: Option<OrderSide>,
    pub market_symbol: Option<String>,
}

#[derive(Clone)]
pub struct OrderService {
    order_repository: Arc<dyn OrderRepository>,
    wallet_repository: Arc<dyn WalletRepository>,
    market_repository: Arc<dyn MarketRepository>,
}

#[derive(Debug, Clone)]
pub struct OrderPagination {
    pub items: Vec<Order>,
    pub page: u32,
    pub per_page: u32,
    pub total_items: i64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

#[derive(Debug, Clone)]
pub struct OrderStats {
    pub total_orders: i64,
    pub pending_orders: i64,
    pub filled_orders: i64,
    pub cancelled_order: i64,

    pub buy_orders: i64,
    pub sell_orders: i64,

    pub total_trade_volume: Decimal,
    pub total_fees: Decimal,
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
        filter: OrderFilter,
    ) -> Result<OrderPagination, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);

        let offset = u64::from(page - 1)
            .checked_mul(u64::from(per_page))
            .and_then(|value| i64::try_from(value).ok())
            .unwrap_or(i64::MAX);

        let total_items = self
            .order_repository
            .count_by_user_id(user_id, &filter)
            .await?;

        let items = self
            .order_repository
            .find_by_user_id(user_id, i64::from(per_page), offset, &filter)
            .await?;

        let total_pages = if total_items == 0 {
            0
        } else {
            ((total_items as u64 + u64::from(per_page) - 1) / u64::from(per_page)) as u32
        };

        Ok(OrderPagination {
            items,
            page,
            per_page,
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1 && total_pages > 0,
        })
    }

    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<OrderStats, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        self.order_repository.get_user_stats(user_id).await
    }
}

pub fn build_filter(
    status: Option<String>,
    side: Option<String>,
    market: Option<String>,
) -> Result<OrderFilter, AppError> {
    let status = match status {
        Some(value) => Some(OrderStatus::from_str(&value.trim().to_lowercase())?),
        None => None,
    };

    let side = match side {
        Some(value) => Some(OrderSide::from_str(&value.trim().to_lowercase())?),
        None => None,
    };

    let market_symbol = match market {
        Some(value) => Some(normalize_market_symbol(value)?),
        None => None,
    };

    Ok(OrderFilter {
        status,
        side,
        market_symbol,
    })
}
