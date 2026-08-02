use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgConnection, PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::wallet::Wallet, errors::AppError,
    repositories::wallet::wallet_repository::WalletRepository,
};

#[derive(Debug, FromRow)]
struct WalletRow {
    id: Uuid,
    user_id: Uuid,
    cash_balance: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<WalletRow> for Wallet {
    fn from(value: WalletRow) -> Self {
        Wallet::restore(
            value.id,
            value.user_id,
            value.cash_balance,
            value.created_at,
            value.updated_at,
        )
    }
}

#[derive(Debug, Clone)]
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WalletRepository for PostgresWalletRepository {
    async fn create(&self, wallet: &Wallet) -> Result<Wallet, AppError> {
        let result = sqlx::query_as::<_, WalletRow>(
            r#"
            INSERT INTO wallets (
                id,
                user_id,
                cash_balance,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                user_id,
                cash_balance,
                created_at,
                updated_at
            "#,
        )
        .bind(wallet.id())
        .bind(wallet.user_id())
        .bind(wallet.cash_balance())
        .bind(wallet.created_at())
        .bind(wallet.updated_at())
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => Ok(row.into()),

            Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
                Err(AppError::WalletAlreadyExists)
            }

            Err(error) => Err(AppError::Database(error)),
        }
    }

    async fn find_by_id(&self, wallet_id: Uuid) -> Result<Option<Wallet>, AppError> {
        let row = sqlx::query_as::<_, WalletRow>(
            r#"
            SELECT
                id,
                user_id,
                cash_balance,
                created_at,
                updated_at
            FROM wallets
            WHERE id = $1
            "#,
        )
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, AppError> {
        let row = sqlx::query_as::<_, WalletRow>(
            r#"
            SELECT
                id,
                user_id,
                cash_balance,
                created_at,
                updated_at
            FROM wallets
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_id_for_update(
        &self,
        connection: &mut PgConnection,
        wallet_id: Uuid,
    ) -> Result<Option<Wallet>, AppError> {
        let row = sqlx::query_as::<_, WalletRow>(
            r#"
        SELECT
            id,
            user_id,
            cash_balance,
            created_at,
            updated_at
        FROM wallets
        WHERE id = $1
        FOR UPDATE
        "#,
        )
        .bind(wallet_id)
        .fetch_optional(connection)
        .await?;

        Ok(row.map(Into::into))
    }
    async fn update(
        &self,
        connection: &mut PgConnection,
        wallet: &Wallet,
    ) -> Result<Wallet, AppError> {
        let row = sqlx::query_as::<_, WalletRow>(
            r#"
        UPDATE wallets
        SET
            cash_balance = $2,
            updated_at = $3
        WHERE id = $1
        RETURNING
            id,
            user_id,
            cash_balance,
            created_at,
            updated_at
        "#,
        )
        .bind(wallet.id())
        .bind(wallet.cash_balance())
        .bind(wallet.updated_at())
        .fetch_optional(connection)
        .await?;

        row.map(Into::into).ok_or(AppError::WalletNotFound)
    }
}
