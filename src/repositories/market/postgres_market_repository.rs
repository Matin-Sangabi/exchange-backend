use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    domain::market::{Market, MarketStatus},
    errors::AppError,
    repositories::market::market_repository::MarketRepository,
};

#[derive(Debug, FromRow)]
struct MarketRow {
    id: Uuid,
    symbol: String,
    base_asset: String,
    quote_asset: String,
    status: String,
    current_price: Option<Decimal>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<MarketRow> for Market {
    type Error = AppError;
    fn try_from(value: MarketRow) -> Result<Self, Self::Error> {
        let status = MarketStatus::from_str(&value.status)?;

        Ok(Market::restore(
            value.id,
            value.symbol,
            value.base_asset,
            value.quote_asset,
            status,
            value.current_price,
            value.created_at,
            value.updated_at,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PostgresMarketRepository {
    pool: PgPool,
}

impl PostgresMarketRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MarketRepository for PostgresMarketRepository {
    async fn create(&self, market: &Market) -> Result<Market, AppError> {
        let result = sqlx::query_as::<_, MarketRow>(
            r#"
            INSERT INTO markets (
                id,
                symbol,
                base_asset,
                quote_asset,
                status,
                current_price,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5::market_status,
                $6,
                $7,
                $8
            )
            RETURNING
                id,
                symbol,
                base_asset,
                quote_asset,
                status::text AS status,
                current_price,
                created_at,
                updated_at
            "#,
        )
        .bind(market.id())
        .bind(market.symbol())
        .bind(market.base_asset())
        .bind(market.quote_asset())
        .bind(market.status().as_str())
        .bind(market.current_price())
        .bind(market.created_at())
        .bind(market.updated_at())
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => row.try_into(),

            Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
                Err(AppError::MarketAlreadyExists)
            }

            Err(error) => Err(AppError::Database(error)),
        }
    }

    async fn find_all(&self) -> Result<Vec<Market>, AppError> {
        let rows = sqlx::query_as::<_, MarketRow>(
            r#"
            SELECT
                id,
                symbol,
                base_asset,
                quote_asset,
                status::text AS status,
                current_price,
                created_at,
                updated_at
            FROM markets
            ORDER BY symbol ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Market>, AppError> {
        let row = sqlx::query_as::<_, MarketRow>(
            r#"
            SELECT
                id,
                symbol,
                base_asset,
                quote_asset,
                status::text AS status,
                current_price,
                created_at,
                updated_at
            FROM markets
            WHERE symbol = $1
            "#,
        )
        .bind(symbol)
        .fetch_optional(&self.pool)
        .await?;

        row.map(TryInto::try_into).transpose()
    }

    async fn update(&self, market: &Market) -> Result<Market, AppError> {
        let row = sqlx::query_as::<_, MarketRow>(
            r#"
            UPDATE markets
            SET
                status = $2::market_status,
                current_price = $3,
                updated_at = $4
            WHERE id = $1
            RETURNING
                id,
                symbol,
                base_asset,
                quote_asset,
                status::text AS status,
                current_price,
                created_at,
                updated_at
            "#,
        )
        .bind(market.id())
        .bind(market.status().as_str())
        .bind(market.current_price())
        .bind(market.updated_at())
        .fetch_optional(&self.pool)
        .await?;

        let row = row.ok_or(AppError::MarketNotFound)?;

        row.try_into()
    }
}
