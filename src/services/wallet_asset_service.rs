use std::sync::Arc;

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::wallet_asset::{WalletAsset, normalize_symbol},
    errors::AppError,
    repositories::{wallet::WalletRepository, wallet_asset::WalletAssetRepository},
};

#[derive(Clone)]
pub struct WalletAssetService {
    pool: PgPool,
    wallet_repository: Arc<dyn WalletRepository>,
    asset_repository: Arc<dyn WalletAssetRepository>,
}

impl WalletAssetService {
    pub fn new(
        pool: PgPool,
        wallet_repository: Arc<dyn WalletRepository>,
        asset_repository: Arc<dyn WalletAssetRepository>,
    ) -> Self {
        Self {
            pool,
            wallet_repository,
            asset_repository,
        }
    }

    pub async fn get_assets(&self, wallet_id: Uuid) -> Result<Vec<WalletAsset>, AppError> {
        self.ensure_wallet_exist(wallet_id).await?;

        self.asset_repository.find_all_by_wallet_id(wallet_id).await
    }

    pub async fn get_asset(
        &self,
        wallet_id: Uuid,
        symbol: String,
    ) -> Result<WalletAsset, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        let symbol = normalize_symbol(symbol)?;

        self.asset_repository
            .find_by_wallet_and_symbol(wallet_id, &symbol)
            .await?
            .ok_or(AppError::WalletAssetNotFound)
    }

    pub async fn deposit(
        &self,
        wallet_id: Uuid,
        symbol: String,
        amount: Decimal,
    ) -> Result<WalletAsset, AppError> {
        let symbol = normalize_symbol(symbol)?;

        let mut transaction = self.pool.begin().await?;

        self.wallet_repository
            .find_by_id_for_update(&mut *transaction, wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        let existing_asset = self
            .asset_repository
            .find_by_wallet_and_symbol_for_update(&mut *transaction, wallet_id, &symbol)
            .await?;

        let updated_asset = match existing_asset {
            Some(mut asset) => {
                asset.deposit(amount)?;

                self.asset_repository
                    .update(&mut *transaction, &asset)
                    .await?
            }

            None => {
                let mut asset = WalletAsset::new(wallet_id, symbol)?;

                asset.deposit(amount)?;

                self.asset_repository
                    .create(&mut *transaction, &asset)
                    .await?
            }
        };

        transaction.commit().await?;

        Ok(updated_asset)
    }

    pub async fn withdraw(
        &self,
        wallet_id: Uuid,
        symbol: String,
        amount: Decimal,
    ) -> Result<WalletAsset, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        let symbol = normalize_symbol(symbol)?;

        self.wallet_repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        let mut transaction = self.pool.begin().await?;

        let mut asset = self
            .asset_repository
            .find_by_wallet_and_symbol_for_update(&mut *transaction, wallet_id, &symbol)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        asset.withdraw(amount)?;

        let update_asset = self
            .asset_repository
            .update(&mut transaction, &asset)
            .await?;

        transaction.commit().await?;

        Ok(update_asset)
    }

    async fn ensure_wallet_exist(&self, wallet_id: Uuid) -> Result<(), AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        self.wallet_repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        Ok(())
    }
}
