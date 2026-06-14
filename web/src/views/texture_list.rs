use api::{Blob, Permissions, Texture, TextureType, del_texture_by_id, get_textures, set_texture};
use dioxus::prelude::*;

use crate::{USER, has_permission, reload_me};

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
    let mut texture = use_signal(|| {
        let mut tex = Texture::default();
        tex.texture_type = tex_type;
        tex
    });
    rsx! {

        article { class: "",
            h1 { "Texture List" }
            div { class: "columns-2 gap-4 sm:columns-3 sm:gap-8",
                for (index , texture) in textures.cloned().unwrap_or_default().into_iter().enumerate() {
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
                div { class: "fab",
                    button {
                        class: "btn btn-lg btn-circle btn-primary",
                        "onclick": "my_modal_1.showModal()",
                        "+"
                    }
                }
                dialog { class: "modal", id: "my_modal_1",
                    div { class: "modal-box",
                        div { class: "modal-action",
                            form {
                                onsubmit: move |evt| {
                                    evt.prevent_default();
                                    spawn(async move {
                                        let t = texture.read().clone();
                                        api::create_texture(t).await.unwrap();
                                        let _ = document::eval("document.getElementById('my_modal_1').close()");
                                        textures.restart();
                                    });
                                },

                                h1 { "Texture" }
                                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                                    label { class: "label",
                                        "Texture Name"
                                        input {
                                            class: "input",
                                            placeholder: "Texture Name",
                                            r#type: "text",
                                            value: "{texture.read().skin_name}",
                                            oninput: move |e| {
                                                let mut t = texture.read().clone();
                                                t.skin_name = e.value().clone();
                                                texture.set(t);
                                            },
                                        }
                                    }
                                    input {
                                        r#type: "file",
                                        class: "file-input file-input-primary",
                                        accept: "image/png",
                                        onchange: move |evt| {
                                            async move {
                                                for file in evt.files() {
                                                    if let Ok(file) = file.read_bytes().await {
                                                        let data = Blob(file.into());
                                                        let mut t = texture.read().clone();
                                                        t.image_data = data;
                                                        let _ = t.compress();
                                                        texture.set(t);
                                                    }
                                                }
                                            }
                                        },
                                    }
                                    if !texture.read().image_data.is_empty() {
                                        img {
                                            src: "data:image/png;base64,{texture.read().get_preview().unwrap_or_default().as_base64()}",
                                            width: "100",
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        r#type: "submit",
                                        "Save"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        
        }
    }
}

#[component]
fn TextureCard(texture: Texture,index:usize, on_change: EventHandler) -> Element {
    let is_set = match texture.texture_type {
        TextureType::Skin => USER.cloned().selected_skin_id == Some(texture.id.clone()),
        TextureType::Cape => USER.cloned().selected_cape_id == Some(texture.id.clone()),
        TextureType::Elytra => USER.cloned().selected_elytra_id == Some(texture.id.clone()),
    };

    let set_me = set_texture;
    let me = use_signal(|| texture.clone());
    let id=use_signal(|| texture.id.clone());

    rsx! {
        div { class: "indicator",
            {
                if is_set {
                    rsx! {
                        span { class: "indicator-item badge badge-primary", "Me" }
                    }
                } else {
                    rsx! {}
                }
            }
            div { class: "card bg-base-100 w-96 shadow-sm",
                figure {
                    img {
                        class: "w-48",
                        alt: "Shoes",
                        src: "data:image/png;base64,{texture.get_preview().unwrap_or_default().as_base64()}",
                    }
                }
                div { class: "card-body",
                    h2 { class: "card-title", {texture.skin_name.clone()} }
                    div { class: "card-actions justify-end",
                        if has_permission(Permissions::TEXTURE_EDIT) {
                            button {
                                class: "btn btn-error",
                                "onclick": format!("del_modal_{}.showModal()", index),
                                "Delete"
                            }
                            dialog {
                                class: "modal",
                                id: format!("del_modal_{}", index),
                                div { class: "modal-box",
                                    h3 { class: "text-lg font-bold", "Hello!" }
                                    p { class: "py-4", {id.clone()} }
                                    div { class: "modal-action",
                                        form { method: "dialog",
                                            button { class: "btn", "Abort" }
                                            button {
                                                class: "btn btn-error",
                                                onclick: move |evt| {
                                                    evt.prevent_default();
                                                    let value = id.clone();
                                                    async move {
                                                        let _ = del_texture_by_id(value.cloned()).await;
                                                        on_change.call(());
                                                        let _ = document::eval(
                                                            format!("document.getElementById('del_modal_{}').close()", index)
                                                                .as_str(),
                                                        );
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| async move {
                                let _ = set_me(me()).await;
                                reload_me();
                            },
                            "Apply to me"
                        }
                    
                    }
                }
            }
        }
    }
}
