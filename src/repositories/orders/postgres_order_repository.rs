use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::orders::{Order, OrderSide, OrderStatus},
    errors::AppError,
    repositories::orders::order_repository::OrderRepository,
};

#[derive(Debug, FromRow)]
struct OrderRow {
    id: Uuid,
    user_id: Uuid,
    wallet_id: Uuid,
    market_symbol: String,
    side: String,
    status: String,
    quantity: Decimal,
    price: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<OrderRow> for Order {
    type Error = AppError;

    fn try_from(value: OrderRow) -> Result<Self, Self::Error> {
        let side = OrderSide::from_str(&value.side)?;

        let status = OrderStatus::from_str(&value.status)?;

        Ok(Order::restore(
            value.id,
            value.user_id,
            value.wallet_id,
            value.market_symbol,
            side,
            status,
            value.quantity,
            value.price,
            value.created_at,
            value.updated_at,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn create(&self, order: &Order) -> Result<Order, AppError> {
        let row = sqlx::query_as::<_, OrderRow>(
            r#"
            INSERT INTO orders (
                id,
                user_id,
                wallet_id,
                market_symbol,
                side,
                status,
                quantity,
                price,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5::order_side,
                $6::order_status,
                $7,
                $8,
                $9,
                $10
            )
            RETURNING
                id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                status::text AS status,
                quantity,
                price,
                created_at,
                updated_at
            "#,
        )
        .bind(order.id())
        .bind(order.user_id())
        .bind(order.wallet_id())
        .bind(order.market_symbol())
        .bind(order.side().as_str())
        .bind(order.status().as_str())
        .bind(order.quantity())
        .bind(order.price())
        .bind(order.created_at())
        .bind(order.updated_at())
        .fetch_one(&self.pool)
        .await?;

        row.try_into()
    }

    async fn find_by_id(&self, order_id: Uuid) -> Result<Option<Order>, AppError> {
        let row = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT
                id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                status::text AS status,
                quantity,
                price,
                created_at,
                updated_at
            FROM orders
            WHERE id = $1
            "#,
        )
        .bind(order_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(TryInto::try_into).transpose()
    }

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, AppError> {
        let rows = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT
                id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                status::text AS status,
                quantity,
                price,
                created_at,
                updated_at
            FROM orders
            WHERE user_id = $1
            ORDER BY created_at DESC, id DESC
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

    async fn update(&self, order: &Order) -> Result<Order, AppError> {
        let row = sqlx::query_as::<_, OrderRow>(
            r#"
            UPDATE orders
            SET
                status = $2::order_status,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                user_id,
                wallet_id,
                market_symbol,
                side::text AS side,
                status::text AS status,
                quantity,
                price,
                created_at,
                updated_at
            "#,
        )
        .bind(order.id())
        .bind(order.status().as_str())
        .bind(order.updated_at())
        .fetch_optional(&self.pool)
        .await?;

        let row = row.ok_or(AppError::OrderNotFound)?;

        row.try_into()
    }
}
