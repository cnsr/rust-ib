-- Add migration script here
CREATE TABLE IF NOT EXISTS boards (
    board_id INTEGER PRIMARY KEY,
    short VARCHAR(5),
    long VARCHAR(255),
    description TEXT NULL,
    created_at BIGSERIAL,
    is_hidden BOOLEAN
);