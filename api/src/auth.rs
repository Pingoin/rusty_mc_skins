use async_trait::async_trait;
use axum_session_auth::*;
use axum_session_sqlx::SessionSqlitePool;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::collections::HashSet;

use crate::{User, db::Db};

pub(crate) type Session = axum_session_auth::AuthSession<User, String, SessionSqlitePool, Db>;
pub(crate) type AuthLayer =
    axum_session_auth::AuthSessionLayer<User, String, SessionSqlitePool, Db>;



#[async_trait]
impl Authentication<User, String, Db> for User {
    async fn load_user(userid: String, pool: Option<&Db>) -> Result<User, anyhow::Error> {


        Ok(User::default())
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}
#[async_trait]
impl HasPermission<SqlitePool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
        self.permissions.contains(perm)
    }
}
