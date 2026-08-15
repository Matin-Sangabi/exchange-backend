-- Add up migration script here
ALTER TABLE orders
ADD COLUMN fee_percent NUMERIC(10, 6),
ADD COLUMN fee_amount NUMERIC(36, 18),
ADD COLUMN executed_at TIMESTAMPTZ;

ALTER TABLE orders
ADD CONSTRAINT orders_fee_percent_non_negative
CHECK (
    fee_percent IS NULL
    OR fee_percent >= 0
);

ALTER TABLE orders
ADD CONSTRAINT orders_fee_amount_non_negative
CHECK (
    fee_amount IS NULL
    OR fee_amount >= 0
);