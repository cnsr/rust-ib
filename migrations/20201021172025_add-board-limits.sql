-- Add migration script here
ALTER TABLE boards 
    ADD max_posts INTEGER DEFAULT 50;
ALTER TABLE posts
    ADD is_locked BOOLEAN DEFAULT FALSE;