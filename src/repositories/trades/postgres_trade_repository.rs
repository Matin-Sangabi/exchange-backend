use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::{orders::OrderSide, trades::trade::Trade},
    errors::AppError,
    repositories::trades::trade_repository::TradeRepository,
};

#[derive(Debug, FromRow)]
struct TradeRow {
    id: Uuid,
    order_id: Uuid,
    user_id: Uuid,
    wallet_id: Uuid,
    market_symbol: String,
    side: String,
    price: Decimal,
    quantity: Decimal,
    quote_amount: Decimal,
    fee_amount: Decimal,
    fee_percent: Decimal,
    executed_at: DateTime<Utc>,
}

impl TryFrom<TradeRow> for Trade {
    type Error = AppError;
    fn try_from(row: TradeRow) -> Result<Self, Self::Error> {
        let side = OrderSide::from_str(&row.side)?;

        Ok(Trade::restore(
            row.id,
            row.order_id,
            row.user_id,
            row.wallet_id,
            row.market_symbol,
            side,
            row.price,
            row.quantity,
            row.quote_amount,
            row.fee_amount,
            row.fee_percent,
            row.executed_at,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PostgresTradeRepository {
    pool: PgPool,
}

impl PostgresTradeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TradeRepository for PostgresTradeRepository {
    async fn find_by_id(&self, trade_id: Uuid) -> Result<Option<Trade>, AppError> {
        let row = sqlx::query_as::<_, TradeRow>(
            r#"
            SELECT
                id,
                order_id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                price,
                quantity,
                quote_amount,
                fee_amount,
                fee_percent,
                executed_at
            FROM trades
            WHERE id = $1
            "#,
        )
        .bind(trade_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(TryInto::try_into).transpose()
    }

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Trade>, AppError> {
        let rows = sqlx::query_as::<_, TradeRow>(
            r#"
            SELECT
                id,
                order_id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                price,
                quantity,
                quote_amount,
                fee_amount,
                fee_percent,
                executed_at
            FROM trades
            WHERE user_id = $1
            ORDER BY executed_at DESC, id DESC
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    async fn find_by_market(
        &self,
        market_symbol: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Trade>, AppError> {
        let rows = sqlx::query_as::<_, TradeRow>(
            r#"
            SELECT
                id,
                order_id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                price,
                quantity,
                quote_amount,
                fee_amount,
                fee_percent,
                executed_at
            FROM trades
            WHERE market_symbol = $1
            ORDER BY executed_at DESC, id DESC
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(market_symbol)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}
