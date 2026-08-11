use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;

use crate::{
    api::{
        AppState,
        dto::{
            CreateMarketRequest, MarketListResponse, MarketPriceResponse, MarketResponse,
            SetMarketPriceRequest,
        },
    },
    errors::AppError,
};

pub async fn create_market(
    State(state): State<AppState>,
    Json(payload): Json<CreateMarketRequest>,
) -> Result<(StatusCode, Json<MarketResponse>), AppError> {
    let market = state
        .market_service
        .create_market(payload.base_asset, payload.quote_asset)
        .await?;

    Ok((StatusCode::CREATED, Json(market.into())))
}

pub async fn get_markets(
    State(state): State<AppState>,
) -> Result<Json<MarketListResponse>, AppError> {
    let markets = state.market_service.get_markets().await?;

    let items = markets.into_iter().map(MarketResponse::from).collect();

    Ok(Json(MarketListResponse { items }))
}

pub async fn get_market(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<MarketResponse>, AppError> {
    let market = state.market_service.get_market(symbol).await?;

    Ok(Json(market.into()))
}

pub async fn set_market_price(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
    Json(payload): Json<SetMarketPriceRequest>,
) -> Result<Json<MarketResponse>, AppError> {
    let price = Decimal::from_str(&payload.price).map_err(|_| AppError::InvalidMarketPrice)?;

    let market = state.market_service.set_price(symbol, price).await?;

    Ok(Json(market.into()))
}

pub async fn get_market_price(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<MarketPriceResponse>, AppError> {
    let market = state.market_service.get_market(symbol).await?;

    let price = market.price()?;

    Ok(Json(MarketPriceResponse {
        symbol: market.symbol().to_owned(),
        price: price.to_string(),
        updated_at: market.updated_at(),
    }))
}

pub async fn get_price(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<Decimal>, AppError> {
    let price = state.market_service.get_price(symbol).await?;
    Ok(Json(price))
}
