use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    api::{
        dto::{CreateWalletRequest, WalletResponse},
        state::AppState,
    },
    errors::AppError,
};

pub async fn create_wallet(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> Result<(StatusCode, Json<WalletResponse>), AppError> {
    let initial_balance =
        Decimal::from_str(&payload.initial_balance).map_err(|_| AppError::InvalidBalanceFormat)?;

    let wallet = state
        .wallet_service
        .create_wallet(payload.user_id, initial_balance)
        .await?;
    Ok((StatusCode::CREATED, Json(wallet.into())))
}

pub async fn get_wallet_by_id(
    State(state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
) -> Result<Json<WalletResponse>, AppError> {
    if wallet_id.is_nil() {
        return Err(AppError::InvalidWalletId);
    }
    let wallet = state.wallet_service.get_wallet_by_id(wallet_id).await?;
    Ok(Json(wallet.into()))
}

pub async fn get_wallet_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<WalletResponse>, AppError> {
    if user_id.is_nil() {
        return Err(AppError::InvalidUserId);
    }

    let wallet = state.wallet_service.get_wallet_by_user_id(user_id).await?;
    Ok(Json(wallet.into()))
}
