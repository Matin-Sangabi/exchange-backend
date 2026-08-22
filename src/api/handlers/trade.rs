use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{TradeListQuery, TradeListResponse, TradeResponse},
        state::AppState,
    },
    errors::AppError,
};

pub async fn get_trade(
    State(state): State<AppState>,
    Path(trade_id): Path<Uuid>,
) -> Result<Json<TradeResponse>, AppError> {
    let trade = state.trade_service.get_trade(trade_id).await?;

    Ok(Json(trade.into()))
}

pub async fn get_user_trades(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<TradeListQuery>,
) -> Result<Json<TradeListResponse>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let trades = state
        .trade_service
        .get_user_trades(user_id, page, per_page)
        .await?;

    Ok(Json(TradeListResponse {
        page,
        per_page,
        items: trades.into_iter().map(TradeResponse::from).collect(),
    }))
}

pub async fn get_market_trades(
    State(state): State<AppState>,
    Path(market_symbol): Path<String>,
    Query(query): Query<TradeListQuery>,
) -> Result<Json<TradeListResponse>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let trades = state
        .trade_service
        .get_market_trades(market_symbol, page, per_page)
        .await?;

    Ok(Json(TradeListResponse {
        page,
        per_page,
        items: trades.into_iter().map(TradeResponse::from).collect(),
    }))
}
