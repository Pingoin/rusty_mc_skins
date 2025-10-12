#[cfg(feature = "server")]
use crate::db;

use super::User;
use dioxus::prelude::*;

#[server(CreateUser)]
pub async fn create_user(user: User) -> Result<User, ServerFnError> {
    let database = db::get_db().await;
    // Optionally, add the user to the database here
    let user = database.add_user(user).await?;

    Ok(user)
}

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    // Optionally, retrieve user data from the database here
    let database = db::get_db().await;
    let users = database.get_users().await?;
    Ok(users)
}

#[server(GetUserById)]
pub async fn get_user_by_id(id: String) -> Result<User, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let user = database.get_user_by_id(id).await?;
    Ok(user)
}

#[server(DelUserById)]
pub async fn del_user_by_id(tex: User) -> Result<(), ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    database.del_user_by_id(tex.id).await?;
    Ok(())
}
