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
