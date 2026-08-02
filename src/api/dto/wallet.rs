use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::wallet::Wallet;

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
    pub initial_balance: String,
}

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Wallet> for WalletResponse {
    fn from(value: Wallet) -> Self {
        WalletResponse {
            id: value.id(),
            user_id: value.user_id(),
            cash_balance: value.cash_balance(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WalletAmountRequest {
    pub amount: String,
}
