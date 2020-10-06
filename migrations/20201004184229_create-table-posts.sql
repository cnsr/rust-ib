-- Add migration script here
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    is_oppost BOOLEAN,
    subject VARCHAR(255) NULL,
    body TEXT NULL,
    created_at BIGSERIAL,
    board_id INTEGER NOT NULL,
    oppost_id INTEGER NULL
);