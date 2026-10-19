use base64::prelude::*;
use dioxus::fullstack::body::Body;
use dioxus::fullstack::response::{IntoResponse, Response};
use dioxus::prelude::*;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::imageops::{overlay, replace};
use image::{DynamicImage, GenericImageView, RgbaImage};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Cursor;
use std::string::ToString;

#[cfg(feature = "server")]
use crate::auth;
#[cfg(feature = "server")]
use crate::db;
#[cfg(feature = "server")]
use crate::Permissions;
use crate::app_error::AppError;

#[post("/api/texture/create", auth: auth::Session)]
pub async fn create_texture(mut texture: Texture) -> Result<Texture, AppError> {
    let user = auth
        .current_user
        .filter(|u| !u.anonymous())
        .ok_or(AppError::Unauthorized)?;
    if !user.has_permission(Permissions::TEXTURE_EDIT) {
        return Err(AppError::Forbidden);
    }
    let database = db::get_db().await;

    // Update of an existing texture: no quota is consumed, but only the owner
    // (or a user with TEXTURE_ADOPT) may replace it.
    if !texture.id.is_empty()
        && let Ok(existing) = database.get_texture_by_id(texture.id.clone()).await
    {
        let is_owner = existing.owner_id.as_deref() == Some(user.id.as_str());
        // Replacing a foreign texture requires TEXTURE_ADOPT ("manage foreign textures").
        let can_manage = user.has_permission(Permissions::TEXTURE_ADOPT);
        if !is_owner && !can_manage {
            return Err(AppError::Forbidden);
        }
        // The owner is kept (orphaned legacy textures get adopted here).
        texture.owner_id = existing.owner_id.or(Some(user.id.clone()));
        return Ok(database.add_texture(texture).await?);
    }

    // New upload: check the per-group quota.
    let limit = database
        .get_effective_max_textures(user.id.clone())
        .await?;
    let used = database.count_textures_by_owner(user.id.clone()).await?;
    if used >= limit {
        return Err(AppError::QuotaExceeded { limit });
    }

    texture.id = String::new();
    texture.owner_id = Some(user.id.clone());
    let texture = database.add_texture(texture).await?;
    Ok(texture)
}

#[get("/api/texture/list")]
pub async fn get_textures() -> Result<Vec<Texture>> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_textures().await?;
    Ok(textures)
}

#[get("/api/texture/{id}")]
pub async fn get_texture_by_id(id: String) -> Result<Texture> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_texture_by_id(id).await?;
    Ok(textures)
}

#[post("/api/texture/{id}/del", auth: auth::Session)]
pub async fn del_texture_by_id(id: String) -> Result<(), AppError> {
    let user = auth
        .current_user
        .filter(|u| !u.anonymous())
        .ok_or(AppError::Unauthorized)?;
    let database = db::get_db().await;
    // Fail-closed: existence and ownership must be established, otherwise no
    // deletion. Own textures: TEXTURE_EDIT. Foreign textures and legacy
    // textures without owner_id (NULL): TEXTURE_ADOPT ("manage foreign textures").
    let texture = database.get_texture_by_id(id.clone()).await?;
    let is_owner = texture.owner_id.as_deref() == Some(user.id.as_str());
    let allowed = if is_owner {
        user.has_permission(Permissions::TEXTURE_EDIT)
    } else {
        user.has_permission(Permissions::TEXTURE_ADOPT)
    };
    if !allowed {
        return Err(AppError::Forbidden);
    }
    database.del_texture_by_id(id).await?;
    Ok(())
}

/// Transfer a texture to the current user. Holders of TEXTURE_ADOPT can claim
/// legacy textures without an owner_id and take over textures that were
/// previously unassigned (or belong to someone else).
#[post("/api/texture/{id}/adopt", auth: auth::Session)]
pub async fn adopt_texture(id: String) -> Result<Texture, AppError> {
    let user = auth
        .current_user
        .filter(|u| !u.anonymous())
        .ok_or(AppError::Unauthorized)?;
    // Taking ownership changes the owner and with it who may delete the
    // texture: that is what TEXTURE_ADOPT ("manage foreign textures") is for.
    if !user.has_permission(Permissions::TEXTURE_ADOPT) {
        return Err(AppError::Forbidden);
    }
    let database = db::get_db().await;
    let mut texture = database.get_texture_by_id(id.clone()).await?;
    if texture.owner_id.as_deref() == Some(user.id.as_str()) {
        return Ok(texture);
    }
    // add_texture writes via INSERT OR REPLACE and keeps id and image data.
    texture.owner_id = Some(user.id.clone());
    database.add_texture(texture).await
}

