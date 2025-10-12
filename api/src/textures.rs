use super::SkinType;
use super::Texture;
#[cfg(feature = "server")]
use crate::db;
use dioxus::prelude::*;

#[server(CreateTexture)]
pub async fn create_texture(texture: Texture) -> Result<Texture, ServerFnError> {
    let database = db::get_db().await;
    let texture = database.add_texture(texture).await?;
    Ok(texture)
}

#[server(GetTextures)]
pub async fn get_textures() -> Result<Vec<Texture>, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_textures().await?;
    Ok(textures)
}

#[server(GetTextureById)]
pub async fn get_texture_by_id(id: String) -> Result<Texture, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_texture_by_id(id).await?;
    Ok(textures)
}

#[server(DelTextureById)]
pub async fn del_texture_by_id(tex: Texture) -> Result<(), ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    database.del_texture_by_id(tex.id).await?;
    Ok(())
}

#[server(DelTextureByType)]
pub async fn get_textures_by_type(tex_type: SkinType) -> Result<Vec<Texture>, ServerFnError> {
    let database = db::get_db().await;
    let tex = database.get_textures_by_type(tex_type).await?;
    Ok(tex)
}
