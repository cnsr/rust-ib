-- Add migration script here
ALTER TABLE posts 
ADD CONSTRAINT board_id
    FOREIGN KEY (board_id)
    REFERENCES boards(board_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;