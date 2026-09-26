use std::collections::HashMap;

use api::{Permissions, Texture, TextureType, get_textures, get_users};
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    components::{NewTexture, TextureCard},
    has_permission,
};

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
    let mut quota = use_resource(|| async move { api::get_my_quota().await.ok() });
    let users = use_resource(|| async move { get_users().await.unwrap_or_default() });

    let owner_names: HashMap<String, String> = users
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|u| (u.id, u.username))
        .collect();

    rsx! {
        article { class: "",
            h1 {
                {
                    match tex_type {
                        TextureType::Skin => tid!("texture-list-skins"),
                        TextureType::Cape => tid!("texture-list-capes"),
                        TextureType::Elytra => tid!("texture-list-elytra"),
                    }
                }
            }
            if has_permission(Permissions::TEXTURE_EDIT) {
                if let Some(Some(q)) = quota.cloned() {
                    p { class: "text-sm text-base-content/60",
                        {tid!("texture-list-quota", used : q.used.to_string(), limit : q.limit.to_string())}
                    }
                }
            }
            div { class: "columns-2 gap-4 sm:columns-3 sm:gap-8 xl:columns-4",
                for (index, texture) in textures.cloned().unwrap_or_default().into_iter().enumerate() {
                    TextureCard {
                        texture: texture.clone(),
                        index,
                        owner_name: texture
                            .owner_id
                            .clone()
                            .and_then(|id| owner_names.get(&id).cloned()),
                        on_change: move || {
                            textures.restart();
                            quota.restart();
                        },
                    }
                }
            }
            if has_permission(Permissions::TEXTURE_EDIT) {
                NewTexture {
                    tex_type,
                    on_change: move || {
                        textures.restart();
                        quota.restart();
                    },
                }
            }
        }
    }
}
