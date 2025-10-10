//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input.to_uppercase())
}

#[cfg(feature = "server")]
pub fn get_router() -> axum::Router {
    use axum::routing::get;

    axum::Router::new().route("/health", get(|| async { "OK" }))
}
#[cfg(feature = "server")]
pub fn init(component: fn() -> Element) {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
            // and we use the generated address the CLI gives us
            let ip = dioxus::cli_config::server_ip()
                .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            let port = dioxus::cli_config::server_port().unwrap_or(8080);
            let address = SocketAddr::new(ip, port);
            let listener = tokio::net::TcpListener::bind(address).await.unwrap();
            let router = get_router()
                // serve_dioxus_application adds routes to server side render the application, serve static assets, and register server functions
                .serve_dioxus_application(ServeConfigBuilder::default(), component)
                .into_make_service();
            axum::serve(listener, router).await.unwrap();
        });
}
