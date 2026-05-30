use sqlx::SqlitePool;
use tokio::fs::create_dir_all;
use tokio::sync::OnceCell;
mod groups;
mod textures;
mod user;

static DATABASE: OnceCell<Db> = OnceCell::const_new();

pub async fn get_db() -> &'static Db {
    DATABASE
        .get_or_init(|| async { Db::new().await.unwrap() })
        .await
}

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> anyhow::Result<Self> {
        create_dir_all("data").await?;

        let pool = SqlitePool::connect("sqlite://data/mcss.sqlite?mode=rwc").await?;
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}
