set dotenv-load := true

db-init:
    sqlx database create

db-migrate:
    sqlx migrate run

db-create-migration name:
    sqlx migrate add {{name}}

serve: init
    dx serve

# Alle Tests im Workspace ausführen (web + api)
test:
    cargo test --workspace

# Nur die Web-Tests ausführen (u.a. i18n-Locale-Parität de/en)
test-web:
    cargo test -p web

# Nur den i18n-Test ausführen (de.ftl und en.ftl definieren dieselben Keys)
test-i18n:
    cargo test -p web i18n

# Nur die API-Tests ausführen
test-api:
    cargo test -p api

# Version aus `git describe` in Cargo.toml schreiben (auch via pre-commit Hook)
sync-version:
    python3 scripts/sync_version.py

# Neuen Tag erstellen: Cargo.toml syncen, committen und taggen
tag version:
    just sync-version
    cargo check -p web
    git add web/Cargo.toml api/Cargo.toml Cargo.lock || true
    git commit -m "chore(release): {{version}}" || true
    git tag {{version}}

# Typ-Check des Frontends (Host + Server-Feature)
check:
    cargo check -p web
    cargo check -p web --features server

init:
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs

docker-init:
    docker buildx create --name container-builder --driver docker-container --bootstrap --use

docker-login:
    docker login

docker:
    docker buildx build --platform linux/amd64 -t pingoin/rusty_mc_skins:latest .