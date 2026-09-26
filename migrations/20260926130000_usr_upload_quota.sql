-- Mitglieder der usr-Gruppe duerfen ebenfalls Texturen hochladen (Quota: 5).
-- TEXTURE_EDIT ist Bit 0, daher wird es per OR ergaenzt, ohne andere Rechte zu entfernen.
UPDATE groups
SET
    max_textures = 5,
    permissions = permissions | 1
WHERE
    id = 'usr';
