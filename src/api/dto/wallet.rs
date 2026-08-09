use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{wallet::Wallet, wallet_transaction::WalletTransaction};

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
    pub reference_id: Uuid,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WalletTransactionResponse {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: String,
    pub amount: String,
    pub balance_before: String,
    pub balance_after: String,
    pub reference_id: Uuid,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<WalletTransaction> for WalletTransactionResponse {
    fn from(value: WalletTransaction) -> Self {
        Self {
            id: value.id(),
            wallet_id: value.wallet_id(),
            transaction_type: value.transaction_type().as_str().to_lowercase(),
            amount: value.amount().to_string(),
            balance_before: value.balance_before().to_string(),
            balance_after: value.balance_after().to_string(),
            reference_id: value.reference_id(),
            description: value.description().map(str::to_owned),
            created_at: value.created_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionListQuery {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

#[derive(Debug, Serialize)]
pub struct WalletTransactionListResponse {
    pub page: u32,
    pub per_page: u32,
    pub items: Vec<WalletTransactionResponse>,
}
