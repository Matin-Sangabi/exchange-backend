-- Add up migration script here
CREATE TABLE wallet_assets (
    id UUID PRIMARY KEY,
    wallet_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    balance NUMERIC(36, 18) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_wallet_assets_wallet
        FOREIGN KEY (wallet_id)
        REFERENCES wallets(id)
        ON DELETE RESTRICT,

    CONSTRAINT wallet_assets_balance_non_negative
        CHECK (balance >= 0),

    CONSTRAINT wallet_assets_symbol_not_empty
        CHECK (LENGTH(TRIM(symbol)) > 0),

    CONSTRAINT wallet_assets_wallet_symbol_unique
        UNIQUE (wallet_id, symbol)
);

CREATE INDEX idx_wallet_assets_wallet_id
    ON wallet_assets(wallet_id);

CREATE INDEX idx_wallet_assets_symbol
    ON wallet_assets(symbol);