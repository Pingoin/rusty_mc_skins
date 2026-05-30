FROM rust:1 AS builder
WORKDIR /app

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

COPY . .

RUN cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs
RUN cd web && curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs
# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release

FROM backplane/upx:latest AS packer
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app
WORKDIR /usr/local/app

RUN upx --best --lzma server

FROM debian AS runtime
COPY --from=packer /usr/local/app /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
