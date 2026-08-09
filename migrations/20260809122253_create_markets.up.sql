-- Add up migration script here
CREATE TYPE market_status AS ENUM ('active', 'inactive');
CREATE TABLE markets (
  id UUID PRIMARY KEY,
  symbol VARCHAR(41) NOT NULL UNIQUE,
  base_asset VARCHAR(20) NOT NULL,
  quote_asset VARCHAR(20) NOT NULL,
  status market_status NOT NULL DEFAULT 'active',
  current_price NUMERIC(36, 18),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT markets_base_asset_not_empty CHECK (LENGTH(TRIM(base_asset)) > 0),
  CONSTRAINT markets_quote_asset_not_empty CHECK (LENGTH(TRIM(quote_asset)) > 0),
  CONSTRAINT markets_assets_must_be_different CHECK (base_asset <> quote_asset),
  CONSTRAINT markets_price_positive CHECK (
    current_price IS NULL
    OR current_price > 0
  ),
  CONSTRAINT markets_symbol_format CHECK (
    symbol = base_asset || '-' || quote_asset
  )
);
CREATE INDEX idx_markets_base_asset ON markets(base_asset);
CREATE INDEX idx_markets_quote_asset ON markets(quote_asset);
CREATE INDEX idx_markets_status ON markets(status);