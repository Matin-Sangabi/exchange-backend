use crate::services::WalletService;

#[derive(Clone)]
pub struct AppState {
    pub wallet_service: WalletService,
}

impl AppState {
    pub fn new(wallet_service: WalletService) -> Self {
        Self { wallet_service }
    }
}
