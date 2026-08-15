-- Add up migration script here
CREATE TYPE wallet_asset_transaction_type AS ENUM ('deposit', 'withdraw');
CREATE TABLE wallet_asset_transactions (
  id UUID PRIMARY KEY,
  wallet_id UUID NOT NULL,
  wallet_asset_id UUID NOT NULL,
  symbol VARCHAR(20) NOT NULL,
  transaction_type wallet_asset_transaction_type NOT NULL,
  amount NUMERIC(36, 18) NOT NULL,
  balance_before NUMERIC(36, 18) NOT NULL,
  balance_after NUMERIC(36, 18) NOT NULL,
  reference_id UUID NOT NULL,
  description VARCHAR(255),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_wallet_asset_transactions_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE RESTRICT,
  CONSTRAINT fk_wallet_asset_transactions_asset FOREIGN KEY (wallet_asset_id) REFERENCES wallet_assets(id) ON DELETE RESTRICT,
  CONSTRAINT wallet_asset_transactions_amount_positive CHECK (amount > 0),
  CONSTRAINT wallet_asset_transactions_balance_before_non_negative CHECK (balance_before >= 0),
  CONSTRAINT wallet_asset_transactions_balance_after_non_negative CHECK (balance_after >= 0),
  CONSTRAINT wallet_asset_transactions_reference_unique UNIQUE (reference_id)
);