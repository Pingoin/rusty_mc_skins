-- Add migration script here
ALTER TABLE users
ADD COLUMN selected_elytra_id TEXT REFERENCES textures(id);