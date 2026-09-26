use crate::Group;
use crate::app_error::AppError;
use sqlx::query;

use super::Db;

impl Db {
    pub async fn get_groups(&self) -> Result<Vec<Group>, AppError> {
        let rows = query!("SELECT * FROM groups").fetch_all(&self.pool).await?;

        let grups = rows
            .into_iter()
            .map(|row| Group {
                id: row.id,
                group_name: row.group_name,
                permissions: row.permissions.into(),
                max_textures: row.max_textures,
                created: None,
            })
            .collect();

        Ok(grups)
    }

    pub async fn user_is_member(
        &self,
        user_id: String,
        group_id: String,
    ) -> Result<bool, AppError> {
        let exists: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM groups_users
            WHERE user_id = ?1 AND group_id = ?2
            LIMIT 1
            "#,
        )
        .bind(&user_id)
        .bind(&group_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(exists.is_some())
    }

    /// Wirksames Textur-Limit eines Nutzers: Maximum ueber alle seine Gruppen.
    /// Nutzer ohne Gruppe duerfen nichts hochladen (0).
    pub async fn get_effective_max_textures(
        &self,
        user_id: String,
    ) -> Result<i64, AppError> {
        let limit: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT MAX(g.max_textures) FROM groups_users gu
            INNER JOIN groups g ON g.id = gu.group_id
            WHERE gu.user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(limit.unwrap_or(0))
    }

    pub async fn get_group_by_id(&self, id: String) -> Result<Group, AppError> {
        let user = sqlx::query_as!(Group, "select * from groups where id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn del_group_by_id(&self, id: String) -> Result<(), AppError> {
        sqlx::query!("delete from groups where id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_group(&self, mut group: Group) -> Result<Group, AppError> {
        let id = if group.id.len() > 0 {
            group.id.clone()
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        let group_name = group.group_name.clone();
        let permissions = group.permissions.bits();
        let max_textures = group.max_textures.max(0);

        query!(
            "INSERT OR REPLACE INTO groups (id, group_name, permissions, max_textures) VALUES (?1, ?2, ?3, ?4)",
            id,
            group_name,
            permissions as i64,
            max_textures,
        )
        .execute(&self.pool)
        .await?;
        group.id = id;
        group.max_textures = max_textures;
        Ok(group)
    }
}
