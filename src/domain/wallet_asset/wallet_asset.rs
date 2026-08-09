use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct WalletAsset {
    id: Uuid,
    wallet_id: Uuid,
    symbol: String,
    balance: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl WalletAsset {
    pub fn new(wallet_id: Uuid, symbol: impl Into<String>) -> Result<Self, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }
        let symbol = normalize_symbol(symbol.into())?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            wallet_id,
            symbol,
            balance: Decimal::ZERO,
            created_at: now,
            updated_at: now,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restore(
        id: Uuid,
        wallet_id: Uuid,
        symbol: String,
        balance: Decimal,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            wallet_id,
            symbol,
            balance,
            created_at,
            updated_at,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) -> Result<(), AppError> {
        if amount <= Decimal::ZERO {
            return Err(AppError::InvalidAmount);
        }

        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(AppError::AssetBalanceOverflow)?;

        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), AppError> {
        if amount <= Decimal::ZERO {
            return Err(AppError::InvalidAmount);
        }

        if self.balance < amount {
            return Err(AppError::InsufficientAssetBalance);
        }

        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(AppError::AssetBalanceOverflow)?;

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn wallet_id(&self) -> Uuid {
        self.wallet_id
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn balance(&self) -> Decimal {
        self.balance
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub fn normalize_symbol(symbol: impl AsRef<str>) -> Result<String, AppError> {
    let symbol = symbol.as_ref().trim().to_uppercase();

    if symbol.is_empty() || symbol.len() > 20 {
        return Err(AppError::InvalidAssetSymbol);
    }
    if !symbol
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        return Err(AppError::InvalidAssetSymbol);
    }

    Ok(symbol)
}
