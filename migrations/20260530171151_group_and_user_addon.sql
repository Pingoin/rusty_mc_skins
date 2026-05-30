-- Add migration script here
ALTER TABLE users
ADD COLUMN created TEXT DEFAULT CURRENT_TIMESTAMP;

UPDATE users
SET
    created = CURRENT_TIMESTAMP
WHERE
    created IS NULL;

ALTER TABLE groups
ADD COLUMN created TEXT DEFAULT CURRENT_TIMESTAMP;

UPDATE groups
SET
    created = CURRENT_TIMESTAMP
WHERE
    created IS NULL;

INSERT OR REPLACE INTO
    groups (id, group_name, permissions)
VALUES
    ('adm', 'Administrator', 3145728),
    ('usr', 'User', 2),
    ('crtr', 'Creator', 1);
