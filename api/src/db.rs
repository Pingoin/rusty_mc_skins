use sqlx::SqlitePool;
use tokio::sync::OnceCell;
mod textures;
mod user;

static DATABASE: OnceCell<Db> = OnceCell::const_new();

pub async fn get_db() -> &'static Db {
    DATABASE
        .get_or_init(|| async { Db::new().await.unwrap() })
        .await
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
}
