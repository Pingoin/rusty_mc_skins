use api::{Permissions, Texture, TextureType, del_texture_by_id, set_texture};
use dioxus::prelude::*;

use crate::{USER, has_permission, reload_me};

#[component]
pub fn TextureCard(texture: Texture,index:usize, on_change: EventHandler) -> Element {
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