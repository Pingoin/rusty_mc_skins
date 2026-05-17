#[cfg(feature = "server")]
use crate::db;

use super::User;
use dioxus::prelude::*;

#[post("/api/user/create")]
pub async fn create_user(user: User) -> Result<User> {
    let database = db::get_db().await;
    // Optionally, add the user to the database here
    let user = database.add_user(user).await?;

    Ok(user)
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
