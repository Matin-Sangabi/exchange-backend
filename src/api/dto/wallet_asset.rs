use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::wallet_asset::WalletAsset;

#[derive(Debug, Deserialize)]
pub struct WalletAssetAmountRequest {
    pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct WalletAssetResponse {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub symbol: String,
    pub balance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<WalletAsset> for WalletAssetResponse {
    fn from(value: WalletAsset) -> Self {
        Self {
            id: value.id(),
            wallet_id: value.wallet_id(),
            symbol: value.symbol().to_owned(),
            balance: value.balance().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WalletAssetListResponse {
    pub items: Vec<WalletAssetResponse>,
}
