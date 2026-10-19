use api::{Permissions, Texture, TextureType, adopt_texture, del_texture_by_id, set_texture};
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{USER, has_permission, reload_me, show_alert};

#[component]
pub fn TextureCard(
    texture: Texture,
    index: usize,
    on_change: EventHandler,
    owner_name: Option<String>,
) -> Element {
    let is_set = match texture.texture_type {
        TextureType::Skin => USER.cloned().selected_skin_id == Some(texture.id.clone()),
        TextureType::Cape => USER.cloned().selected_cape_id == Some(texture.id.clone()),
        TextureType::Elytra => USER.cloned().selected_elytra_id == Some(texture.id.clone()),
    };

    let set_me = set_texture;
    let me = use_signal(|| texture.clone());
    let id = use_signal(|| texture.id.clone());

    // Deleting and taking ownership are enforced server-side via TEXTURE_ADOPT
    // ("manage foreign textures"); these flags only hide the buttons from
    // users who would be rejected by the API anyway.
    let current_user = USER.cloned();
    let is_own = texture.owner_id.as_deref() == Some(current_user.id.as_str());
    let can_delete = if is_own {
        has_permission(Permissions::TEXTURE_EDIT)
    } else {
        has_permission(Permissions::TEXTURE_ADOPT)
    };
    let can_adopt = has_permission(Permissions::TEXTURE_ADOPT) && !is_own;

    rsx! {
        div { class: "indicator mb-4 w-full break-inside-avoid sm:mb-8",
            {
                if is_set {
                    rsx! {
                        span { class: "indicator-item badge badge-primary", {tid!("texture-active")} }
                    }
                } else {
                    rsx! {}
                }
            }
            div { class: "card bg-base-100 w-full shadow-sm",
                figure {
                    img {
                        class: "w-full max-w-48",
                        alt: "{texture.skin_name}",
                        src: "data:image/png;base64,{texture.get_preview().unwrap_or_default().as_base64()}",
                    }
                }
                div { class: "card-body",
                    h2 { class: "card-title", {texture.skin_name.clone()} }
                    if let Some(name) = owner_name.clone() {
                        p { class: "text-xs text-base-content/60",
                            {tid!("texture-owner", name : name)}
                        }
                    }
                    div { class: "card-actions justify-end",
                        if can_adopt {
                            button {
                                class: "btn btn-warning",
                                "onclick": format!("adopt_modal_{}.showModal()", index),
                                {tid!("texture-adopt")}
                            }
                            dialog {
                                class: "modal",
                                id: format!("adopt_modal_{}", index),
                                div { class: "modal-box",
                                    h3 { class: "text-lg font-bold", {tid!("texture-adopt-title")} }
                                    p { class: "py-4",
                                        {tid!("texture-adopt-message", name : texture.skin_name.clone())}
                                    }
                                    div { class: "modal-action",
                                        form { method: "dialog",
                                            button { class: "btn", {tid!("texture-abort")} }
                                            button {
                                                class: "btn btn-warning",
                                                onclick: move |evt| {
                                                    evt.prevent_default();
                                                    let value = id.clone();
                                                    async move {
                                                        match adopt_texture(value.cloned()).await {
                                                            Ok(_) => {
                                                                on_change.call(());
                                                                let _ = document::eval(
                                                                    format!("document.getElementById('adopt_modal_{}').close()", index)
                                                                        .as_str(),
                                                                );
                                                            }
                                                            Err(e) => {
                                                                show_alert(e.to_string());
                                                            }
                                                        }
                                                    }
                                                },
                                                {tid!("texture-adopt")}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if can_delete {
                            button {
                                class: "btn btn-error",
                                "onclick": format!("del_modal_{}.showModal()", index),
                                {tid!("texture-delete")}
                            }
                            dialog {
                                class: "modal",
                                id: format!("del_modal_{}", index),
                                div { class: "modal-box",
                                    h3 { class: "text-lg font-bold", {tid!("texture-delete-title")} }
                                    p { class: "py-4",
                                        {tid!("texture-delete-message", name : texture.skin_name.clone())}
                                    }
                                    div { class: "modal-action",
                                        form { method: "dialog",
                                            button { class: "btn", {tid!("texture-abort")} }
                                            button {
                                                class: "btn btn-error",
                                                onclick: move |evt| {
                                                    evt.prevent_default();
                                                    let value = id.clone();
                                                    async move {
                                                        match del_texture_by_id(value.cloned()).await {
                                                            Ok(_) => {
                                                                on_change.call(());
                                                                let _ = document::eval(
                                                                    format!("document.getElementById('del_modal_{}').close()", index)
                                                                        .as_str(),
                                                                );
                                                            }
                                                            Err(e) => {
                                                                show_alert(e.to_string());
                                                            }
                                                        }
                                                    }
                                                },
                                                {tid!("texture-delete")}
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
                            {tid!("texture-apply")}
                        }

                    }
                }
            }
        }
    }
}
