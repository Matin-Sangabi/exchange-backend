use std::sync::Arc;

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{
        wallet::Wallet,
        wallet_transaction::{WalletTransaction, WalletTransactionType},
    },
    errors::AppError,
    repositories::{wallet::WalletRepository, wallet_transaction::WalletTransactionRepository},
};

#[derive(Clone)]
pub struct WalletService {
    pool: PgPool,
    wallet_repository: Arc<dyn WalletRepository>,
    transaction_repository: Arc<dyn WalletTransactionRepository>,
}

impl WalletService {
    pub fn new(
        pool: PgPool,
        wallet_repository: Arc<dyn WalletRepository>,
        transaction_repository: Arc<dyn WalletTransactionRepository>,
    ) -> Self {
        Self {
            wallet_repository,
            pool,
            transaction_repository,
        }
    }

    pub async fn create_wallet(
        &self,
        user_id: Uuid,
        initial_balance: Decimal,
    ) -> Result<Wallet, AppError> {
        let wallet = Wallet::new(user_id, initial_balance)?;
        self.wallet_repository.create(&wallet).await
    }

    pub async fn get_wallet_by_id(&self, wallet_id: Uuid) -> Result<Wallet, AppError> {
        self.wallet_repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)
    }

    pub async fn get_wallet_by_user_id(&self, user_id: Uuid) -> Result<Wallet, AppError> {
        if user_id.is_nil() {
            return Err(AppError::InvalidUserId);
        }

        self.wallet_repository
            .find_by_user_id(user_id)
            .await?
            .ok_or(AppError::WalletNotFound)
    }

    pub async fn deposit(
        &self,
        wallet_id: Uuid,
        amount: Decimal,
        reference_id: Uuid,
        description: Option<String>,
    ) -> Result<Wallet, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }
        if reference_id.is_nil() {
            return Err(AppError::InvalidReferenceId);
        }

        let mut database_transaction = self.pool.begin().await?;

        let mut wallet = self
            .wallet_repository
            .find_by_id_for_update(&mut *database_transaction, wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        let balance_before = wallet.cash_balance();

        wallet.deposit(amount)?;

        let balance_after = wallet.cash_balance();

        let ledger_entry = WalletTransaction::new(
            wallet.id(),
            WalletTransactionType::Deposit,
            amount,
            balance_before,
            balance_after,
            reference_id,
            description,
        )?;

        let updated_wallet = self
            .wallet_repository
            .update(&mut *database_transaction, &wallet)
            .await?;

        self.transaction_repository
            .create(&mut *database_transaction, &ledger_entry)
            .await?;

        database_transaction.commit().await?;

        Ok(updated_wallet)
    }

    pub async fn withdraw(
        &self,
        wallet_id: Uuid,
        amount: Decimal,
        reference_id: Uuid,
        description: Option<String>,
    ) -> Result<Wallet, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        if reference_id.is_nil() {
            return Err(AppError::InvalidReferenceId);
        }

        let mut database_transaction = self.pool.begin().await?;

        let mut wallet = self
            .wallet_repository
            .find_by_id_for_update(&mut *database_transaction, wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        let balance_before = wallet.cash_balance();

        wallet.withdraw(amount)?;

        let balance_after = wallet.cash_balance();

        let ledger_entry = WalletTransaction::new(
            wallet_id,
            WalletTransactionType::WithDraw,
            amount,
            balance_before,
            balance_after,
            reference_id,
            description,
        )?;

        let update_wallet = self
            .wallet_repository
            .update(&mut *database_transaction, &wallet)
            .await?;

        self.transaction_repository
            .create(&mut *database_transaction, &ledger_entry)
            .await?;

        database_transaction.commit().await?;

        Ok(update_wallet)
    }

    pub async fn get_wallet_transactions(
        &self,
        wallet_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<WalletTransaction>, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }

        self.wallet_repository
            .find_by_id(wallet_id)
            .await?
            .ok_or(AppError::WalletNotFound)?;

        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);

        let limit = i64::from(per_page);
        let offset = i64::from((page - 1) * per_page);

        self.transaction_repository
            .find_by_wallet_id(wallet_id, limit, offset)
            .await
    }
}
