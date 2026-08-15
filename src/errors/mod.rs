use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User id must not be nil")]
    InvalidUserId,

    #[error("Wallet id must not be nil")]
    InvalidWalletId,

    #[error("Initial balance must be zero or greater")]
    InvalidBalance,

    #[error("Balance must be a valid decimal number")]
    InvalidBalanceFormat,

    #[error("Wallet already exists")]
    WalletAlreadyExists,

    #[error("Wallet not found")]
    WalletNotFound,

    #[error("Database operation failed")]
    Database(#[from] sqlx::Error),

    #[error("Amount must be greater than zero")]
    InvalidAmount,

    #[error("Amount must be a valid decimal number")]
    InvalidAmountFormat,

    #[error("Wallet balance is insufficient")]
    InsufficientBalance,

    #[error("Wallet balance overflow")]
    BalanceOverflow,

    #[error("Invalid reference id")]
    InvalidReferenceId,

    #[error("Description must be less than 255 char ")]
    DescriptionTooLong,

    #[error("The transaction reference already exists")]
    DuplicateTransactionReference,

    #[error("Wallet transaction not found")]
    WalletTransactionNotFound,

    #[error("Asset symbol is invalid")]
    InvalidAssetSymbol,

    #[error("Asset balance is insufficient")]
    InsufficientAssetBalance,

    #[error("Wallet asset was not found")]
    WalletAssetNotFound,

    #[error("Wallet asset already exists")]
    WalletAssetAlreadyExists,

    #[error("Asset balance overflow")]
    AssetBalanceOverflow,

    #[error("Market symbol is invalid")]
    InvalidMarketSymbol,

    #[error("Base and quote assets must be different")]
    SameMarketAssets,

    #[error("Market price must be greater than zero")]
    InvalidMarketPrice,

    #[error("Market price has not been set")]
    MarketPriceNotSet,

    #[error("Market already exists")]
    MarketAlreadyExists,

    #[error("Market was not found")]
    MarketNotFound,

    #[error("Invalid market status")]
    InvalidMarketStatus,

    #[error("Order was not found")]
    OrderNotFound,

    #[error("Order quantity must be greater than zero")]
    InvalidOrderQuantity,

    #[error("Order price must be greater than zero")]
    InvalidOrderPrice,

    #[error("Invalid order side")]
    InvalidOrderSide,

    #[error("Invalid order status")]
    InvalidOrderStatus,

    #[error("Order has already been processed")]
    OrderAlreadyProcessed,

    #[error("Order quantity must be a valid decimal number")]
    InvalidOrderQuantityFormat,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
}

impl ErrorResponse {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::InvalidUserId => (
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "User id must be a valid non-nil UUID",
            ),

            AppError::InvalidWalletId => (
                StatusCode::BAD_REQUEST,
                "INVALID_WALLET_ID",
                "Wallet id must be a valid non-nil UUID",
            ),

            AppError::InvalidBalance => (
                StatusCode::BAD_REQUEST,
                "INVALID_BALANCE",
                "Initial balance must be zero or greater",
            ),

            AppError::InvalidBalanceFormat => (
                StatusCode::BAD_REQUEST,
                "INVALID_BALANCE_FORMAT",
                "Balance must be a valid decimal string",
            ),

            AppError::WalletAlreadyExists => (
                StatusCode::CONFLICT,
                "WALLET_ALREADY_EXISTS",
                "A wallet already exists for this user",
            ),

            AppError::WalletNotFound => (
                StatusCode::NOT_FOUND,
                "WALLET_NOT_FOUND",
                "Wallet was not found",
            ),

            AppError::InvalidAmount => (
                StatusCode::BAD_REQUEST,
                "INVALID_AMOUNT",
                "Amount must be greater than zero",
            ),
            AppError::InvalidAmountFormat => (
                StatusCode::BAD_REQUEST,
                "INVALID_AMOUNT_FORMAT",
                "Amount must be a valid decimal string",
            ),

