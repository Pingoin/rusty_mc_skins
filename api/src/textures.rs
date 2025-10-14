use std::io::Cursor;

use base64::prelude::*;
use dioxus::prelude::*;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::imageops::{overlay, replace};
use image::{DynamicImage, GenericImageView, RgbaImage};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "server")]
use crate::db;

#[server(CreateTexture)]
pub async fn create_texture(texture: Texture) -> Result<Texture, ServerFnError> {
    let database = db::get_db().await;
    let texture = database.add_texture(texture).await?;
    Ok(texture)
}

#[server(GetTextures)]
pub async fn get_textures() -> Result<Vec<Texture>, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_textures().await?;
    Ok(textures)
}

#[server(GetTextureById)]
pub async fn get_texture_by_id(id: String) -> Result<Texture, ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    let textures = database.get_texture_by_id(id).await?;
    Ok(textures)
}

#[server(DelTextureById)]
pub async fn del_texture_by_id(tex: Texture) -> Result<(), ServerFnError> {
    // Optionally, retrieve user data from the database
    let database = db::get_db().await;
    database.del_texture_by_id(tex.id).await?;
    Ok(())
}

#[server(DelTextureByType)]
pub async fn get_textures_by_type(tex_type: SkinType) -> Result<Vec<Texture>, ServerFnError> {
    let database = db::get_db().await;
    let tex = database.get_textures_by_type(tex_type).await?;
    Ok(tex)
}

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub enum SkinType {
    #[default]
    Skin,
    Cape,
    Elytra,
}

impl From<String> for SkinType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Skin" => Self::Skin,
            "Cape" => Self::Cape,
            "Elytra" => Self::Elytra,
            _ => Self::Skin,
        }
    }
}

impl Into<String> for SkinType {
    fn into(self) -> String {
        match self {
            SkinType::Skin => "Skin",
            SkinType::Cape => "Cape",
            SkinType::Elytra => "Elytra",
        }
        .to_string()
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Texture {
    pub id: String,
    pub skin_name: String,
    pub texture_type: SkinType,
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

        let mut output = RgbaImage::new(18, 34);
        // Base Skin
        let head = img.view(8, 8, 8, 8).to_image();
        let left_leg = img.view(4, 20, 4, 12).to_image();
        let right_leg = img.view(20, 52, 4, 12).to_image();
        let left_arm = img.view(44, 20, 4, 12).to_image();
        let right_arm = img.view(36, 52, 4, 12).to_image();
        let body = img.view(20, 20, 8, 12).to_image();
        replace(&mut output, &head, 5, 1);
        replace(&mut output,&left_leg,5,21);
        replace(&mut output,&right_leg,9,21);
        replace(&mut output,&left_arm,1,9);
        replace(&mut output,&right_arm,13,9);
        replace(&mut output,&body,5,9);

        // Top layer     
        let head = img.view(40, 8, 8, 8).to_image();
        let left_leg = img.view(4, 36, 4, 12).to_image();
        let right_leg = img.view(4, 52, 4, 12).to_image();
        let left_arm = img.view(44, 36, 4, 12).to_image();
        let right_arm = img.view(52, 52, 4, 12).to_image();
        let body = img.view(20, 36, 8, 12).to_image();
        overlay(&mut output, &head, 5, 1);
        overlay(&mut output,&left_leg,5,21);
        overlay(&mut output,&right_leg,9,21);
        overlay(&mut output,&left_arm,1,9);
        overlay(&mut output,&right_arm,13,9);
        overlay(&mut output,&body,5,9);

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

#[derive(Debug, Clone, Default)]
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
