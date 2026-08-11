CREATE TYPE order_side AS ENUM (
    'buy',
    'sell'
);

CREATE TYPE order_status AS ENUM (
    'pending',
    'filled',
    'cancelled'
);

CREATE TABLE orders (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    wallet_id UUID NOT NULL,
    market_symbol VARCHAR(41) NOT NULL,

    side order_side NOT NULL,
    status order_status NOT NULL DEFAULT 'pending',

    quantity NUMERIC(36, 18) NOT NULL,
    price NUMERIC(36, 18) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_orders_wallet
        FOREIGN KEY (wallet_id)
        REFERENCES wallets(id)
        ON DELETE RESTRICT,

    CONSTRAINT orders_quantity_positive
        CHECK (quantity > 0),

    CONSTRAINT orders_price_positive
        CHECK (price > 0)
);

CREATE INDEX idx_orders_user_id
    ON orders(user_id);

CREATE INDEX idx_orders_wallet_id
    ON orders(wallet_id);

CREATE INDEX idx_orders_market_symbol
    ON orders(market_symbol);

CREATE INDEX idx_orders_status
    ON orders(status);

CREATE INDEX idx_orders_user_created_at
    ON orders(user_id, created_at DESC);