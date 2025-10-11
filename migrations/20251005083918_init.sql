-- Add migration script here
CREATE TABLE textures (
    id                  TEXT PRIMARY KEY NOT NULL,
    skin_name           TEXT NOT NULL UNIQUE,
    texture_type        TEXT NOT NULL,
    image_data          BLOB NOT NULL
);

CREATE TABLE users (
    id                TEXT PRIMARY KEY NOT NULL,
    username          TEXT NOT NULL UNIQUE,
    password_hash     TEXT NOT NULL,
    selected_skin_id  TEXT,
    selected_cape_id  TEXT,
    FOREIGN KEY (selected_skin_id) REFERENCES textures(id),
    FOREIGN KEY (selected_cape_id) REFERENCES textures(id)
);
