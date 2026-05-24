db-init:
    sqlx database create

db-migrate:
    sqlx migrate run

db-create-migration name:
    sqlx migrate add -r {{name}}

serve: init
    cd web && dx serve

init:
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs
