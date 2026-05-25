use api::{Blob, Texture, TextureType, get_texture_by_id};
use dioxus::prelude::*;
#[cfg(feature = "web")]
use rfd::AsyncFileDialog;

use crate::views::Route;

#[component]
pub fn TextureEdit(id: String) -> Element {
    let mut texture = use_signal(|| Texture::default());

    let tex_id = texture.read().clone().id;

    if !(id.len() == 0 || id == "new".to_string() || tex_id == id) {
        spawn(async move {
            let t = get_texture_by_id(id).await.unwrap_or_default();
            texture.set(t);
        });
    }
    #[cfg(feature = "web")]
    let file_handle=move |_| {
                    async move {
                        let file = AsyncFileDialog::new()
                            .add_filter("Textures", &["png"])
                            .set_directory("/")
                            .pick_file()
                            .await;
                        let data = Blob(file.unwrap().read().await);
                        let mut t = texture.read().clone();
                        t.image_data = data;
                        texture.set(t);
                    }
                };
                #[cfg(not(feature = "web"))]
                let file_handle= move |_| {
                    async move {}};

    rsx! {
        article {
            h1 { "Texture" }
            input {
                value: "{texture.read().skin_name}",
                oninput: move |e| {
                    let mut t = texture.read().clone();
                    t.skin_name = e.value().clone();
                    t.compress().unwrap();
                    texture.set(t);
                },
            }
            select {
                onchange: move |e| {
                    let mut t = texture.read().clone();
                    t.texture_type = match e.value().as_str() {
                        "Skin" => TextureType::Skin,
                        "Cape" => TextureType::Cape,
                        "Elytra" => TextureType::Elytra,
                        _ => TextureType::Skin,
                    };
                    texture.set(t);
                },
                option { value: "Skin", "Skin" }
                option { value: "Cape", "Cape" }
                option { value: "Elytra", "Elytra" }
            }
            button { onclick: file_handle, "Pick Texture" }

            img {
                src: "data:image/png;base64,{texture.read().get_preview().unwrap_or_default().as_base64()}",
                width: "100",
            }
            button {
                onclick: move |_| {
                    async move {
                        let nav = navigator();
                        let t = texture.read().clone();
                        api::create_texture(t).await.unwrap();
                        nav.push(Route::TextureList {
                            tex_type: api::TextureType::Skin,
                        });
                    }
                },
                "Save"
            }
            button {
                onclick: move |_| {
                    async move {
                        let nav = navigator();
                        let t = texture.read().clone();
                        api::del_texture_by_id(t.id).await.unwrap();
                        nav.push(Route::TextureList {
                            tex_type: api::TextureType::Skin,
                        });
                    }
                },
                "delete"
            }
        }
    }
}
