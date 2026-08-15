-- ...existing code...
DO $$ BEGIN IF NOT EXISTS (
  SELECT 1
  FROM pg_type
  WHERE typname = 'wallet_asset_transaction_type'
) THEN CREATE TYPE wallet_asset_transaction_type AS ENUM ('deposit', 'withdraw');
END IF;
END $$;
CREATE TABLE IF NOT EXISTS wallet_asset_transactions (
  id UUID PRIMARY KEY,
  wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
  wallet_asset_id UUID NOT NULL REFERENCES wallet_assets(id) ON DELETE CASCADE,
  symbol VARCHAR(64) NOT NULL,
  transaction_type wallet_asset_transaction_type NOT NULL,
  amount NUMERIC(36, 18) NOT NULL,
  balance_before NUMERIC(36, 18) NOT NULL,
  balance_after NUMERIC(36, 18) NOT NULL,
  reference_id UUID NOT NULL,
  description VARCHAR(255),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_wallet_asset_transactions_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE RESTRICT,
  CONSTRAINT fk_wallet_asset_transactions_wallet_asset FOREIGN KEY (wallet_asset_id) REFERENCES wallet_assets(id) ON DELETE RESTRICT,
  CONSTRAINT wallet_asset_transactions_reference_unique UNIQUE (reference_id),
  CONSTRAINT wallet_asset_transactions_balance_before_non_negative CHECK (balance_before >= 0),
  CONSTRAINT wallet_asset_transactions_balance_after_non_negative CHECK (balance_after >= 0)
);
CREATE INDEX IF NOT EXISTS idx_wallet_asset_transactions_wallet_id ON wallet_asset_transactions(wallet_id);
CREATE INDEX IF NOT EXISTS idx_wallet_asset_transactions_wallet_asset_id ON wallet_asset_transactions(wallet_asset_id);
CREATE INDEX IF NOT EXISTS idx_wallet_asset_transactions_created_at ON wallet_asset_transactions(created_at DESC);
-- ...existing code...