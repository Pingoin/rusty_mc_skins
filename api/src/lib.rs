//! This crate contains all shared fullstack server functions.
#[cfg(feature = "server")]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

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

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
    pub selected_elytra_id: Option<String>,
}


mod users;
pub use users::*;

mod textures;
pub use textures::*;

#[server]
pub async fn hash_password(password: String) -> Result<String, ServerFnError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    // Passwort hashen
    Ok(password_hash)
}
