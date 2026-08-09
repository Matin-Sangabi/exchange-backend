use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgConnection, PgPool, postgres::PgConnectOptions, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::wallet_asset::WalletAsset, errors::AppError,
    repositories::wallet_asset::WalletAssetRepository,
};

#[derive(Debug, FromRow)]
struct WalletAssetRow {
    id: Uuid,
    wallet_id: Uuid,
    symbol: String,
    balance: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<WalletAssetRow> for WalletAsset {
    fn from(value: WalletAssetRow) -> Self {
        WalletAsset::restore(
            value.id,
            value.wallet_id,
            value.symbol,
            value.balance,
            value.created_at,
            value.updated_at,
        )
    }
}

#[derive(Debug, Clone)]
pub struct PostgresWalletAssetRepository {
    pool: PgPool,
}

impl PostgresWalletAssetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WalletAssetRepository for PostgresWalletAssetRepository {
    async fn create(
        &self,
        connection: &mut PgConnection,
        asset: &WalletAsset,
    ) -> Result<WalletAsset, AppError> {
        let result = sqlx::query_as::<_, WalletAssetRow>(
            r#"
                INSERT INTO wallet_assets (
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                "#,
        )
        .bind(asset.id())
        .bind(asset.wallet_id())
        .bind(asset.symbol())
        .bind(asset.balance())
        .bind(asset.created_at())
        .bind(asset.updated_at())
        .fetch_one(connection)
        .await;

        match result {
            Ok(row) => Ok(row.into()),

            Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
                Err(AppError::WalletAssetAlreadyExists)
            }

            Err(error) => Err(AppError::Database(error)),
        }
    }

    async fn find_by_wallet_and_symbol(
        &self,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<Option<WalletAsset>, AppError> {
        let row = sqlx::query_as::<_, WalletAssetRow>(
            r#"
                SELECT
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                FROM wallet_assets
                WHERE wallet_id = $1
                  AND symbol = $2
                "#,
        )
        .bind(wallet_id)
        .bind(symbol)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_wallet_and_symbol_for_update(
        &self,
        connection: &mut PgConnection,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<Option<WalletAsset>, AppError> {
        let row = sqlx::query_as::<_, WalletAssetRow>(
            r#"
                SELECT
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                FROM wallet_assets
                WHERE wallet_id = $1
                  AND symbol = $2
                FOR UPDATE
                "#,
        )
        .bind(wallet_id)
        .bind(symbol)
        .fetch_optional(connection)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn update(
        &self,
        connection: &mut PgConnection,
        asset: &WalletAsset,
    ) -> Result<WalletAsset, AppError> {
        let row = sqlx::query_as::<_, WalletAssetRow>(
            r#"
                UPDATE wallet_assets
                SET
                    balance = $2,
                    updated_at = $3
                WHERE id = $1
                RETURNING
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                "#,
        )
        .bind(asset.id())
        .bind(asset.balance())
        .bind(asset.updated_at())
        .fetch_optional(connection)
        .await?;

        row.map(Into::into).ok_or(AppError::WalletAssetNotFound)
    }

    async fn find_all_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<WalletAsset>, AppError> {
        let rows = sqlx::query_as::<_, WalletAssetRow>(
            r#"
                SELECT
                    id,
                    wallet_id,
                    symbol,
                    balance,
                    created_at,
                    updated_at
                FROM wallet_assets
                WHERE wallet_id = $1
                ORDER BY symbol ASC
                "#,
        )
        .bind(wallet_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
