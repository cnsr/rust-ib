-- Add migration script here
ALTER TABLE posts
    ADD replies integer[];