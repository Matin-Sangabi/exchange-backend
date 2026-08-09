use std::sync::Arc;

use anyhow::{Context, Result};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{debug, info};

use crate::{
    api::{AppState, create_router},
    config::AppConfig,
    database::{check_database_health, create_database_pool, migrations::run_migrations},
    repositories::{
        market::{MarketRepository, PostgresMarketRepository},
        wallet::{PostgresWalletRepository, WalletRepository},
        wallet_asset::{PostgresWalletAssetRepository, WalletAssetRepository},
        wallet_transaction::{PostgresWalletTransactionRepository, WalletTransactionRepository},
    },
    services::{MarketService, WalletAssetService, WalletService},
};

pub struct Application {
    config: AppConfig,
    database_pool: PgPool,
    state: AppState,
}

impl Application {
    pub async fn build(config: AppConfig) -> Result<Self> {
        debug!("Building application");

        let database_pool = create_database_pool(&config.database).await?;

        check_database_health(&database_pool).await?;

        run_migrations(&database_pool).await?;

        let wallet_repository: Arc<dyn WalletRepository> =
            Arc::new(PostgresWalletRepository::new(database_pool.clone()));

        let transaction_repository: Arc<dyn WalletTransactionRepository> = Arc::new(
            PostgresWalletTransactionRepository::new(database_pool.clone()),
        );

        let market_repositories: Arc<dyn MarketRepository> =
            Arc::new(PostgresMarketRepository::new(database_pool.clone()));

        let wallet_asset_repository: Arc<dyn WalletAssetRepository> =
            Arc::new(PostgresWalletAssetRepository::new(database_pool.clone()));

        let wallet_service = WalletService::new(
            database_pool.clone(),
            wallet_repository.clone(),
            transaction_repository,
        );

        let wallet_asset_service = WalletAssetService::new(
            database_pool.clone(),
            wallet_repository,
            wallet_asset_repository,
        );

        let market_service = MarketService::new(market_repositories);

        let state = AppState::new(wallet_service, wallet_asset_service, market_service);

        info!(
            application = %config.app_name,
            environment = %config.env,
            address = %config.server.address(),
            database_max_connections =
                config.database.max_connections,
            "Application initialized"
        );

        Ok(Self {
            config,
            database_pool,
            state,
        })
    }

    pub async fn run(self) -> Result<()> {
        let address = self.config.server.address();

        let listener = TcpListener::bind(&address)
            .await
            .with_context(|| format!("Failed to bind TCP listener to {address}"))?;

        let router = create_router(self.state);

        info!(
            address = %address,
            "HTTP server listening"
        );

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("HTTP server failed")?;

        info!("HTTP server stopped");

        Ok(())
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn database_pool(&self) -> &PgPool {
        &self.database_pool
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(
                error = %error,
                "Failed to install Ctrl+C signal handler"
            );
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};

        match signal(SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }

            Err(error) => {
                tracing::error!(
                    error = %error,
                    "Failed to install terminate signal handler"
                );

                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C signal received");
        }

        _ = terminate => {
            tracing::info!("Terminate signal received");
        }
    }

    tracing::info!("Starting graceful shutdown");
}
