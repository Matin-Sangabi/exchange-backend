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
        let order = self.lock_order(tx, order_id).await?;

        self.ensure_order_pending(&order)?;

        let wallet = self.lock_wallet(tx, order.wallet_id).await?;

        let market = self.load_market(tx, &order.market_symbol).await?;

        let order_side = OrderSide::from_str(&order.side)?;
        let total_value = self.calculate_total_value(&order)?;

        match order_side {
            OrderSide::Buy => {
                self.execute_buy(tx, &order, &wallet, &market, total_value)
                    .await?;
            }
            OrderSide::Sell => {
                self.execute_sell(tx, &order, &wallet, &market, total_value)
                    .await?;
            }
        }

        println!("Order: {order:#?}");
        println!("Wallet: {wallet:#?}");
        println!("Market: {market:#?}");
        let updated_order = self.update_order(tx, order_id).await?;

        let domain_order = Self::map_order(updated_order)?;

        Ok(domain_order)
    }

    fn map_order(row: OrderExecutionRow) -> Result<Order, AppError> {
        let side = OrderSide::from_str(&row.side)?;
        let status = OrderStatus::from_str(&row.status)?;

        Ok(Order::restore(
            row.id,
            row.user_id,
            row.wallet_id,
            row.market_symbol,
            side,
            status,
            row.quantity,
            row.price,
            row.created_at,
            row.updated_at,
        ))
    }

    async fn lock_order(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
    ) -> Result<OrderExecutionRow, AppError> {
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

        Ok(order)
    }

    fn ensure_order_pending(&self, order: &OrderExecutionRow) -> Result<(), AppError> {
        let status = OrderStatus::from_str(&order.status)?;

        if status != OrderStatus::Pending {
            return Err(AppError::OrderAlreadyProcessed);
        }

        Ok(())
    }

    async fn lock_wallet(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
    ) -> Result<WalletExecutionRow, AppError> {
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
        .bind(wallet_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::WalletNotFound)?;

        Ok(wallet)
    }

    async fn load_market(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        market_symbol: &str,
    ) -> Result<MarketExecutionRow, AppError> {
        let market = sqlx::query_as::<_, MarketExecutionRow>(
            r#"
                SELECT
                base_asset,
                quote_asset
                FROM markets
                WHERE symbol = $1
            "#,
        )
        .bind(market_symbol)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::MarketNotFound)?;

        Ok(market)
    }

    fn calculate_total_value(&self, order: &OrderExecutionRow) -> Result<Decimal, AppError> {
        order
            .price
            .checked_mul(order.quantity)
            .ok_or(AppError::InvalidOrderPrice)
    }

    async fn execute_buy(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        order: &OrderExecutionRow,
        wallet: &WalletExecutionRow,
        market: &MarketExecutionRow,
        total_value: Decimal,
    ) -> Result<(), AppError> {
        if wallet.cash_balance < total_value {
            return Err(AppError::InsufficientBalance);
        }
        let new_cash_balance = wallet
            .cash_balance
            .checked_sub(total_value)
            .ok_or(AppError::InsufficientBalance)?;

        self.update_cash_balance(tx, wallet.id, new_cash_balance)
            .await?;

        self.create_cash_transaction(
            tx,
            wallet.id,
            "withdraw",
            total_value,
            wallet.cash_balance,
            new_cash_balance,
            order.id,
            format!("Buy order execution : {}", order.market_symbol),
        )
        .await?;

        let asset = self
            .ensure_and_lock_asset(tx, wallet.id, &market.base_asset)
            .await?;

        let new_asset_balance = asset
            .balance
            .checked_add(order.quantity)
            .ok_or(AppError::InvalidOrderQuantity)?;

        self.update_asset_balance(tx, asset.id, new_asset_balance)
            .await?;

        self.create_asset_transaction(
            tx,
            wallet.id,
            asset.id,
            &market.base_asset,
            "deposit",
            order.quantity,
            asset.balance,
            new_asset_balance,
            order.id,
            format!("Buy order execution : {}", order.market_symbol),
        )
        .await?;

        Ok(())
    }

    async fn execute_sell(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        order: &OrderExecutionRow,
        wallet: &WalletExecutionRow,
        market: &MarketExecutionRow,
        total_value: Decimal,
    ) -> Result<(), AppError> {
        let asset = self
            .lock_existing_asset(tx, wallet.id, &market.base_asset)
            .await?;

        if asset.balance < order.quantity {
            return Err(AppError::InsufficientAssetBalance);
        }

        let new_asset_balance = asset
            .balance
            .checked_sub(order.quantity)
            .ok_or(AppError::InsufficientAssetBalance)?;

        self.update_asset_balance(tx, asset.id, new_asset_balance)
            .await?;

        self.create_asset_transaction(
            tx,
            wallet.id,
            asset.id,
            &market.base_asset,
            "withdraw",
            order.quantity,
            asset.balance,
            new_asset_balance,
            order.id,
            format!("Sell order execution : {}", order.market_symbol),
        )
        .await?;

        let new_cash_balance = wallet
            .cash_balance
            .checked_add(total_value)
            .ok_or(AppError::BalanceOverflow)?;

        self.update_cash_balance(tx, wallet.id, new_cash_balance)
            .await?;

        self.create_cash_transaction(
            tx,
            wallet.id,
            "deposit",
            total_value,
            wallet.cash_balance,
            new_cash_balance,
            order.id,
            format!("Sell order execution : {}", order.market_symbol),
        )
        .await?;

        Ok(())
    }

    async fn update_cash_balance(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
        new_balance: Decimal,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
                    UPDATE wallets
                    SET 
                        cash_balance = $2,
                        updated_at = NOW()
                    WHERE id = $1
                    "#,
        )
        .bind(wallet_id)
        .bind(new_balance)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_cash_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
        transaction_type: &str,
        amount: Decimal,
        balance_before: Decimal,
        balance_after: Decimal,
        reference_id: Uuid,
        description: String,
    ) -> Result<(), AppError> {
        sqlx::query(
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
                created_at
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
                NOW()
            )
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(wallet_id)
        .bind(transaction_type)
        .bind(amount)
        .bind(balance_before)
        .bind(balance_after)
        .bind(reference_id)
        .bind(description)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn ensure_and_lock_asset(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<WalletAssetExecutionRow, AppError> {
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
                ON CONFLICT (wallet_id, symbol) 
                DO NOTHING
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(wallet_id)
        .bind(symbol)
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
        .bind(wallet_id)
        .bind(symbol)
        .fetch_one(&mut **tx)
        .await?;

        Ok(asset)
    }

    async fn update_asset_balance(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        asset_id: Uuid,
        new_balance: Decimal,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
                UPDATE wallet_assets
                SET
                    balance = $2,
                    updated_at = NOW()
                WHERE id = $1
            "#,
        )
        .bind(asset_id)
        .bind(new_balance)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_asset_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
        wallet_asset_id: Uuid,
        symbol: &str,
        transaction_type: &str,
        amount: Decimal,
        balance_before: Decimal,
        balance_after: Decimal,
        reference_id: Uuid,
        description: String,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
        INSERT INTO wallet_asset_transactions (
            id,
            wallet_id,
            wallet_asset_id,
            symbol,
            transaction_type,
            amount,
            balance_before,
            balance_after,
            reference_id,
            description,
            created_at
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5::wallet_asset_transaction_type,
            $6,
            $7,
            $8,
            $9,
            $10,
            NOW()
        )
        "#,
        )
        .bind(Uuid::new_v4())
        .bind(wallet_id)
        .bind(wallet_asset_id)
        .bind(symbol)
        .bind(transaction_type)
        .bind(amount)
        .bind(balance_before)
        .bind(balance_after)
        .bind(reference_id)
        .bind(description)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn lock_existing_asset(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet_id: Uuid,
        symbol: &str,
    ) -> Result<WalletAssetExecutionRow, AppError> {
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
        .bind(wallet_id)
        .bind(symbol)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::WalletAssetNotFound)?;
        Ok(asset)
    }

    async fn update_order(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
    ) -> Result<OrderExecutionRow, AppError> {
        let updated_order: OrderExecutionRow = sqlx::query_as::<_, OrderExecutionRow>(
            r#"
                UPDATE orders
                SET
                    status = 'filled'::order_status,
                    updated_at = NOW()
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
        .bind(order_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok(updated_order)
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
