-- Add migration script here
ALTER TABLE posts 
ADD CONSTRAINT oppost_id
    FOREIGN KEY (oppost_id)
    REFERENCES posts(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;