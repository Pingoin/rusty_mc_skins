//! This crate contains all shared fullstack server functions.
use base64::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use argon2::{
    password_hash::{
        rand_core::OsRng,
         PasswordHasher, SaltString
    },
    Argon2
};

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

#[derive(Debug, Deserialize, Clone,Default, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub enum SkinType {
    #[default]
    Skin,
    Cape,
    Elytra,
}

impl From<String> for SkinType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Skin" => Self::Skin,
            "Cape" => Self::Cape,
            "Elytra" => Self::Elytra,
            _ => Self::Skin,
        }
    }
}

impl Into<String> for SkinType {
    fn into(self) -> String {
        match self {
            SkinType::Skin => "Skin",
            SkinType::Cape => "Cape",
            SkinType::Elytra => "Elytra",
        }
        .to_string()
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Texture {
    pub id: String,
    pub skin_name: String,
    pub texture_type: SkinType,
    pub image_data: Blob,
}

use serde::de::Error as DeError;
use serde::{Deserializer, Serializer};

#[derive(Debug, Clone, Default)]
pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn as_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.0)
    }
}

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

mod users;
pub use users::*;

mod textures;
pub use textures::*;

#[server]
pub async fn hash_password(password:String)->Result<String, ServerFnError> {
let salt = SaltString::generate(&mut OsRng);

// Argon2 with default params (Argon2id v19)
let argon2 = Argon2::default();

// Hash password to PHC string ($argon2id$v=19$...)
let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    // Passwort hashen
    Ok(password_hash)
}