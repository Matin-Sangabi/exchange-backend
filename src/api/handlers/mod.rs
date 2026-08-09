pub mod health;
pub mod market;
pub mod wallet;
pub mod wallet_asset;

pub use health::health;
pub use wallet::{
    create_wallet, deposit, get_wallet_by_id, get_wallet_by_user_id, get_wallet_transaction,
    withdraw,
};

pub use wallet_asset::{
    deposit_wallet_asset, get_wallet_asset, get_wallet_assets, withdraw_wallet_asset,
};

pub use market::{create_market, get_market, get_market_price, get_markets, set_market_price};
