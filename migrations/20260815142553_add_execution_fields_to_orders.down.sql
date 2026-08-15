-- Add down migration script here
ALTER TABLE orders DROP CONSTRAINT IF EXISTS orders_fee_amount_non_negative;
ALTER TABLE orders DROP CONSTRAINT IF EXISTS orders_fee_percent_non_negative;
ALTER TABLE orders DROP COLUMN IF EXISTS executed_at;
ALTER TABLE orders DROP COLUMN IF EXISTS fee_amount;
ALTER TABLE orders DROP COLUMN IF EXISTS fee_percent;