use std::io::Cursor;
use base64::prelude::*;
use dioxus::fullstack::body::Body;
use dioxus::fullstack::response::{IntoResponse, Response};
use dioxus::prelude::*;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::imageops::{overlay, replace};
use image::{DynamicImage, GenericImageView, RgbaImage};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::string::ToString;



#[cfg(feature="server")]
use crate::auth;
#[cfg(feature = "server")]
use crate::db;

#[post("/api/texture/create")]
pub async fn create_texture(texture: Texture) -> Result<Texture> {
    let database = db::get_db().await;
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

#[post("/api/texture/{id}/del")]
pub async fn del_texture_by_id(id: String) -> Result<()> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    database.del_texture_by_id(id).await?;
    Ok(())
}

#[get("/api/texture/my/{tex_type}", auth: auth::Session)]
pub async fn get_my_texture_type(tex_type:TextureType) -> Result<Texture> {
    let id= auth.id;

    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = match tex_type{
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

#[derive(Debug, Deserialize, Clone, PartialEq,strum_macros::Display,strum_macros::EnumString,Default, Serialize)]
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


