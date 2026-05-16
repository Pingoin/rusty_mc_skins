-- Add migration script here
CREATE TABLE groups (
    id                  TEXT PRIMARY KEY NOT NULL,
    group_name           TEXT NOT NULL UNIQUE,
    permissions          INTEGER NOT NULL
);

CREATE TABLE groups_users(
    user_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    PRIMARY KEY (user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);
