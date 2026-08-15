-- Add up migration script here
ALTER TYPE wallet_transaction_type
ADD VALUE IF NOT EXISTS 'trade';

ALTER TYPE wallet_transaction_type
ADD VALUE IF NOT EXISTS 'fee';