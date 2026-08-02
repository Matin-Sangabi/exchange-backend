use std::sync::Arc;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{domain::wallet::Wallet, errors::AppError, repositories::wallet::WalletRepository};

#[derive(Clone)]
pub struct WalletService {
    repository: Arc<dyn WalletRepository>,
}

impl WalletService {
    pub fn new(repository: Arc<dyn WalletRepository>) -> Self {
        Self { repository }
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
}
