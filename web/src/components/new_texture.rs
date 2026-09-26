use api::{AppError, Blob, Texture, TextureType};
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::show_alert;

fn file_stem(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem.to_string(),
        _ => name.to_string(),
    }
}

#[component]
pub fn NewTexture(tex_type: TextureType, on_change: EventHandler) -> Element {
    let mut texture = use_signal(|| Texture::default());
    let mut t = texture.read().clone();
    if t.texture_type != tex_type {
        t.texture_type = tex_type;
        texture.set(t);
    }

    rsx! {
        div { class: "fab",
            button {
                class: "btn btn-lg btn-circle btn-primary",
                "onclick": format!("new_tex_modal_{}.showModal()", tex_type),
                "+"
            }
        }
        dialog { class: "modal", id: format!("new_tex_modal_{}", tex_type),
            div { class: "modal-box",
                form { method: "dialog",
                    button {
                        class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                        aria_label: tid!("new-texture-close").to_string(),
                        title: tid!("new-texture-close").to_string(),
                        "✕"
                    }
                }
                div { class: "modal-action",
                    form {
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            let msg_quota = tid!("new-texture-error-quota").to_string();
                            let msg_generic = tid!("new-texture-error-generic").to_string();
                            spawn(async move {
                                let mut t = texture.read().clone();
                                t.texture_type = tex_type;
                                t.owner_id = None;
                                match api::create_texture(t).await {
                                    Ok(_) => {
                                        let _ = document::eval(
                                            &format!("document.getElementById('new_tex_modal_{}').close()", tex_type),
                                        );
                                        on_change.call(());
                                    }
                                    Err(e) => {
                                        let msg = match e {
                                            AppError::QuotaExceeded { limit } => {
                                                format!("{msg_quota} ({limit})")
                                            }
                                            AppError::Unauthorized | AppError::Forbidden => {
                                                msg_generic.clone()
                                            }
                                            _ => format!("{msg_generic}: {e}"),
                                        };
                                        show_alert(msg);
                                    }
                                }
                            });
                        },

                        h1 { {tid!("new-texture-title")} }
                        fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                            label { class: "label",
                                {tid!("new-texture-name")}
                                input {
                                    class: "input",
                                    placeholder: "{tid!(\"new-texture-name-placeholder\")}",
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
                                            let file_name = file_stem(&file.name());
                                            if let Ok(file) = file.read_bytes().await {
                                                let data = Blob(file.into());
                                                let mut t = texture.read().clone();
                                                t.image_data = data;
                                                let _ = t.compress();
                                                if t.skin_name.trim().is_empty() {
                                                    t.skin_name = file_name.clone();
                                                }
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
                            button { class: "btn btn-primary", r#type: "submit",
                                {tid!("new-texture-save")}
                            }
                        }
                    }
                }
            }
        }
    }
}
