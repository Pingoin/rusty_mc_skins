use crate::Permissions;
use crate::Texture;
#[cfg(feature = "server")]
use crate::auth;
#[cfg(feature = "server")]
use crate::db;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
    pub selected_elytra_id: Option<String>,
    pub permissions: Permissions,
    pub created: Option<String>,
}

impl User {
    pub fn anonymous(&self) -> bool {
        self.id == "".to_string()
    }
}

#[post("/api/user/create")]
pub async fn create_user(user: User, password: String) -> Result<User> {
    let database = db::get_db().await;

    let user = database.add_user(user, password).await?;

    Ok(user)
}

#[post("/api/user/texture/set", auth: auth::Session)]
pub async fn set_texture(texture: Texture) -> Result<()> {
    let database = db::get_db().await;
    if let Some(user) = auth.clone().current_user
        && !user.anonymous()
    {
        if texture.id == "".to_string() {
            database
                .set_texture(user.id.clone(), None, texture.texture_type)
                .await?;
        } else {
            database
                .set_texture(user.id.clone(), Some(texture.id), texture.texture_type)
                .await?;
        }
        auth.cache_clear_user(user.id);
    }

    Ok(())
}

#[get("/api/user/list")]
pub async fn get_users() -> Result<Vec<User>> {
    // Optionally, retrieve user data from the database here
    let database = db::get_db().await;
    let users = database.get_users().await?;
    Ok(users)
}

#[get("/api/user/{id}")]
pub async fn get_user_by_id(id: String) -> Result<User> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let user = database.get_user_by_id(id).await?;
    Ok(user)
}

#[post("/api/user/{id}/del")]
pub async fn del_user_by_id(id: String) -> Result<()> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    database.del_user_by_id(id).await?;
    Ok(())
}

#[post("/api/user/me", auth: auth::Session)]
pub async fn get_me() -> Result<User> {
    let user = auth.current_user;
    if let Some(user) = user {
        if user.anonymous() {
            Err(anyhow::anyhow!("anonymous User").into())
        } else {
            Ok(user)
        }
    } else {
        Err(anyhow::anyhow!("no User").into())
    }
}

/// We use the `auth::Session` extractor to get access to the current user session.
/// This lets us modify the user session, log in/out, and access the current user.
#[post("/api/user/login", auth: auth::Session)]
pub async fn login(user: String, password: String) -> Result<()> {
    let database = db::get_db().await;
    let user = database.get_user_by_name(user).await?;
    user.verify_password(password)?;
    let user: User = user.into();
    auth.login_user(user.id);
    Ok(())
}

/// Just like `login`, but this time we log out the user.
#[post("/api/user/logout", auth: auth::Session)]
pub async fn logout() -> Result<()> {
    auth.logout_user();
    Ok(())
}
