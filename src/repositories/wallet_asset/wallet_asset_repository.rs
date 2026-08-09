use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{domain::wallet_asset::WalletAsset, errors::AppError};

#[async_trait]
pub trait WalletAssetRepository: Send + Sync {
    async fn create(
        &self,
        connection: &mut PgConnection,
        asset: &WalletAsset,
    ) -> Result<WalletAsset, AppError>;

    async fn find_by_wallet_and_symbol(
        &self,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<Option<WalletAsset>, AppError>;

    async fn find_by_wallet_and_symbol_for_update(
        &self,
        connection: &mut PgConnection,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<Option<WalletAsset>, AppError>;

    async fn update(
        &self,
        connection: &mut PgConnection,
        asset: &WalletAsset,
    ) -> Result<WalletAsset, AppError>;

    async fn find_all_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<WalletAsset>, AppError>;
}
