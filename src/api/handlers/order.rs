use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    api::{
        AppState, dto::{
            CreateOrderRequest, OrderListQuery, OrderListResponse, OrderResponse, OrderStatsResponse, order::PaginationMeta,
        },
    }, domain::orders::OrderSide, errors::AppError, services::order_service::build_filter,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let side = OrderSide::from_str(&payload.side)?;

    let quantity =
        Decimal::from_str(&payload.quantity).map_err(|_| AppError::InvalidOrderQuantityFormat)?;

    let order = state
        .order_service
        .create_order(
            payload.user_id,
            payload.wallet_id,
            payload.market_symbol,
            side,
            quantity,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(order.into())))
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = state.order_service.get_order(order_id).await?;

    Ok(Json(order.into()))
}

pub async fn get_user_orders(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<OrderListQuery>,
) -> Result<Json<OrderListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let filter = build_filter(query.status, query.side, query.market)?;

    let result = state
        .order_service
        .get_user_order(user_id, page, per_page, filter)
        .await?;

    let items = result.items.into_iter().map(OrderResponse::from).collect();

    Ok(Json(OrderListResponse {
        items,
        pagination: PaginationMeta {
            page: result.page,
            per_page: result.per_page,
            total_items: result.total_items,
            total_pages: result.total_pages,
            has_next: result.has_next,
            has_previous: result.has_previous,
        },
    }))
}
pub async fn execute_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = state.order_execute_service.execute_order(order_id).await?;
    Ok(Json(order.into()))
}

pub async fn cancel_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = state.order_execute_service.cancel_order(order_id).await?;
    Ok(Json(order.into()))
}

pub async fn get_user_order_stats(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<OrderStatsResponse>, AppError> {
    let stats = state.order_service.get_user_stats(user_id).await?;

    Ok(Json(stats.into()))
}
