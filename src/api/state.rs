use crate::services::{
    MarketService, OrderExecutionService, OrderService, WalletAssetService, WalletService,
};

#[derive(Clone)]
pub struct AppState {
    pub wallet_service: WalletService,
    pub wallet_asset_service: WalletAssetService,
    pub market_service: MarketService,
    pub order_service: OrderService,
    pub order_execute_service: OrderExecutionService,
}

impl AppState {
    pub fn new(
        wallet_service: WalletService,
        wallet_asset_service: WalletAssetService,
        market_service: MarketService,
        order_service: OrderService,
        order_execute_service: OrderExecutionService,
    ) -> Self {
        Self {
            wallet_service,
            wallet_asset_service,
            market_service,
            order_service,
            order_execute_service,
        }
    }
}
