use axum::{
    Router,
    routing::{get, post},
};

use tower_http::trace::TraceLayer;

use crate::api::{
    handlers::{
        create_market, create_order, create_wallet, deposit, deposit_wallet_asset, execute_order,
        get_market, get_market_price, get_markets, get_order, get_price, get_user_orders,
        get_wallet_asset, get_wallet_assets, get_wallet_by_id, get_wallet_by_user_id,
        get_wallet_transaction, health, set_market_price, withdraw, withdraw_wallet_asset,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    let wallet_routes = Router::new()
        .route("/wallets", post(create_wallet))
        .route("/wallets/{wallet_id}", get(get_wallet_by_id))
        .route("/wallets/user/{user_id}", get(get_wallet_by_user_id))
        .route("/wallets/{wallet_id}/deposit", post(deposit))
        .route("/wallets/{wallet_id}/withdraw", post(withdraw))
        .route(
            "/wallets/{wallet_id}/transactions",
            get(get_wallet_transaction),
        )
        .route("/wallet/{wallet_id}/assets", get(get_wallet_assets))
        .route("/wallet/{wallet_id}/assets/{symbol}", get(get_wallet_asset))
        .route(
            "/wallet/{wallet_id}/assets/{symbol}/deposit",
            post(deposit_wallet_asset),
        )
        .route(
            "/wallet/{wallet_id}/assets/{symbol}/withdraw",
            post(withdraw_wallet_asset),
        );

    let market_routes = Router::new()
        .route("/markets", post(create_market).get(get_markets))
        .route("/markets/{symbol}", get(get_market))
        .route(
            "/markets/{symbol}/price",
            get(get_market_price).put(set_market_price),
        )
        .route("/markets/price/{symbol}", get(get_price));

    let order_routes = Router::new()
        .route("/orders", post(create_order))
        .route("/orders/{order_id}", get(get_order))
        .route("/orders/user/{user_id}", get(get_user_orders))
        .route("/orders/{order_id}/execute", post(execute_order));

    Router::new()
        .route("/health", get(health))
        .nest(
            "/api/v1",
            wallet_routes.merge(market_routes).merge(order_routes),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
