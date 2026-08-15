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
        AppState,
        dto::{CreateOrderRequest, OrderListQuery, OrderListResponse, OrderResponse},
    },
    domain::orders::OrderSide,
    errors::AppError,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let side = OrderSide::from_str(&payload.side)?;

    println!("we are here");

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

    let orders = state
        .order_service
        .get_user_order(user_id, page, per_page)
        .await?;

    let items = orders.into_iter().map(OrderResponse::from).collect();

    Ok(Json(OrderListResponse { items }))
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
