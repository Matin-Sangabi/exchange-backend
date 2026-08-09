-- Add up migration script here
CREATE TYPE wallet_transaction_type AS ENUM ('deposit', 'withdraw');
CREATE TABLE wallet_transactions (
  id UUID PRIMARY KEY,
  wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
  transaction_type wallet_transaction_type NOT NULL,
  amount NUMERIC(36, 18) NOT NULL,
  balance_before NUMERIC(36, 18) NOT NULL,
  balance_after NUMERIC(36, 18) NOT NULL,
  reference_id UUID NOT NULL,
  description VARCHAR(255),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_wallet_transactions_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE RESTRICT,
  CONSTRAINT wallet_transactions_balance_before_non_negative CHECK (balance_before >= 0),
  CONSTRAINT wallet_transactions_balance_after_non_negative CHECK (balance_after >= 0),
  CONSTRAINT wallet_transactions_reference_unique UNIQUE (reference_id)
);
CREATE INDEX idx_wallet_transactions_wallet_id ON wallet_transactions(wallet_id);
CREATE INDEX idx_wallet_transactions_created_at ON wallet_transactions(created_at DESC);
CREATE INDEX idx_wallet_transactions_wallet_created_at ON wallet_transactions(wallet_id, created_at DESC);