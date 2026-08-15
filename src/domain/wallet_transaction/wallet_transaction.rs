use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTransactionType {
    Deposit,
    WithDraw,
    Trade,
    Fee,
}

impl WalletTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deposit => "deposit",
            Self::WithDraw => "withdraw",
            Self::Fee => "fee",
            Self::Trade => "trade",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WalletTransaction {
    id: Uuid,
    wallet_id: Uuid,
    transaction_type: WalletTransactionType,
    amount: Decimal,
    balance_before: Decimal,
    balance_after: Decimal,
    reference_id: Uuid,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl WalletTransaction {
    pub fn new(
        wallet_id: Uuid,
        transaction_type: WalletTransactionType,
        amount: Decimal,
        balance_before: Decimal,
        balance_after: Decimal,
        reference_id: Uuid,
        description: Option<String>,
    ) -> Result<Self, AppError> {
        if wallet_id.is_nil() {
            return Err(AppError::InvalidWalletId);
        }
        if reference_id.is_nil() {
            return Err(AppError::InvalidReferenceId);
        }

        if amount <= Decimal::ZERO {
            return Err(AppError::InvalidAmount);
        }

        if balance_before < Decimal::ZERO || balance_after < Decimal::ZERO {
            return Err(AppError::InvalidBalance);
        }

        if let Some(description) = &description {
            if description.chars().count() > 255 {
                return Err(AppError::DescriptionTooLong);
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            wallet_id,
            transaction_type,
            amount,
            balance_before,
            balance_after,
            reference_id,
            description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restore(
        id: Uuid,
        wallet_id: Uuid,
        transaction_type: WalletTransactionType,
        amount: Decimal,
        balance_before: Decimal,
        balance_after: Decimal,
        reference_id: Uuid,
        description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            wallet_id,
            transaction_type,
            amount,
            balance_before,
            balance_after,
            reference_id,
            description,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn wallet_id(&self) -> Uuid {
        self.wallet_id
    }

    pub fn transaction_type(&self) -> WalletTransactionType {
        self.transaction_type
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn balance_before(&self) -> Decimal {
        self.balance_before
    }

    pub fn balance_after(&self) -> Decimal {
        self.balance_after
    }

    pub fn reference_id(&self) -> Uuid {
        self.reference_id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
