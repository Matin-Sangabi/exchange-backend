use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction, prelude::FromRow};
use uuid::Uuid;

use crate::{
    domain::orders::{Order, OrderSide, OrderStatus},
    errors::AppError,
    repositories::order_execution::order_execution_repository::OrderExecutionRepository,
};

#[derive(Debug, FromRow)]
struct OrderExecutionRow {
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

#[derive(Debug, FromRow)]
struct WalletExecutionRow {
    id: Uuid,
    cash_balance: Decimal,
}

#[derive(Debug, FromRow)]
struct MarketExecutionRow {
    base_asset: String,
    quote_asset: String,
}

#[derive(Debug, FromRow)]
struct WalletAssetExecutionRow {
    id: Uuid,
    wallet_id: Uuid,
    symbol: String,
    balance: Decimal,
}

#[derive(Debug, Clone)]
pub struct PostgresOrderExecutionRepository {
    pool: PgPool,
}

impl PostgresOrderExecutionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn execute_in_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
    ) -> Result<Order, AppError> {
        // todo
        let order = sqlx::query_as::<_, OrderExecutionRow>(
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
        FOR UPDATE
        "#,
        )
        .bind(order_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::OrderNotFound)?;

        let order_status = OrderStatus::from_str(&order.status)?;

        if order_status != OrderStatus::Pending {
            return Err(AppError::OrderAlreadyProcessed);
        }

        let wallet = sqlx::query_as::<_, WalletExecutionRow>(
            r#"
               SELECT
                id,
                cash_balance
                FROM wallets
                WHERE id = $1
                FOR UPDATE
           "#,
        )
        .bind(order.wallet_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::WalletNotFound)?;

        let market = sqlx::query_as::<_, MarketExecutionRow>(
            r#"
                SELECT
                base_asset,
                quote_asset
                FROM markets
                WHERE symbol = $1
            "#,
        )
        .bind(&order.market_symbol)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::MarketNotFound)?;

        let order_side = OrderSide::from_str(&order.side)?;
        let total_value = order
            .price
            .checked_mul(order.quantity)
            .ok_or(AppError::InvalidOrderPrice)?;

        match order_side {
            OrderSide::Buy => {
                if wallet.cash_balance < total_value {
                    return Err(AppError::InsufficientAssetBalance);
                }

                let new_cash_balance = wallet
                    .cash_balance
                    .checked_sub(total_value)
                    .ok_or(AppError::InsufficientAssetBalance)?;

                sqlx::query(
                    r#"
                      UPDATE wallets
                      SET 
                        cash_balance = $2,
                        updated_at = NOW()
                      WHERE id = $1
                    "#,
                )
                .bind(wallet.id)
                .bind(new_cash_balance)
                .execute(&mut **tx)
                .await?;

                // create into wallet
                sqlx::query(
                    r#"
                          INSERT INTO wallet_assets (
                            id,
                            wallet_id,
                            symbol,
                            balance,
                            created_at,
                            updated_at
                        )
                        VALUES ($1, $2, $3, 0, NOW(), NOW())
                        ON CONFLICT (wallet_id, symbol) DO NOTHING
                      "#,
                )
                .bind(Uuid::new_v4())
                .bind(wallet.id)
                .bind(&market.base_asset)
                .execute(&mut **tx)
                .await?;

                let asset = sqlx::query_as::<_, WalletAssetExecutionRow>(
                    r#"
                      SELECT
                      id,
                      wallet_id,
                      symbol,
                      balance
                      FROM wallet_assets
                      WHERE wallet_id = $1
                      AND symbol = $2
                      FOR UPDATE
                      "#,
                )
                .bind(wallet.id)
                .bind(&market.base_asset)
                .fetch_one(&mut **tx)
                .await?;

                let new_asset_balance = asset
                    .balance
                    .checked_add(order.quantity)
                    .ok_or(AppError::InvalidOrderQuantity)?;

                sqlx::query(
                    r#"
                        UPDATE wallet_assets
                        SET
                        balance = $2,
                        updated_at = NOW()
                        WHERE id = $1
                        "#,
                )
                .bind(asset.id)
                .bind(new_asset_balance)
                .execute(&mut **tx)
                .await?;
            }
            OrderSide::Sell => {}
        }

        println!("Order: {order:#?}");
        println!("Wallet: {wallet:#?}");
        println!("Market: {market:#?}");
        todo!()
    }
}

#[async_trait]
impl OrderExecutionRepository for PostgresOrderExecutionRepository {
    async fn execute(&self, order_id: Uuid) -> Result<Order, AppError> {
        let mut tx = self.pool.begin().await?;

        let result = self.execute_in_transaction(&mut tx, order_id).await;

        match result {
            Ok(order) => {
                tx.commit().await?;
                Ok(order)
            }

            Err(error) => {
                tx.rollback().await?;
                Err(error)
            }
        }
    }
}