            AppError::InsufficientBalance => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "INSUFFICIENT_BALANCE",
                "Wallet balance is insufficient",
            ),

            AppError::InvalidReferenceId => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "INVALID_REFERENCE_ID",
                "reference id  is insufficient",
            ),

            AppError::DescriptionTooLong => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "DESCRIPTION_TO_LONG",
                "description too long !",
            ),

            AppError::DuplicateTransactionReference => (
                StatusCode::CONFLICT,
                "DUPLICATE_TRANSACTION_REFERENCE",
                "This financial operation has already been processed",
            ),

            AppError::WalletTransactionNotFound => (
                StatusCode::NOT_FOUND,
                "WALLET_TRANSACTION_NOT_FOUND",
                "Wallet transaction was not found",
            ),

            AppError::InvalidAssetSymbol => (
                StatusCode::BAD_REQUEST,
                "INVALID_ASSET_SYMBOL",
                "Asset symbol must contain only valid uppercase characters",
            ),

            AppError::InsufficientAssetBalance => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "INSUFFICIENT_ASSET_BALANCE",
                "Wallet asset balance is insufficient",
            ),

            AppError::WalletAssetNotFound => (
                StatusCode::NOT_FOUND,
                "WALLET_ASSET_NOT_FOUND",
                "Wallet asset was not found",
            ),

            AppError::WalletAssetAlreadyExists => (
                StatusCode::CONFLICT,
                "WALLET_ASSET_ALREADY_EXISTS",
                "This asset already exists in the wallet",
            ),

            AppError::AssetBalanceOverflow => {
                error!("Wallet asset balance overflow");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ASSET_BALANCE_OVERFLOW",
                    "Wallet asset balance could not be updated",
                )
            }

            AppError::BalanceOverflow => {
                error!("Wallet balance overflow");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "BALANCE_OVERFLOW",
                    "Wallet balance could not be updated",
                )
            }

            AppError::InsufficientBalance => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "INSUFFICIENT_BALANCE",
                "Wallet does not have enough balance",
            ),

            AppError::InsufficientAssetBalance => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "INSUFFICIENT_ASSET_BALANCE",
                "Wallet does not have enough asset balance",
            ),

            AppError::InvalidMarketSymbol => (
                StatusCode::BAD_REQUEST,
                "INVALID_MARKET_SYMBOL",
                "Market assets must contain valid alphanumeric symbols",
            ),

            AppError::SameMarketAssets => (
                StatusCode::BAD_REQUEST,
                "SAME_MARKET_ASSETS",
                "Base and quote assets must be different",
            ),

            AppError::InvalidMarketPrice => (
                StatusCode::BAD_REQUEST,
                "INVALID_MARKET_PRICE",
                "Market price must be greater than zero",
            ),

            AppError::MarketPriceNotSet => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "MARKET_PRICE_NOT_SET",
                "The market price has not been set",
            ),

            AppError::InvalidOrderQuantityFormat => (
                StatusCode::BAD_REQUEST,
                "INVALID_ORDER_QUANTITY_FORMAT",
                "Order quantity must be a valid decimal string",
            ),

            AppError::MarketAlreadyExists => (
                StatusCode::CONFLICT,
                "MARKET_ALREADY_EXISTS",
                "This market already exists",
            ),

            AppError::MarketNotFound => (
                StatusCode::NOT_FOUND,
                "MARKET_NOT_FOUND",
                "Market was not found",
            ),

            AppError::InvalidMarketStatus => {
                error!("Invalid market status stored in database");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INVALID_MARKET_STATUS",
                    "An invalid market status was encountered",
                )
            }

            AppError::OrderNotFound => (
                StatusCode::NOT_FOUND,
                "ORDER_NOT_FOUND",
                "Order was not found",
            ),

            AppError::InvalidOrderQuantity => (
                StatusCode::BAD_REQUEST,
                "INVALID_ORDER_QUANTITY",
                "Order quantity must be greater than zero",
            ),

            AppError::InvalidOrderPrice => (
                StatusCode::BAD_REQUEST,
                "INVALID_ORDER_PRICE",
                "Order price must be greater than zero",
            ),

            AppError::InvalidOrderSide => (
                StatusCode::BAD_REQUEST,
                "INVALID_ORDER_SIDE",
                "Order side must be buy or sell",
            ),

            AppError::InvalidOrderStatus => {
                error!("Invalid order status stored in database");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INVALID_ORDER_STATUS",
                    "Invalid order status was encountered",
                )
            }

            AppError::OrderAlreadyProcessed => (
                StatusCode::CONFLICT,
                "ORDER_ALREADY_PROCESSED",
                "Order has already been processed",
            ),

            AppError::Database(database_error) => {
                error!(
                    error = ?database_error,
                    "Database operation failed"
                );

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "An internal database error occurred",
                )
            }
        };

        let body = ErrorResponse::new(code, message);

        (status, Json(body)).into_response()
    }
}
