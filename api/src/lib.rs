//! This crate contains all shared fullstack server functions.
use base64::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod db;
#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
mod app_error;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input.to_uppercase())
}

#[cfg(feature = "server")]
pub fn init(component: fn() -> Element) {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            server::init(component).await.unwrap();
        });
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub password_hash: String,
    pub avatar_image: Blob,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum SkinType {
    Skin,
    Cape,
    Elytra,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Texture {
    pub id: Option<String>,
    pub skin_name: String,
    pub texture_type: SkinType,
    pub image_data: Blob,
}

use serde::de::Error as DeError;
use serde::{Deserializer, Serializer};

#[derive(Debug, Clone)]
pub struct Blob(pub Vec<u8>);

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = BASE64_STANDARD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64_STANDARD
            .decode(&s)
            .map_err(|e| D::Error::custom(e.to_string()))?;
        Ok(Blob(decoded))
    }
}

impl From<Vec<u8>> for Blob {
    fn from(vec: Vec<u8>) -> Self {
        Blob(vec)
    }
}

#[server(CreateUser)]
async fn create_user(user: User) -> Result<User, ServerFnError> {
    let database = db::get_db().await;
    // Optionally, add the user to the database here
    let user = database.add_user(user).await?;

    Ok(user)
}

#[server(GetUsers)]
async fn get_users() -> Result<Vec<User>, ServerFnError> {
    // Optionally, retrieve user data from the database here
    let database = db::get_db().await;
    let users = database.get_users().await?;
    Ok(users)
}

#[server(CreateTexture)]
async fn create_texture(textrure: Texture) -> Result<Texture, ServerFnError> {
    // Optionally, add the texture to the database here
    let database = db::get_db().await;
    let texture = database.add_texture(textrure).await?;

    Ok(texture)
}

#[server(GetTextures)]
async fn get_textures() -> Result<Vec<Texture>, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_textures().await?;
    Ok(textures)
}
