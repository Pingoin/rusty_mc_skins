-- New permission TEXTURE_ADOPT (bit 2): take over, delete and overwrite
-- foreign and legacy textures. Previously this was implicitly possible via
-- USER_EDIT/GROUP_EDIT, so the administrator group loses nothing.
UPDATE groups
SET
    permissions = permissions | 4
WHERE
    id = 'adm';
