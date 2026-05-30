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
            SELECT 1 FROM user_groups
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

        query!(
            "INSERT OR REPLACE INTO groups (id, group_name, permissions) VALUES (?1, ?2, ?3)",
            id,
            group_name,
            permissions as i64,
        )
        .execute(&self.pool)
        .await?;
        group.id = id;
        Ok(group)
    }
}
