use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{domain::wallet_transaction::WalletTransaction, errors::AppError};

#[async_trait]
pub trait WalletTransactionRepository: Send + Sync {
    async fn create(
        &self,
        connection: &mut PgConnection,
        transaction: &WalletTransaction,
    ) -> Result<WalletTransaction, AppError>;

    async fn find_by_reference_id(
        &self,
        reference_id: Uuid,
    ) -> Result<Option<WalletTransaction>, AppError>;

    async fn find_by_wallet_id(
        &self,
        wallet_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WalletTransaction>, AppError>;
}
