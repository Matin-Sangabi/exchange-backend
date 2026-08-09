use crate::services::{MarketService, WalletAssetService, WalletService};

#[derive(Clone)]
pub struct AppState {
    pub wallet_service: WalletService,
    pub wallet_asset_service: WalletAssetService,
    pub market_service: MarketService,
}

impl AppState {
    pub fn new(
        wallet_service: WalletService,
        wallet_asset_service: WalletAssetService,
        market_service: MarketService,
    ) -> Self {
        Self {
            wallet_service,
            wallet_asset_service,
            market_service,
        }
    }
}
