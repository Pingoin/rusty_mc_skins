use dioxus::fullstack::extract::Path;
use dioxus::fullstack::response::{IntoResponse, Response};
use dioxus::server::axum::routing::get;
use dioxus::{prelude::*, server::axum::Router};

use crate::{Blob, TextureType, db};

pub async fn get_router(app: fn() -> Element) -> anyhow::Result<Router> {
    use crate::auth::*;
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::AuthConfig;
    use axum_session_sqlx::SessionSqlitePool;
    use sqlx::sqlite::SqlitePoolOptions;

    let database = db::get_db().await;
    let sessoin_db = SqlitePoolOptions::new()
        .max_connections(20)
        .connect_with("sqlite::memory:".parse()?)
        .await?;

    let router = dioxus::server::router(app)
        .route("/skins/{user_name}", get(get_skin))
        .route("/capes/{user_name}", get(get_cape))
        .route("/elytras/{user_name}", get(get_elytra))
        .layer(AuthLayer::new(Some(database.clone())).with_config(
            AuthConfig::<String>::default().with_anonymous_user_id(Some("".to_ascii_uppercase())),
        ))
        .layer(SessionLayer::new(
            SessionStore::<SessionSqlitePool>::new(
                Some(sessoin_db.into()),
                SessionConfig::default().with_table_name("session_table"),
            )
            .await?,
        ));

    Ok(router)
}

pub async fn get_skin(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, TextureType::Skin)
        .await
        .unwrap()
        .into_response()
}

async fn get_cape(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, TextureType::Cape)
        .await
        .unwrap()
        .into_response()
}

async fn get_elytra(Path(user_name): Path<String>) -> Response {
    get_tex(user_name, TextureType::Elytra)
        .await
        .unwrap()
        .into_response()
}
async fn get_tex(user_name: String, texture_type: TextureType) -> Result<Blob> {
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
