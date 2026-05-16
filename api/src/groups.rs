#[cfg(feature = "server")]
use crate::db;

use super::Group;
use dioxus::prelude::*;

#[server(CreateGroup)]
pub async fn create_group(group: Group) -> Result<Group, ServerFnError> {
    let database = db::get_db().await;
    // Optionally, add the group to the database here
    let group = database.add_group(group).await?;

    Ok(group)
}

#[server(GetGroups)]
pub async fn get_groups() -> Result<Vec<Group>, ServerFnError> {
    // Optionally, retrieve group data from the database here
    let database = db::get_db().await;
    let groups = database.get_groups().await?;
    Ok(groups)
}

#[server(GetGroupById)]
pub async fn get_group_by_id(id: String) -> Result<Group, ServerFnError> {
    // Optionally, retrieve group data from the database
    let database = db::get_db().await;
    let group = database.get_group_by_id(id).await?;
    Ok(group)
}

#[server(DelGroupById)]
pub async fn del_group_by_id(tex: Group) -> Result<(), ServerFnError> {
    // Optionally, retrieve group data from the database
    let database = db::get_db().await;
    database.del_group_by_id(tex.id).await?;
    Ok(())
}

#[server(IsMember)]
pub async fn user_is_member(user_id: String, group_id: String) -> Result<bool, ServerFnError> {
    let database = db::get_db().await;
    let exists = database.user_is_member(user_id, group_id).await?;
    Ok(exists)
}
