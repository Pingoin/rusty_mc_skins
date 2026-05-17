#[cfg(feature = "server")]
use crate::db;

use super::Group;
use dioxus::prelude::*;

#[post("/api/group/create")]
pub async fn create_group(group: Group) -> Result<Group> {
    let database = db::get_db().await;
    // Optionally, add the group to the database here
    let group = database.add_group(group).await?;
    Ok(group)
}

#[get("/api/group/list")]
pub async fn get_groups() -> Result<Vec<Group>> {
    // Optionally, retrieve group data from the database here
    let database = db::get_db().await;
    let groups = database.get_groups().await?;
    Ok(groups)
}


#[get("/api/group/{id}")]
pub async fn get_group_by_id(id: String) -> Result<Group> {
    // Optionally, retrieve group data from the database
    let database = db::get_db().await;
    let group = database.get_group_by_id(id).await?;
    Ok(group)
}

#[get("/api/group/{id}/del")]
pub async fn del_group_by_id(id: String) -> Result<()> {
    // Optionally, retrieve group data from the database
    let database = db::get_db().await;
    database.del_group_by_id(id).await?;
    Ok(())
}

#[get("/api/user/is_member")]
pub async fn user_is_member(user_id: String, group_id: String) -> Result<bool> {
    let database = db::get_db().await;
    let exists = database.user_is_member(user_id, group_id).await?;
    Ok(exists)
}
