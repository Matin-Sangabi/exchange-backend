use async_trait::async_trait;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgConnection, PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::wallet_transaction::{WalletTransaction, WalletTransactionType},
    errors::AppError,
    repositories::wallet_transaction::wallet_transaction::WalletTransactionRepository,
};

#[derive(Debug, FromRow)]
struct WalletTransactionRow {
    id: Uuid,
    wallet_id: Uuid,
    transaction_type: String,
    amount: Decimal,
    balance_before: Decimal,
    balance_after: Decimal,
    reference_id: Uuid,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<WalletTransactionRow> for WalletTransaction {
    type Error = AppError;

    fn try_from(value: WalletTransactionRow) -> Result<Self, Self::Error> {
        let transaction_type = match value.transaction_type.as_str() {
            "deposit" => WalletTransactionType::Deposit,
            "withdraw" => WalletTransactionType::WithDraw,
            _ => {
                return Err(AppError::InvalidAmount);
            }
        };

        Ok(WalletTransaction::restore(
            value.id,
            value.wallet_id,
            transaction_type,
            value.amount,
            value.balance_before,
            value.balance_after,
            value.reference_id,
            value.description,
            value.created_at,
            value.updated_at,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PostgresWalletTransactionRepository {
    pool: PgPool,
}

impl PostgresWalletTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]

impl WalletTransactionRepository for PostgresWalletTransactionRepository {
    async fn create(
        &self,
        connection: &mut PgConnection,
        transaction: &WalletTransaction,
    ) -> Result<WalletTransaction, AppError> {
        let result = sqlx::query_as::<_, WalletTransactionRow>(
            r#"
                INSERT INTO wallet_transactions (
                    id,
                    wallet_id,
                    transaction_type,
                    amount,
                    balance_before,
                    balance_after,
                    reference_id,
                    description,
                    created_at,
                    updated_at
                )
                VALUES (
                    $1,
                    $2,
                    $3::wallet_transaction_type,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10
                )
                RETURNING
                    id,
                    wallet_id,
                    transaction_type::text
                        AS transaction_type,
                    amount,
                    balance_before,
                    balance_after,
                    reference_id,
                    description,
                    created_at,
                    updated_at
                "#,
        )
        .bind(transaction.id())
        .bind(transaction.wallet_id())
        .bind(transaction.transaction_type().as_str())
        .bind(transaction.amount())
        .bind(transaction.balance_before())
        .bind(transaction.balance_after())
        .bind(transaction.reference_id())
        .bind(transaction.description())
        .bind(transaction.created_at())
        .bind(transaction.updated_at())
        .fetch_one(connection)
        .await;

        match result {
            Ok(row) => row.try_into(),

            Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
                Err(AppError::DuplicateTransactionReference)
            }

            Err(error) => Err(AppError::Database(error)),
        }
    }

    async fn find_by_reference_id(
        &self,
        reference_id: Uuid,
    ) -> Result<Option<WalletTransaction>, AppError> {
        let row = sqlx::query_as::<_, WalletTransactionRow>(
            r#"
                SELECT
                    id,
                    wallet_id,
                    transaction_type::text
                        AS transaction_type,
                    amount,
                    balance_before,
                    balance_after,
                    reference_id,
                    description,
                    created_at,
                    updated_at
                FROM wallet_transactions
                WHERE reference_id = $1
                "#,
        )
        .bind(reference_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(TryInto::try_into).transpose()
    }

    async fn find_by_wallet_id(
        &self,
        wallet_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WalletTransaction>, AppError> {
        let rows = sqlx::query_as::<_, WalletTransactionRow>(
            r#"
                SELECT
                    id,
                    wallet_id,
                    transaction_type::text
                        AS transaction_type,
                    amount,
                    balance_before,
                    balance_after,
                    reference_id,
                    description,
                    created_at,
                    updated_at
                FROM wallet_transactions
                WHERE wallet_id = $1
                ORDER BY created_at DESC, id DESC
                LIMIT $2
                OFFSET $3
                "#,
        )
        .bind(wallet_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}
