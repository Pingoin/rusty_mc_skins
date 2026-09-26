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

# Typ-Check des Frontends (Host + Server-Feature)
check:
    cargo check -p web
    cargo check -p web --features server

init:
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs
    cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs

# Release: Version aus Tag in web/api Cargo.toml schreiben, in HEAD
# amenden, annotierten Tag auf HEAD setzen und inkl. Tags pushen.
# Beispiel: just release v0.7.0
release tag:
    #!/usr/bin/env bash
    set -euo pipefail
    version="{{tag}}"
    version="${version#v}"
    sed -i "s/^version = \".*\"/version = \"$version\"/" web/Cargo.toml api/Cargo.toml
    cargo generate-lockfile
    git add web/Cargo.toml api/Cargo.toml Cargo.lock
    if ! git diff --cached --quiet; then
        SKIP_VERSION_SYNC=1 git commit --amend --no-edit --no-verify
    fi
    git tag -a -f "{{tag}}" -m "Release {{tag}}"
    git push --follow-tags

docker-init:
    docker buildx create --name container-builder --driver docker-container --bootstrap --use

docker-login:
    docker login

docker:
    docker buildx build --platform linux/amd64 -t pingoin/rusty_mc_skins:latest .