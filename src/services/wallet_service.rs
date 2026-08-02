use std::sync::Arc;

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::wallet::Wallet, errors::AppError, repositories::wallet::WalletRepository};

#[derive(Clone)]
pub struct WalletService {
    pool: PgPool,
    repository: Arc<dyn WalletRepository>,
}

impl WalletService {
    pub fn new(repository: Arc<dyn WalletRepository>, pool: PgPool) -> Self {
        Self { repository, pool }
    }

    pub async fn create_wallet(
        &self,
        user_id: Uuid,
        initial_balance: Decimal,
    ) -> Result<Wallet, AppError> {
        let wallet = Wallet::new(user_id, initial_balance)?;
        self.repository.create(&wallet).await
    }

    pub async fn get_wallet_by_id(&self, wallet_id: Uuid) -> Result<Wallet, AppError> {
        self.repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)
    }

    pub async fn get_wallet_by_user_id(&self, user_id: Uuid) -> Result<Wallet, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        self.repository
            .find_by_user_id(user_id)
            .await?
            .ok_or(AppError::WalletNotFound)
    }

    pub async fn deposit(&self, wallet_id: Uuid, amount: Decimal) -> Result<Wallet, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        let mut transaction = self.pool.begin().await?;

        let mut wallet = self
            .repository
            .find_by_id_for_update(&mut *transaction, wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        wallet.deposit(amount)?;

        let updated_wallet = self.repository.update(&mut *transaction, &wallet).await?;

        transaction.commit().await?;

        Ok(updated_wallet)
    }

    pub async fn withdraw(&self, wallet_id: Uuid, amount: Decimal) -> Result<Wallet, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        let mut transaction = self.pool.begin().await?;

        let mut wallet = self
            .repository
            .find_by_id_for_update(&mut *transaction, wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        wallet.withdraw(amount)?;

        let update_wallet = self.repository.update(&mut *transaction, &wallet).await?;

        transaction.commit().await?;

        Ok(update_wallet)
    }
}
