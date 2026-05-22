use std::collections::HashSet;

use crate::{app_error::AppError, SkinType};
use crate::{Blob, User};
#[cfg(feature = "server")]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use argon2::{PasswordHash, PasswordVerifier};
use sqlx::query;

use super::Db;

async fn hash_password(password: String) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    // Passwort hashen
    Ok(password_hash)
}

impl Db {
    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let rows = query!("SELECT * FROM users").fetch_all(&self.pool).await?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                username: row.username,
                selected_skin_id: row.selected_skin_id,
                selected_cape_id: row.selected_cape_id,
                selected_elytra_id: row.selected_elytra_id,
                anonymous: false,
                permissions: HashSet::new(),
            })
            .collect();

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<User, AppError> {
        let user = sqlx::query_as!(DbUser, "select * from users where id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user.into())
    }

    pub async fn del_user_by_id(&self, id: String) -> Result<(), AppError> {
        sqlx::query!("delete from users where id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub(crate) async fn get_user_by_name(&self, name: String) -> Result<DbUser, AppError> {
        let user = sqlx::query_as!(DbUser, "select * from users where username = ?", name)
            .fetch_one(&self.pool)
            .await?;
        Ok(user.into())
    }

    pub async fn add_user(&self, mut user: User, password: String) -> Result<User, AppError> {
        let id = if user.id.len() > 0 {
            user.id.clone()
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let username = user.username.clone();
        let password_hash = hash_password(password).await?;
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
    ) -> Result<Option<Blob>, AppError> {
        let tex: String = texture_type.clone().into();

        let row = match texture_type.clone(){
                SkinType::Skin =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_skin_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            .fetch_optional(&self.pool)
            .await?.map(|row| row.image_data),
                SkinType::Cape => sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_cape_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            .fetch_optional(&self.pool)
            .await?.map(|row| row.image_data),
                SkinType::Elytra =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_elytra_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex).fetch_optional(&self.pool).await?.map(|row| row.image_data),
            };

        Ok(row.map(|data| Blob(data)))
    }
}

pub(crate) struct DbUser {
    id: String,
    username: String,
    selected_skin_id: Option<String>,
    selected_cape_id: Option<String>,
    selected_elytra_id: Option<String>,
    password_hash: String,
}

impl DbUser {
    pub fn verify_password(&self, password: String) -> anyhow::Result<()> {
        let parsed_hash =
            PasswordHash::new(&self.password_hash).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        Ok(())
    }
}

impl Into<User> for DbUser {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            selected_skin_id: self.selected_skin_id,
            selected_cape_id: self.selected_cape_id,
            selected_elytra_id: self.selected_elytra_id,
            anonymous: false,
            permissions: HashSet::new(),
        }
    }
}