#[get("/api/texture/my/{tex_type}", auth: auth::Session)]
pub async fn get_my_texture_type(tex_type: TextureType) -> Result<Texture> {
    let id = auth.id;

    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = match tex_type {
        TextureType::Skin => database.get_skin_by_user_id(id).await?,
        TextureType::Cape => database.get_cape_by_user_id(id).await?,
        TextureType::Elytra => database.get_elytra_by_user_id(id).await?,
    };
    Ok(textures)
}

#[get("/api/texture/list/{tex_type}")]
pub async fn get_textures_by_type(tex_type: String) -> Result<Vec<Texture>> {
    let database = db::get_db().await;
    let tex = database.get_textures_by_type(tex_type.into()).await?;
    Ok(tex)
}

#[get("/api/texture/owner/{owner_id}")]
pub async fn get_textures_by_owner(owner_id: String) -> Result<Vec<Texture>, AppError> {
    let database = db::get_db().await;
    let tex = database.get_textures_by_owner(owner_id).await?;
    Ok(tex)
}

/// Own texture quota: number of uploads already made + effective group limit
/// (maximum across all of the user's groups).
#[post("/api/texture/quota", auth: auth::Session)]
pub async fn get_my_quota() -> Result<TextureQuota, AppError> {
    let user = auth
        .current_user
        .filter(|u| !u.anonymous())
        .ok_or(AppError::Unauthorized)?;
    let database = db::get_db().await;
    let used = database.count_textures_by_owner(user.id.clone()).await?;
    let limit = database
        .get_effective_max_textures(user.id.clone())
        .await?;
    Ok(TextureQuota { used, limit })
}

#[derive(
    Debug,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    strum_macros::Display,
    strum_macros::EnumString,
    Default,
    Serialize,
)]
pub enum TextureType {
    #[default]
    Skin,
    Cape,
    Elytra,
}

impl From<String> for TextureType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Skin" => Self::Skin,
            "Cape" => Self::Cape,
            "Elytra" => Self::Elytra,
            _ => Self::Skin,
        }
    }
}

impl Into<String> for TextureType {
    fn into(self) -> String {
        match self {
            TextureType::Skin => "Skin",
            TextureType::Cape => "Cape",
            TextureType::Elytra => "Elytra",
        }
        .to_string()
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Default)]
pub struct Texture {
    pub id: String,
    pub skin_name: String,
    pub texture_type: TextureType,
    pub image_data: Blob,
    /// User ID of the uploader. `None` for legacy textures without an assignment.
    #[serde(default)]
    pub owner_id: Option<String>,
}

/// Used/occupied quota of a user.
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Default)]
pub struct TextureQuota {
    pub used: i64,
    pub limit: i64,
}

impl Texture {
    pub fn compress(&mut self) -> Result<(), anyhow::Error> {
        let img = image::load_from_memory_with_format(
            self.image_data.0.as_slice(),
            image::ImageFormat::Png,
        )?
        .to_rgba8();

        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        let encoder = PngEncoder::new_with_quality(
            &mut cursor,
            CompressionType::Best, // beste Kompression (verlustfrei)
            FilterType::Adaptive,  // adaptive Filter für bessere Kompression
        );

        DynamicImage::ImageRgba8(img).write_with_encoder(encoder)?;

        self.image_data = Blob(buf);
        Ok(())
    }

