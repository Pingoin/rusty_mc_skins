-- Add migration script here
CREATE TABLE session_table (
                id VARCHAR(128) NOT NULL PRIMARY KEY,
                expires BIGINT NULL,
                session TEXT NOT NULL
            )
