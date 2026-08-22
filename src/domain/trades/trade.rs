use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::orders::OrderSide;

#[derive(Debug, Clone)]
pub struct Trade {
    id: Uuid,
    user_id: Uuid,
    order_id: Uuid,
    wallet_id: Uuid,
    market_symbol: String,
    side: OrderSide,
    price: Decimal,
    quantity: Decimal,
    quote_amount: Decimal,
    fee_amount: Decimal,
    fee_percent: Decimal,
    executed_at: DateTime<Utc>,
}

impl Trade {
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: Uuid,
        order_id: Uuid,
        user_id: Uuid,
        wallet_id: Uuid,
        market_symbol: String,
        side: OrderSide,
        price: Decimal,
        quantity: Decimal,
        quote_amount: Decimal,
        fee_amount: Decimal,
        fee_percent: Decimal,
        executed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            order_id,
            user_id,
            wallet_id,
            market_symbol,
            side,
            price,
            quantity,
            quote_amount,
            fee_amount,
            fee_percent,
            executed_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn order_id(&self) -> Uuid {
        self.order_id
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

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn quote_amount(&self) -> Decimal {
        self.quote_amount
    }

    pub fn fee_amount(&self) -> Decimal {
        self.fee_amount
    }

    pub fn fee_percent(&self) -> Decimal {
        self.fee_percent
    }

    pub fn executed_at(&self) -> DateTime<Utc> {
        self.executed_at
    }
}
