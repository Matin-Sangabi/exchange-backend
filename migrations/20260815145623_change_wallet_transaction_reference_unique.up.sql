-- Add up migration script here
ALTER TABLE wallet_transactions DROP CONSTRAINT IF EXISTS wallet_transactions_reference_unique;
ALTER TABLE wallet_transactions
ADD CONSTRAINT wallet_transactions_reference_type_unique UNIQUE (reference_id, transaction_type);