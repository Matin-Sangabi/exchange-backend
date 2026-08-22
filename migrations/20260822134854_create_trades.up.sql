-- Add up migration script here
CREATE TABLE trades (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL,
    user_id UUID NOT NULL,
    wallet_id UUID NOT NULL,

    market_symbol VARCHAR(41) NOT NULL,
    side order_side NOT NULL,

    price NUMERIC(36, 18) NOT NULL,
    quantity NUMERIC(36, 18) NOT NULL,
    quote_amount NUMERIC(36, 18) NOT NULL,

    fee_amount NUMERIC(36, 18) NOT NULL,
    fee_percent NUMERIC(10, 6) NOT NULL,

    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_trades_order
        FOREIGN KEY (order_id)
        REFERENCES orders(id)
        ON DELETE RESTRICT,

    CONSTRAINT fk_trades_wallet
        FOREIGN KEY (wallet_id)
        REFERENCES wallets(id)
        ON DELETE RESTRICT,

    CONSTRAINT trades_price_positive
        CHECK (price > 0),

    CONSTRAINT trades_quantity_positive
        CHECK (quantity > 0),

    CONSTRAINT trades_quote_amount_positive
        CHECK (quote_amount > 0),

    CONSTRAINT trades_fee_non_negative
        CHECK (fee_amount >= 0),

    CONSTRAINT trades_fee_percent_non_negative
        CHECK (fee_percent >= 0),

    CONSTRAINT trades_order_unique
        UNIQUE (order_id)
);

CREATE INDEX idx_trades_market_symbol
    ON trades(market_symbol);

CREATE INDEX idx_trades_user_id
    ON trades(user_id);

CREATE INDEX idx_trades_wallet_id
    ON trades(wallet_id);

CREATE INDEX idx_trades_executed_at
    ON trades(executed_at DESC);

CREATE INDEX idx_trades_market_executed_at
    ON trades(market_symbol, executed_at DESC);