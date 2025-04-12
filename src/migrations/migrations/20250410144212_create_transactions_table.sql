-- Add migration script here
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    merchant_reference VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    customer_id VARCHAR NOT NULL,
    basket_id VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    timestamp BIGINT NOT NULL
);
