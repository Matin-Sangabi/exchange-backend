use crate::services::{WalletAssetService, WalletService};

#[derive(Clone)]
pub struct AppState {
    pub wallet_service: WalletService,
    pub wallet_asset_service: WalletAssetService,
}

impl AppState {
    pub fn new(wallet_service: WalletService, wallet_asset_service: WalletAssetService) -> Self {
        Self {
            wallet_service,
            wallet_asset_service,
        }
    }
}
