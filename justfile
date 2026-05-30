db-init:
    sqlx database create

db-migrate:
    sqlx migrate run

db-create-migration name:
    sqlx migrate add {{name}}

serve: init
    cd web && dx serve

init:
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs

docker-init:
    docker buildx create --name container-builder --driver docker-container --bootstrap --use

docker-login:
    docker login

docker:
    docker buildx build --platform linux/amd64 -t pingoin/rusty_mc_skins:latest .