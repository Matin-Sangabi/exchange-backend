-- Add down migration script here
DROP TABLE IF EXISTS orders;

DROP TYPE IF EXISTS order_status;
DROP TYPE IF EXISTS order_side;