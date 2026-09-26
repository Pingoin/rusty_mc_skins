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

    pub(crate) fn get_pool(&self) -> SqlitePool {
        self.pool.clone()
    }

    #[cfg(test)]
    pub(crate) fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;

    async fn fresh_db() -> Db {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite");
        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .expect("migrations apply cleanly");
        Db::from_pool(pool)
    }

    #[tokio::test]
    async fn seed_groups_have_quota_defaults() {
        let db = fresh_db().await;
        let groups = db.get_groups().await.expect("groups load");
        let by_id: std::collections::HashMap<_, _> =
            groups.into_iter().map(|g| (g.id.clone(), g)).collect();
        assert_eq!(by_id["adm"].max_textures, 1000);
        assert_eq!(by_id["crtr"].max_textures, 25);
        assert_eq!(by_id["usr"].max_textures, 5);
        // usr-Mitglieder duerfen hochladen (TEXTURE_EDIT = Bit 0)
        assert!(by_id["usr"].permissions.contains(crate::Permissions::TEXTURE_EDIT));
    }

    #[tokio::test]
    async fn texture_owner_link_and_effective_quota() {
        let db = fresh_db().await;
        let pool = db.get_pool();

        // Nutzer Karl ist in usr (0) und crtr (25) -> wirksam 25.
        sqlx::query!("INSERT INTO users (id, username, password_hash) VALUES ('u1', 'karl', 'x')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("INSERT INTO groups_users (user_id, group_id) VALUES ('u1', 'usr'), ('u1', 'crtr')")
            .execute(&pool)
            .await
            .unwrap();

        assert_eq!(db.get_effective_max_textures("u1".into()).await.unwrap(), 25);
        assert_eq!(db.count_textures_by_owner("u1".into()).await.unwrap(), 0);

        let tex = crate::Texture {
            id: String::new(),
            skin_name: "KarlSkin".into(),
            texture_type: crate::TextureType::Skin,
            image_data: crate::Blob(vec![1, 2, 3]),
            owner_id: Some("u1".into()),
        };
        let saved = db.add_texture(tex).await.expect("add texture");
        assert_eq!(saved.owner_id.as_deref(), Some("u1"));

        assert_eq!(db.count_textures_by_owner("u1".into()).await.unwrap(), 1);
        let mine = db.get_textures_by_owner("u1".into()).await.unwrap();
        assert_eq!(mine.len(), 1);
        assert_eq!(mine[0].id, saved.id);

        // Nutzer ohne Gruppe darf nichts hochladen.
        assert_eq!(db.get_effective_max_textures("ghost".into()).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn permissions_or_across_overlapping_groups() {
        // usr (TEXTURE_EDIT|TEXTURE_USE) und crtr (TEXTURE_EDIT) teilen sich
        // Bit 0: SUM() wuerde daraus faelschlich Bit 2 machen und beide Rechte
        // verlieren. Das OR muss alle vier Rechte liefern.
        let db = fresh_db().await;
        let pool = db.get_pool();
        sqlx::query!("INSERT INTO users (id, username, password_hash) VALUES ('u1', 'karl', 'x')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("INSERT INTO groups_users (user_id, group_id) VALUES ('u1', 'usr'), ('u1', 'crtr'), ('u1', 'adm')")
            .execute(&pool)
            .await
            .unwrap();

        let user = db.get_user_by_id("u1".into()).await.unwrap();
        assert!(user.has_permission(crate::Permissions::TEXTURE_USE));
        assert!(user.has_permission(crate::Permissions::TEXTURE_EDIT));
        assert!(user.has_permission(crate::Permissions::USER_EDIT));
        assert!(user.has_permission(crate::Permissions::GROUP_EDIT));
    }

    #[test]
    fn wire_compat_defaults() {
        // Alte Clients senden weder owner_id noch max_textures.
        let tex: crate::Texture =
            serde_json::from_str(r#"{"id":"","skin_name":"x","texture_type":"Skin","image_data":""}"#)
                .expect("texture default owner");
        assert_eq!(tex.owner_id, None);
        let group: crate::Group =
            serde_json::from_str(r#"{"id":"","group_name":"g","permissions":0}"#)
                .expect("group default quota");
        assert_eq!(group.max_textures, crate::groups::DEFAULT_MAX_TEXTURES);
    }
}
