use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    api::{
        AppState,
        dto::{WalletAssetAmountRequest, WalletAssetListResponse, WalletAssetResponse},
    },
    errors::AppError,
};

fn parse_asset_amount(amount: &str) -> Result<Decimal, AppError> {
    Decimal::from_str(amount).map_err(|_| AppError::InvalidAmountFormat)
}

pub async fn get_wallet_assets(
    State(state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
) -> Result<Json<WalletAssetListResponse>, AppError> {
    let assets = state.wallet_asset_service.get_assets(wallet_id).await?;

    let items: Vec<WalletAssetResponse> =
        assets.into_iter().map(WalletAssetResponse::from).collect();

    Ok(Json(WalletAssetListResponse { items }))
}

pub async fn get_wallet_asset(
    State(state): State<AppState>,
    Path((wallet_id, symbol)): Path<(Uuid, String)>,
) -> Result<Json<WalletAssetResponse>, AppError> {
    let asset = state
        .wallet_asset_service
        .get_asset(wallet_id, symbol)
        .await?;

    Ok(Json(asset.into()))
}

pub async fn deposit_wallet_asset(
    State(state): State<AppState>,
    Path((wallet_id, symbol)): Path<(Uuid, String)>,
    Json(payload): Json<WalletAssetAmountRequest>,
) -> Result<Json<WalletAssetResponse>, AppError> {
    let amount = parse_asset_amount(&payload.amount)?;

    let asset = state
        .wallet_asset_service
        .deposit(wallet_id, symbol, amount)
        .await?;

    Ok(Json(asset.into()))
}

pub async fn withdraw_wallet_asset(
    State(state): State<AppState>,
    Path((wallet_id, symbol)): Path<(Uuid, String)>,
    Json(payload): Json<WalletAssetAmountRequest>,
) -> Result<Json<WalletAssetResponse>, AppError> {
    let amount = parse_asset_amount(&payload.amount)?;

    let asset = state
        .wallet_asset_service
        .withdraw(wallet_id, symbol, amount)
        .await?;

    Ok(Json(asset.into()))
}