    pub fn get_preview(&self) -> Result<Blob, anyhow::Error> {
        let img = image::load_from_memory_with_format(
            self.image_data.0.as_slice(),
            image::ImageFormat::Png,
        )?
        .to_rgba8();

        let output = if self.texture_type != TextureType::Skin {
            img
        } else {
            let factor = img.width() / 64;
            let ratio = img.width() / img.height();

            let mut output = RgbaImage::new(18 * factor, 34 * factor);
            if ratio == 1 {
                // Base Skin
                let head = img
                    .view(8 * factor, 8 * factor, 8 * factor, 8 * factor)
                    .to_image();
                let left_leg = img
                    .view(4 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_leg = img
                    .view(20 * factor, 52 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let left_arm = img
                    .view(44 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_arm = img
                    .view(36 * factor, 52 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let body = img
                    .view(20 * factor, 20 * factor, 8 * factor, 12 * factor)
                    .to_image();
                replace(&mut output, &head, (4 * factor).into(), (0 * factor).into());
                replace(
                    &mut output,
                    &left_leg,
                    (4 * factor).into(),
                    (20 * factor).into(),
                );
                replace(
                    &mut output,
                    &right_leg,
                    (8 * factor).into(),
                    (20 * factor).into(),
                );
                replace(
                    &mut output,
                    &left_arm,
                    (0 * factor).into(),
                    (8 * factor).into(),
                );
                replace(
                    &mut output,
                    &right_arm,
                    (12 * factor).into(),
                    (8 * factor).into(),
                );
                replace(&mut output, &body, (4 * factor).into(), (8 * factor).into());

                // Top layer
                let head = img
                    .view(40 * factor, 8 * factor, 8 * factor, 8 * factor)
                    .to_image();
                let left_leg = img
                    .view(4 * factor, 36 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_leg = img
                    .view(4 * factor, 52 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let left_arm = img
                    .view(44 * factor, 36 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_arm = img
                    .view(52 * factor, 52 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let body = img
                    .view(20 * factor, 36 * factor, 8 * factor, 12 * factor)
                    .to_image();
                overlay(&mut output, &head, (4 * factor).into(), (0 * factor).into());
                overlay(
                    &mut output,
                    &left_leg,
                    (4 * factor).into(),
                    (20 * factor).into(),
                );
                overlay(
                    &mut output,
                    &right_leg,
                    (8 * factor).into(),
                    (20 * factor).into(),
                );
                overlay(
                    &mut output,
                    &left_arm,
                    (0 * factor).into(),
                    (8 * factor).into(),
                );
                overlay(
                    &mut output,
                    &right_arm,
                    (12 * factor).into(),
                    (8 * factor).into(),
                );
                overlay(&mut output, &body, (4 * factor).into(), (8 * factor).into());
            } else {
                let head = img
                    .view(8 * factor, 8 * factor, 8 * factor, 8 * factor)
                    .to_image();
                let left_leg = img
                    .view(4 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_leg = img
                    .view(4 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let left_arm = img
                    .view(44 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let right_arm = img
                    .view(44 * factor, 20 * factor, 4 * factor, 12 * factor)
                    .to_image();
                let body = img
                    .view(20 * factor, 20 * factor, 8 * factor, 12 * factor)
                    .to_image();
                replace(&mut output, &head, (4 * factor).into(), (0 * factor).into());
                replace(
                    &mut output,
                    &left_leg,
                    (4 * factor).into(),
                    (20 * factor).into(),
                );
                replace(
                    &mut output,
                    &right_leg,
                    (8 * factor).into(),
                    (20 * factor).into(),
                );
                replace(
                    &mut output,
                    &left_arm,
                    (0 * factor).into(),
                    (8 * factor).into(),
                );
                replace(
                    &mut output,
                    &right_arm,
                    (12 * factor).into(),
                    (8 * factor).into(),
                );
                replace(&mut output, &body, (4 * factor).into(), (8 * factor).into());

                let head = img
                    .view(40 * factor, 8 * factor, 8 * factor, 8 * factor)
                    .to_image();
                overlay(&mut output, &head, (4 * factor).into(), (0 * factor).into());
            }
            output
        };
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        let encoder = PngEncoder::new_with_quality(
            &mut cursor,
            CompressionType::Best, // beste Kompression (verlustfrei)
            FilterType::Adaptive,  // adaptive Filter für bessere Kompression
        );

        DynamicImage::ImageRgba8(output).write_with_encoder(encoder)?;
        Ok(Blob(buf))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn as_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.0)
    }
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = BASE64_STANDARD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64_STANDARD
            .decode(&s)
            .map_err(|e| D::Error::custom(e.to_string()))?;
        Ok(Blob(decoded))
    }
}

impl From<Vec<u8>> for Blob {
    fn from(vec: Vec<u8>) -> Self {
        Blob(vec)
    }
}

impl IntoResponse for Blob {
    fn into_response(self) -> Response {
        Response::builder()
            .status(200)
            .header("Content-Type", "image/png")
            .header("cache-control", "max-age=3600")
            .body(Body::from(self.0))
            .unwrap()
    }
}
