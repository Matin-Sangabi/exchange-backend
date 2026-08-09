use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use sqlx::query;
use uuid::Uuid;

use crate::{
    api::{
        dto::{
            CreateWalletRequest, TransactionListQuery, WalletAmountRequest, WalletResponse,
            WalletTransactionListResponse, WalletTransactionResponse,
        },
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

fn parse_amount(amount_str: &str) -> Result<Decimal, AppError> {
    Decimal::from_str(amount_str).map_err(|_| AppError::InvalidAmountFormat)
}

pub async fn deposit(
    State(state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Json(payload): Json<WalletAmountRequest>,
) -> Result<Json<WalletResponse>, AppError> {
    if wallet_id.is_nil() {
        return Err(AppError::InvalidWalletId);
    }

    let amount = parse_amount(&payload.amount)?;

    let wallet = state
        .wallet_service
        .deposit(wallet_id, amount, payload.reference_id, payload.description)
        .await?;
    Ok(Json(wallet.into()))
}

pub async fn withdraw(
    State(state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Json(payload): Json<WalletAmountRequest>,
) -> Result<Json<WalletResponse>, AppError> {
    if wallet_id.is_nil() {
        return Err(AppError::InvalidWalletId);
    }

    let amount = parse_amount(&payload.amount)?;

    let wallet = state
        .wallet_service
        .withdraw(wallet_id, amount, payload.reference_id, payload.description)
        .await?;
    Ok(Json(wallet.into()))
}

pub async fn get_wallet_transaction(
    State(state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Query(query): Query<TransactionListQuery>,
) -> Result<Json<WalletTransactionListResponse>, AppError> {
    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 100);

    let transactions = state
        .wallet_service
        .get_wallet_transactions(wallet_id, page, per_page)
        .await?;

    let items = transactions
        .into_iter()
        .map(WalletTransactionResponse::from)
        .collect();

    Ok(Json(WalletTransactionListResponse {
        page,
        per_page,
        items,
    }))
}
