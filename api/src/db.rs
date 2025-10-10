
use sqlx::{SqlitePool, query};
use tokio::sync::OnceCell;
use crate::{Blob, Texture, User};
use crate::app_error::AppError;

static DATABASE: OnceCell<Db> = OnceCell::const_new();

pub async fn get_db() -> &'static Db {
    DATABASE.get_or_init(|| async { Db::new().await.unwrap() }).await
}

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect("sqlite://mcss.sqlite?mode=rwc").await?;
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn add_user(&self,mut user: User) -> Result<User,AppError> {
        let id = if let Some(id) = user.id.clone() {
            id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let username = user.username.clone();
        let password_hash = user.password_hash.clone();
        let avatar = user.avatar_image.clone().0;
        query!(
            "INSERT OR REPLACE INTO users (id, username, password_hash, avatar_image) VALUES (?1, ?2, ?3, ?4)",
            id,
            username,
            password_hash,
            avatar 
        )
        .execute(&self.pool)
        .await?;
        user.id=Some(id);
        Ok(user)
    }

    pub async fn add_texture(&self,mut texture:Texture) -> Result<Texture,AppError> {
        let id = if let Some(id) = texture.id.clone() {
            id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let name = texture.skin_name.clone();
        let texture_type = match texture.texture_type {
            crate::SkinType::Skin => "Skin",
            crate::SkinType::Cape => "Cape",
            crate::SkinType::Elytra => "Elytra",
        };
        let image_data = texture.image_data.clone();
        query!(
            "INSERT OR REPLACE INTO textures (id, skin_name, texture_type, image_data) VALUES (?1, ?2, ?3, ?4)",
            id,
            name,
            texture_type,
            image_data.0
        )
        .execute(&self.pool)
        .await?;
        texture.id=Some(id);
        Ok(texture)
    }

    pub async fn get_textures(&self) -> Result<Vec<Texture>,AppError> {
        let rows = query!(
            "SELECT id, skin_name, texture_type, image_data FROM textures"
        )
        .fetch_all(&self.pool)
        .await?;

        let textures = rows
            .into_iter()
            .map(|row| Texture {
                id: Some(row.id),
                skin_name: row.skin_name,
                texture_type: match row.texture_type.as_str() {
                    "Skin" => crate::SkinType::Skin,
                    "Cape" => crate::SkinType::Cape,
                    "Elytra" => crate::SkinType::Elytra,
                    _ => crate::SkinType::Skin, // fallback or handle error
                },
                image_data: Blob(row.image_data),
            })
            .collect();

        Ok(textures)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let rows = query!(
            "SELECT id, username, password_hash, avatar_image, selected_skin_id, selected_cape_id FROM users"
        )
        .fetch_all(&self.pool)
        .await?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: Some(row.id),
                username: row.username,
                password_hash: row.password_hash,
                avatar_image: Blob(row.avatar_image.unwrap_or_default()),
                selected_skin_id: row.selected_skin_id,
                selected_cape_id: row.selected_cape_id,
            })
            .collect();

        Ok(users)
    } 
}
