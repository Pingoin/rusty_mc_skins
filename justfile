db-init:
    sqlx database create

db-migrate:
    sqlx migrate run

db-create-migration name:
    sqlx migrate add -r {{name}}

serve:
    cd web && dx serve
