use crate::{Blob, Permissions, User};
use crate::{TextureType, app_error::AppError};
#[cfg(feature = "server")]
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use argon2::{PasswordHash, PasswordVerifier};
use sqlx::{QueryBuilder, Sqlite, query};

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
                permissions: Permissions::empty(),
                created: None,
            })
            .collect();

        Ok(users)
    }

    pub async fn get_user_count(&self) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT
                u.id,
                u.username,
                u.selected_skin_id,
                u.selected_cape_id,
                u.selected_elytra_id,
                u.created,
                COALESCE(SUM(g.permissions), 0) AS permissions
            FROM users u
            LEFT JOIN groups_users gu
                ON gu.user_id = u.id
            LEFT JOIN groups g
                ON g.id = gu.group_id
            WHERE u.id = ?
            GROUP BY u.id;",
            id
        )
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
        let count = self.get_user_count().await?;

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
        user.id = id.clone();

        let mut groups = vec![(id.clone(), "usr".to_string())];
        if count == 0 {
            groups.push((id.clone(), "adm".to_string()));
            groups.push((id.clone(), "crtr".to_string()));
        }

        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("INSERT OR REPLACE INTO groups_users (user_id, group_id) ");

        qb.push_values(groups.iter(), |mut b, group| {
            b.push_bind(group.0.clone()).push_bind(group.1.clone());
        });

        qb.build().execute(&self.pool).await?;

        Ok(user)
    }

    pub(crate) async fn set_texture(
        &self,
        user_id: String,
        texture_id: Option<String>,
        texture_type: TextureType,
    ) -> anyhow::Result<()> {
        let sql = match texture_type {
            TextureType::Skin => "UPDATE users SET selected_skin_id = ? WHERE users.id = ?;",
            TextureType::Cape => "UPDATE users SET selected_cape_id = ? WHERE users.id = ?;",
            TextureType::Elytra => "UPDATE users SET selected_elytra_id = ? WHERE users.id = ?;",
        };

        sqlx::query(sql)
            .bind(texture_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_skin(
        &self,
        user_name: String,
        texture_type: TextureType,
    ) -> Result<Option<Blob>, AppError> {
        let tex: String = texture_type.clone().into();

        let row = match texture_type.clone(){
                TextureType::Skin =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_skin_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            
                .fetch_optional(&self.pool).await?.map(|row| row.image_data),
                TextureType::Cape => sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_cape_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)            
                .fetch_optional(&self.pool).await?.map(|row| row.image_data),
                TextureType::Elytra =>  sqlx::query!("SELECT t.image_data FROM users u JOIN textures t ON t.id = u.selected_elytra_id WHERE u.username = ?1 AND t.texture_type = ?2;", user_name,tex)
                .fetch_optional(&self.pool).await?.map(|row| row.image_data),
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
    created: Option<String>,
}

impl DbUser {
    pub fn verify_password(&self, password: String) -> Result<(),AppError> {
        let parsed_hash =
            PasswordHash::new(&self.password_hash).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::WrongPassword)?;
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
            permissions: Permissions::empty(),
            created: self.created,
        }
    }
}
