use sqlx::query;

use crate::{app_error::AppError, Blob, TextureType, Texture};

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
        query!(
            "INSERT OR REPLACE INTO textures (id, skin_name, texture_type, image_data) VALUES (?1, ?2, ?3, ?4)",
            id,
            name,
            texture_type,
            image_data.0,
        )
        .execute(&self.pool)
        .await?;
        texture.id = id;
        Ok(texture)
    }

    pub async fn get_textures(&self) -> Result<Vec<Texture>, AppError> {
        let rows = query!("SELECT id, skin_name, texture_type, image_data FROM textures")
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
        let texture = sqlx::query_as!(Texture, 
            "SELECT textures.id AS id,skin_name,texture_type,image_data 
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_skin_id 
            WHERE users.id = ? ", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(texture)
    }

    pub async fn get_cape_by_user_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(Texture, 
            "SELECT textures.id AS id,skin_name,texture_type,image_data 
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_cape_id 
            WHERE users.id = ? ", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(texture)
    }

    pub async fn get_elytra_by_user_id(&self, id: String) -> Result<Texture, AppError> {
        let texture = sqlx::query_as!(Texture, 
            "SELECT textures.id AS id,skin_name,texture_type,image_data 
            FROM users INNER JOIN textures 
            ON textures.id = users.selected_elytra_id 
            WHERE users.id = ? ", id)
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

    pub async fn get_textures_by_type(&self, tex_type: TextureType) -> Result<Vec<Texture>, AppError> {
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
}
