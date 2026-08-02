use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::wallet::Wallet, errors::AppError};

#[async_trait]

pub trait WalletRepository: Send + Sync {
    async fn create(&self, wallet: &Wallet) -> Result<Wallet, AppError>;

    async fn find_by_id(&self, wallet_id: Uuid) -> Result<Option<Wallet>, AppError>;

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, AppError>;
}
