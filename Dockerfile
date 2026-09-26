# syntax=docker/dockerfile:1
# Optimiert für BuildKit-Layer-Caching + GHA-Cache (type=gha,mode=max).
# Reihenfolge-Prinzip: selten ändernde Schritte (Toolchain, dx, Deps) zuerst,
# schnell ändernder App-Code zuletzt.

ARG RUST_VERSION=1-bookworm
ARG DX_VERSION=0.7
ARG SQLX_VERSION=0.9

FROM rust:${RUST_VERSION} AS builder
WORKDIR /app
ARG DX_VERSION
ARG SQLX_VERSION

# git wird für Checkout-Metadaten gebraucht; schlanke Layer, kein Reinstall bei Code-Änderungen
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# wasm-Ziel für den Dioxus-Frontend-Build (verhindert impliziten Download pro Build)
RUN rustup target add wasm32-unknown-unknown

# Toolchain-Helfer: eigener Layer, wird nur bei Versionswechsel neu gebaut
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall dioxus-cli@${DX_VERSION} sqlx-cli@${SQLX_VERSION} --root /.cargo -y
ENV PATH="/.cargo/bin:$PATH"

# --- Dependency-Layer: nur Manifeste kopieren, damit `cargo fetch` gecacht bleibt ---
# `cargo` braucht für die Workspace-Auflösung vorhandene Targets -> minimale
# Dummy-Quellen, die danach wieder entfernt werden (echter Code kommt per COPY).
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml ./api/Cargo.toml
COPY web/Cargo.toml ./web/Cargo.toml
RUN mkdir -p api/src web/src \
    && echo 'pub fn __placeholder() {}' > api/src/lib.rs \
    && echo 'fn main() {}' > web/src/main.rs \
    && cargo fetch --locked \
    && rm -rf api/src web/src

# --- Echter Build: erst ab hier invalidiert App-Code den Cache ---
COPY . .

# DaisyUI/Tailwind-Assets + SQLite für sqlx `query!`-Makros (compile-time checked).
# Ein RUN statt fünf -> weniger Layer, bessere Cache-Wiederverwendung.
# DATABASE_URL explizit statt via .env (siehe .dockerignore), SQLX offline nicht möglich ohne .sqlx-Cache.
ENV DATABASE_URL=sqlite://data/mcss.sqlite
RUN cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs \
    && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs \
    && cd /app && mkdir -p data && sqlx database create && sqlx migrate run

# Release-Bundle mit gecachten Registries/Targets (BuildKit-Cache-Mounts statt Layer-Bloat).
# `dx bundle` schreibt nach target/dx/web/release/web/ – danach aus dem Mount herauskopieren.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    dx bundle --web --release \
    && mkdir -p /out/app && cp -r /app/target/dx/web/release/web/. /out/app/

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /out/app /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
