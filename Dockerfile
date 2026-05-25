FROM rust:1 AS builder
WORKDIR /app

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"


COPY ./web/src/build.rs ./web/src/main.rs
COPY ./web/Cargo.toml ./web/Cargo.toml
COPY ./web/tailwind.css /web/
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./api/src/build.rs ./api/src/lib.rs
COPY ./Cargo.toml ./Cargo.lock ./
RUN dx bundle --web --release
RUN rm ./web/src/main.rs
RUN rm ./api/src/lib.rs
COPY . .
# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release

FROM debian AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
