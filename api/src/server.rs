use dioxus::fullstack::extract::Path;
use dioxus::fullstack::response::{IntoResponse, Response};
use dioxus::server::axum::routing::get;
use dioxus::{prelude::*, server::axum::Router};

use crate::{db, Blob, SkinType};

pub async fn get_router(app: fn() -> Element) -> Router {
    let router = dioxus::server::router(app)
        .route("/skin/{user_name}", get(get_skin))
        .route("/cape/{user_name}", get(get_cape));

    router
}

pub async fn get_skin(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, SkinType::Skin)
        .await
        .unwrap()
        .into_response()
}

async fn get_cape(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, SkinType::Cape)
        .await
        .unwrap()
        .into_response()
}

async fn get_tex(user_name: String, texture_type: SkinType) -> Result<Blob> {
    let database = db::get_db().await;

    let user_name = std::path::Path::new(user_name.as_str())
        .file_stem() // "Klaus" als OsStr
        .and_then(|s| s.to_str()) // in &str konvertieren
        .unwrap_or(user_name.as_str())
        .to_string();
    println!("{}", user_name.clone());

    match database.get_skin(user_name, texture_type).await {
        Ok(Some(image_data)) => Ok(image_data),
        _ => Err(anyhow::anyhow!("Texture not found").into()),
    }
}
