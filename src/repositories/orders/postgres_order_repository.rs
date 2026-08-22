use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::orders::{Order, OrderSide, OrderStatus},
    errors::AppError,
    repositories::orders::order_repository::OrderRepository,
    services::order_service::{OrderFilter, OrderStats},
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
    fee_percent: Option<Decimal>,
    fee_amount: Option<Decimal>,
    executed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct OrderStatsRow {
    total_orders: i64,
    pending_orders: i64,
    filled_orders: i64,
    cancelled_orders: i64,
    buy_orders: i64,
    sell_orders: i64,
    total_trade_volume: Decimal,
    total_fees: Decimal,
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
            value.fee_percent,
            value.fee_amount,
            value.executed_at,
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
                fee_percent,
                fee_amount,
                executed_at,
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
                fee_percent,
                fee_amount,
                executed_at,
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
        filter: &OrderFilter,
    ) -> Result<Vec<Order>, AppError> {
        let mut query = QueryBuilder::<Postgres>::new(
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
            fee_percent,
            fee_amount,
            executed_at,
            created_at,
            updated_at
        FROM orders
        WHERE user_id =
        "#,
        );

        query.push_bind(user_id);

        if let Some(status) = filter.status {
            query.push(" AND status = ");
            query.push_bind(status.as_str());
            query.push("::order_status");
        }

        if let Some(side) = filter.side {
            query.push(" AND side = ");
            query.push_bind(side.as_str());
            query.push("::order_side");
        }

        if let Some(market_symbol) = &filter.market_symbol {
            query.push(" AND market_symbol = ");
            query.push_bind(market_symbol);
        }

        query.push(" ORDER BY created_at DESC, id DESC");

        query.push(" LIMIT ");
        query.push_bind(limit);

        query.push(" OFFSET ");
        query.push_bind(offset);

        let rows = query
            .build_query_as::<OrderRow>()
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
                fee_percent,
                fee_amount,
                executed_at,
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

    async fn count_by_user_id(&self, user_id: Uuid, filter: &OrderFilter) -> Result<i64, AppError> {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
        SELECT COUNT(*)
        FROM orders
        WHERE user_id =
        "#,
        );

        query.push_bind(user_id);

        if let Some(status) = filter.status {
            query.push(" AND status = ");
            query.push_bind(status.as_str());
            query.push("::order_status");
        }

        if let Some(side) = filter.side {
            query.push(" AND side = ");
            query.push_bind(side.as_str());
            query.push("::order_side");
        }

        if let Some(market_symbol) = &filter.market_symbol {
            query.push(" AND market_symbol = ");
            query.push_bind(market_symbol);
        }

        let count = query
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn get_user_stats(&self, user_id: Uuid) -> Result<OrderStats, AppError> {
        let stats = sqlx::query_as::<_, OrderStatsRow>(
            r#"
        SELECT
            COUNT(*)::BIGINT AS total_orders,

            COUNT(*) FILTER (
                WHERE status = 'pending'::order_status
            )::BIGINT AS pending_orders,

            COUNT(*) FILTER (
                WHERE status = 'filled'::order_status
            )::BIGINT AS filled_orders,

            COUNT(*) FILTER (
                WHERE status = 'cancelled'::order_status
            )::BIGINT AS cancelled_orders,

            COUNT(*) FILTER (
                WHERE side = 'buy'::order_side
                AND status = 'filled'::order_status
            )::BIGINT AS buy_orders,

            COUNT(*) FILTER (
                WHERE side = 'sell'::order_side
                AND status = 'filled'::order_status
            )::BIGINT AS sell_orders,

            COALESCE(
                SUM(price * quantity) FILTER (
                    WHERE status = 'filled'::order_status
                ),
                0
            ) AS total_trade_volume,

            COALESCE(
                SUM(fee_amount) FILTER (
                    WHERE status = 'filled'::order_status
                ),
                0
            ) AS total_fees

        FROM orders
        WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(OrderStats {
            total_orders: stats.total_orders,
            pending_orders: stats.pending_orders,
            filled_orders: stats.filled_orders,
            buy_orders: stats.buy_orders,
            sell_orders: stats.sell_orders,
            total_trade_volume: stats.total_trade_volume,
            total_fees: stats.total_fees,
            cancelled_order: stats.cancelled_orders,
        })
    }
}
