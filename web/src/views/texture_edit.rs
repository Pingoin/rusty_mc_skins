use api::{get_texture_by_id, Blob, SkinType, Texture};
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn TextureEdit(id: String) -> Element {
    let mut texture = use_signal(|| Texture::default());

    let tex_id=texture.read().clone().id;

    if !(id.len() == 0 || id == "new".to_string()|| tex_id==id) {
        spawn(async move {
            let t = get_texture_by_id(id).await.unwrap_or_default();
            texture.set(t);
        });
    }

    rsx! {
        article {
            h1 { "Texture" }
            input {
                value: "{texture.read().skin_name}",
                oninput: move |e| {
                    let mut t = texture.read().clone();
                    t.skin_name = e.value().clone();
                    texture.set(t);
                }
            }
            select {
                onchange: move |e| {
                    let mut t = texture.read().clone();
                    t.texture_type = match e.value().as_str() {
                        "Skin" => SkinType::Skin,
                        "Cape" => SkinType::Cape,
                        "Elytra" => SkinType::Elytra,
                        _ => SkinType::Skin,
                    };
                    texture.set(t);
                },
                option { value: "Skin", "Skin" }
                option { value: "Cape", "Cape" }
                option { value: "Elytra", "Elytra" }
            }
             input {
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".png",
            // pick multiple files
            multiple: false,
            onchange:move |evt| {
                async move {
                if let Some(file_engine) = &evt.files() {
                    let files = file_engine.files();
                    if files.len() > 0 {
                        let file = &files[0];
                        let mut t = texture.read().clone();
                        // read the file as bytes
                        let data = file_engine.read_file(file).await.unwrap_or_default();
                        t.image_data = Blob(data);
                        texture.set(t);
                    }
                }
            }
        }  
        }
            button {
                onclick: move |_| {
                    async move {
                        let nav = navigator();
                    let t = texture.read().clone();
                    api::create_texture(t).await.unwrap();
                    nav.push(Route::TextureList{});

                }

                },
                "Save"
            }
            button {  
                onclick: move|_|{
                    async move {
                        let nav = navigator();
                    let t = texture.read().clone();
                    api::del_texture_by_id(t).await.unwrap();
                    nav.push(Route::TextureList{});

                }
                },
                "delete"
            }
        }
    }
}
