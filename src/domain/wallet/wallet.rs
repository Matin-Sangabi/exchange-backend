use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone)]

pub struct Wallet {
    id: Uuid,
    user_id: Uuid,
    cash_balance: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Wallet {
    pub fn new(user_id: Uuid, initial_balance: Decimal) -> Result<Self, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        if initial_balance.is_sign_negative() {
            return Err(AppError::InvalidBalanceFormat);
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            cash_balance: initial_balance,
            created_at: now,
            updated_at: now,
        })
    }

    pub(crate) fn restore(
        id: Uuid,
        user_id: Uuid,
        cash_balance: Decimal,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            cash_balance,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn cash_balance(&self) -> Decimal {
        self.cash_balance
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
