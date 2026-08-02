use axum::{
    Router,
    routing::{get, post},
};

use tower_http::trace::TraceLayer;

use crate::api::{
    handlers::{create_wallet, deposit, get_wallet_by_id, get_wallet_by_user_id, health, withdraw},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    let wallet_routes = Router::new()
        .route("/wallets", post(create_wallet))
        .route("/wallets/{wallet_id}", get(get_wallet_by_id))
        .route("/wallets/user/{user_id}", get(get_wallet_by_user_id))
        .route("/wallets/{wallet_id}/deposit", post(deposit))
        .route("/wallets/{wallet_id}/withdraw", post(withdraw));

    Router::new()
        .route("/health", get(health))
        .nest("/api/v1", wallet_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
