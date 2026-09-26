-- Texturen mit dem hochladenden Benutzer verknuepfen
ALTER TABLE textures
ADD COLUMN owner_id TEXT REFERENCES users(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_textures_owner_id ON textures(owner_id);

-- Gruppenspezifische Obergrenze fuer Texturen pro Nutzer
-- (wirksam ist jeweils das Maximum ueber alle Gruppen eines Nutzers)
ALTER TABLE groups
ADD COLUMN max_textures INTEGER NOT NULL DEFAULT 5;

UPDATE groups
SET
    max_textures = 1000
WHERE
    id = 'adm';

UPDATE groups
SET
    max_textures = 25
WHERE
    id = 'crtr';

UPDATE groups
SET
    max_textures = 0
WHERE
    id = 'usr';
