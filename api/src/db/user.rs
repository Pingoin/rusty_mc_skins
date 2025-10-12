use crate::User;
use crate::{app_error::AppError, SkinType};
use sqlx::query;

use super::Db;

impl Db {
    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let rows = query!("SELECT * FROM users").fetch_all(&self.pool).await?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                username: row.username,
                password_hash: row.password_hash,
                selected_skin_id: row.selected_skin_id,
                selected_cape_id: row.selected_cape_id,
                selected_elytra_id: row.selected_elytra_id,
            })
            .collect();

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<User, AppError> {
        let user = sqlx::query_as!(User, "select * from users where id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn del_user_by_id(&self, id: String) -> Result<(), AppError> {
        sqlx::query!("delete from users where id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_user(&self, mut user: User) -> Result<User, AppError> {
        let id = if user.id.len() > 0 {
            user.id.clone()
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let username = user.username.clone();
        let password_hash = user.password_hash.clone();
        let selected_skin_id = user.selected_skin_id.clone();
        let selected_cape_id = user.selected_cape_id.clone();

        query!(
            "INSERT OR REPLACE INTO users (id, username, password_hash,selected_skin_id,selected_cape_id) VALUES (?1, ?2, ?3, ?4,?5)",
            id,
            username,
            password_hash,
            selected_skin_id,
            selected_cape_id,
        )
        .execute(&self.pool)
        .await?;
        user.id = id;
        Ok(user)
    }

    pub async fn get_skin(
        &self,
        user_name: String,
        texture_type: SkinType,
    ) -> Result<Option<Vec<u8>>, AppError> {
        let tex: String = texture_type.clone().into();

        let row = match texture_type.clone(){
                SkinType::Skin =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_skin_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            .fetch_optional(&self.pool)
            .await?.map(|row| row.image_data),
                SkinType::Cape => sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_cape_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            .fetch_optional(&self.pool)
            .await?.map(|row| row.image_data),
                SkinType::Elytra =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_elytra_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex).fetch_optional(&self.pool).await?.map(|row| row.image_data),
            };
        Ok(row)
    }
}
