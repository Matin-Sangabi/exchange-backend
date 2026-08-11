use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }

    pub fn from_str(value: &str) -> Result<Self, AppError> {
        match value {
            "sell" => Ok(Self::Sell),
            "buy" => Ok(Self::Buy),
            _ => Err(AppError::InvalidOrderSide),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Filled => "filled",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(value: &str) -> Result<Self, AppError> {
        match value {
            "pending" => Ok(Self::Pending),
            "filled" => Ok(Self::Filled),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(AppError::InvalidOrderStatus),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    id: Uuid,
    user_id: Uuid,
    wallet_id: Uuid,
    market_symbol: String,
    side: OrderSide,
    status: OrderStatus,
    quantity: Decimal,
    price: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl Order {
    pub fn new(
        user_id: Uuid,
        wallet_id: Uuid,
        market_symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
    ) -> Result<Self, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        if quantity <= Decimal::ZERO {
            return Err(AppError::InvalidOrderQuantity);
        }

        if price <= Decimal::ZERO {
            return Err(AppError::InvalidOrderPrice);
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            wallet_id,
            market_symbol,
            side,
            status: OrderStatus::Pending,
            quantity,
            price,
            created_at: now,
            updated_at: now,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restore(
        id: Uuid,
        user_id: Uuid,
        wallet_id: Uuid,
        market_symbol: String,
        side: OrderSide,
        status: OrderStatus,
        quantity: Decimal,
        price: Decimal,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            wallet_id,
            market_symbol,
            side,
            status,
            quantity,
            price,
            created_at,
            updated_at,
        }
    }

    pub fn fill(&mut self) -> Result<(), AppError> {
        if self.status != OrderStatus::Pending {
            return Err(AppError::OrderAlreadyProcessed);
        }

        self.status = OrderStatus::Filled;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), AppError> {
        if self.status != OrderStatus::Pending {
            return Err(AppError::OrderAlreadyProcessed);
        }

        self.status = OrderStatus::Cancelled;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn total_value(&self) -> Decimal {
        self.price * self.quantity
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn wallet_id(&self) -> Uuid {
        self.wallet_id
    }

    pub fn market_symbol(&self) -> &str {
        &self.market_symbol
    }

    pub fn side(&self) -> OrderSide {
        self.side
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
