use sqlx::query;

use crate::{Blob, Texture, TextureType, app_error::AppError};

use super::Db;

impl Db {
    pub async fn add_texture(&self, mut texture: Texture) -> Result<Texture, AppError> {
        let id = if texture.id.len() > 0 {
            texture.id.clone()
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let name = texture.skin_name.clone();
        let texture_type = match texture.texture_type {
            TextureType::Skin => "Skin",
            TextureType::Cape => "Cape",
            TextureType::Elytra => "Elytra",
        };
        let image_data = texture.image_data.clone();
        let owner_id = texture.owner_id.clone();
        query!(
            "INSERT OR REPLACE INTO textures (id, skin_name, texture_type, image_data, owner_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            name,
            texture_type,
            image_data.0,
            owner_id,
        )
        .execute(&self.pool)
        .await?;
        texture.id = id;
        Ok(texture)
    }

    pub async fn get_textures(&self) -> Result<Vec<Texture>, AppError> {
        let rows =
            query!("SELECT id, skin_name, texture_type, image_data, owner_id FROM textures")
                .fetch_all(&self.pool)
                .await?;

        let textures = rows
            .into_iter()
            .map(|row| Texture {
                id: row.id,
                skin_name: row.skin_name,
                texture_type: match row.texture_type.as_str() {
                    "Skin" => TextureType::Skin,
                    "Cape" => TextureType::Cape,
                    "Elytra" => TextureType::Elytra,
                    _ => TextureType::Skin, // fallback or handle error
                },
                image_data: Blob(row.image_data),
                owner_id: row.owner_id,
            })
            .collect();

        Ok(textures)
    }

    pub async fn get_texture_by_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(Texture, "select * from textures where id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(texture)
    }

    pub async fn get_skin_by_user_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(
            Texture,
            "SELECT textures.id AS id,skin_name,texture_type,image_data,owner_id
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_skin_id 
            WHERE users.id = ? ",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(texture)
    }
    pub async fn get_cape_by_user_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(
            Texture,
            "SELECT textures.id AS id,skin_name,texture_type,image_data,owner_id
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_cape_id 
            WHERE users.id = ? ",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(texture)
    }

    pub async fn get_elytra_by_user_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(
            Texture,
            "SELECT textures.id AS id,skin_name,texture_type,image_data,owner_id
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_elytra_id 
            WHERE users.id = ? ",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(texture)
    }

    pub async fn del_texture_by_id(&self, id: String) -> Result<(), AppError> {
        sqlx::query!("delete from textures where id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_textures_by_type(
        &self,
        tex_type: TextureType,
    ) -> Result<Vec<Texture>, AppError> {
        let tex_type: String = tex_type.into();
        let textures = sqlx::query_as!(
            Texture,
            "select * from textures where texture_type = ?",
            tex_type
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(textures)
    }

    pub async fn count_textures_by_owner(&self, owner_id: String) -> Result<i64, AppError> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM textures WHERE owner_id = ?")
                .bind(owner_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(count)
    }

    pub async fn get_textures_by_owner(
        &self,
        owner_id: String,
    ) -> Result<Vec<Texture>, AppError> {
        let textures = sqlx::query_as!(
            Texture,
            "select * from textures where owner_id = ?",
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(textures)
    }
}
