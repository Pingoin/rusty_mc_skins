use crate::app_error::AppError;
use sqlx::{query};
use crate::User;

use super::Db;

impl Db {

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let rows = query!(
            "SELECT id, username, password_hash, selected_skin_id, selected_cape_id FROM users"
        )
        .fetch_all(&self.pool)
        .await?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                username: row.username,
                password_hash: row.password_hash,
                selected_skin_id: row.selected_skin_id,
                selected_cape_id: row.selected_cape_id,
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

        query!(
            "INSERT OR REPLACE INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)",
            id,
            username,
            password_hash,
        )
        .execute(&self.pool)
        .await?;
        user.id = id;
        Ok(user)
    }

}
