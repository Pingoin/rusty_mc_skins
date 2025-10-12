use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use dioxus::prelude::*;

use crate::{db, SkinType};

pub async fn init(component: fn() -> Element) -> Result<(), anyhow::Error> {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    use axum::routing::get;

    let router = axum::Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/skin/:user_name", get(get_user_skin))
        .route("/cape/:user_name", get(get_user_cape))
        // serve_dioxus_application adds routes to server side render the application, serve static assets, and register server functions
        .serve_dioxus_application(ServeConfigBuilder::default(), component)
        .into_make_service();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

pub async fn get_user_skin(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, SkinType::Skin).await
}

pub async fn get_user_cape(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, SkinType::Cape).await
}

async fn get_tex(user_name: String, texture_type: SkinType) -> Response {
    let database = db::get_db().await;

    let user_name = std::path::Path::new(user_name.as_str())
        .file_stem() // "Klaus" als OsStr
        .and_then(|s| s.to_str()) // in &str konvertieren
        .unwrap_or(user_name.as_str())
        .to_string();
    println!("{}", user_name.clone());

    match database.get_skin(user_name, texture_type).await {
        Ok(Some(image_data)) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "image/png"),
                (header::CACHE_CONTROL, "public, max-age=3600"),
            ],
            image_data,
        )
            .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Texture not found").into_response(),
        Err(e) => {
            eprintln!("Database error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
        }
    }
}
