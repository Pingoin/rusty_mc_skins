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

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
    pub selected_elytra_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default, Serialize)]
pub struct Group {
    pub id: String,
    pub group_name: String,
    pub permissions: Permissions,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Serialize)]
pub struct Permissions(i64);

#[repr(u8)]
#[derive(EnumIter, AsRefStr, Clone, PartialEq, Debug)]
pub enum Permission {
    EditGroups = 0,
    EditOtherUser = 1,
}

impl Permissions {
    pub const ALL: [Permission; 2] = [Permission::EditGroups, Permission::EditOtherUser];

    pub fn has_permission(&self, perm: Permission) -> bool {
        (self.0 & (1 << perm as i64)) != 0
    }

    pub fn set_permission(&mut self, perm: Permission, state: bool) {
        if state {
            self.0 |= 1 << perm as i64;
        } else {
            self.0 &= !(1 << perm as i64);
        }
    }
}

impl From<i64> for Permissions {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

mod users;
use strum_macros::{AsRefStr, EnumIter};
pub use users::*;

mod textures;
pub use textures::*;

mod groups;
pub use groups::*;

mod auth;

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
