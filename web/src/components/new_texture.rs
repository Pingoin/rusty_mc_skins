use api::{Blob, Texture, TextureType};
use dioxus::prelude::*;

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
                div { class: "modal-action",
                    form {
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            spawn(async move {
                                let mut t = texture.read().clone();
                                t.texture_type = tex_type;
                                api::create_texture(t).await.unwrap();
                                let _ = document::eval(
                                    &format!("document.getElementById('new_tex_modal_{}').close()", tex_type),
                                );
                                on_change.call(());
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
                            button { class: "btn btn-primary", r#type: "submit", "Save" }
                        }
                    }
                }
            }
        }
    }
}
