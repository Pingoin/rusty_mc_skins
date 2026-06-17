use api::{Permissions, Texture, TextureType, get_textures};
use dioxus::prelude::*;

use crate::{components::{NewTexture, TextureCard}, has_permission};

#[component]
pub fn TextureList(tex_type: TextureType) -> Element {
    let mut textures = use_resource(use_reactive!(|tex_type| async move {
        get_textures()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|t| tex_type == t.texture_type)
            .collect::<Vec<Texture>>()
    }));

    rsx! {
        article { class: "",
            h1 { "Texture List" }
            div { class: "columns-2 gap-4 sm:columns-3 sm:gap-8",
                for (index, texture) in textures.cloned().unwrap_or_default().into_iter().enumerate() {
                    TextureCard {
                        texture,
                        index,
                        on_change: move || {
                            textures.restart();
                        },
                    }
                }
            }
            if has_permission(Permissions::TEXTURE_EDIT) {
                NewTexture {
                    tex_type,
                    on_change: move || {
                        textures.restart();
                    },
                }
            }
        }
    }
}

