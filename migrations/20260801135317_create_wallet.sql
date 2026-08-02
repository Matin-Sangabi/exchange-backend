-- Add migration script here
CREATE TABLE wallets (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL UNIQUE,
  cash_balance NUMERIC(36, 18) NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT wallets_cash_balance_non_negative CHECK (cash_balance >= 0)
)