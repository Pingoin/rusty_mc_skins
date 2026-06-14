use crate::{User, db::Db};
use async_trait::async_trait;
use axum_session_auth::*;
use axum_session_sqlx::SessionSqlitePool;

pub(crate) type Session = axum_session_auth::AuthSession<User, String, SessionSqlitePool, Db>;
pub(crate) type AuthLayer =
    axum_session_auth::AuthSessionLayer<User, String, SessionSqlitePool, Db>;

#[async_trait]
impl Authentication<User, String, Db> for User {
    async fn load_user(userid: String, pool: Option<&Db>) -> anyhow::Result<User> {
        let user = pool
            .ok_or(anyhow::anyhow!("no database"))?
            .get_user_by_id(userid)
            .await?;

        Ok(user)
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous()
    }

    fn is_active(&self) -> bool {
        !self.anonymous()
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous()
    }
}


