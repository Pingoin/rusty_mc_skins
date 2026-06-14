use api::{Permissions, Texture, TextureType, User, get_me, get_textures, set_texture};
use dioxus::prelude::*;


use crate::{components::NewTexture, has_permission};

#[component]
pub fn TextureList(tex_type: TextureType) -> Element {
    let textures = use_resource(use_reactive!(|tex_type| async move {
        get_textures()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|t| tex_type == t.texture_type)
            .collect::<Vec<Texture>>()
    }));
    let user = use_resource(|| async move { get_me().await.ok() });

    rsx! {

        article { class: "",
            h1 { "Texture List" }
            div { class: "columns-2 gap-4 sm:columns-3 sm:gap-8",
                for texture in textures.cloned().unwrap_or_default() {
                    TextureCard { texture, user_res: user.clone() }
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
                        h3 { class: "text-lg font-bold", "Hello!" }
                        p { class: "py-4", "Press ESC key or click the button below to close" }
                        div { class: "modal-action",
                            NewTexture { tex_type }
                        }
                    }
                }
            }
        
        }
    } 
}

#[component]
fn TextureCard(texture: Texture, user_res: Resource<Option<User>>) -> Element {
    let user = user_res.cloned().flatten();

    let is_set = match user {
        Some(user) => match texture.texture_type {
            TextureType::Skin => user.selected_skin_id == Some(texture.id.clone()),
            TextureType::Cape => user.selected_cape_id == Some(texture.id.clone()),
            TextureType::Elytra => user.selected_elytra_id == Some(texture.id.clone()),
        },
        None => false,
    };

    let set_me = set_texture;
    let me = use_signal(|| texture.clone());

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
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| async move {
                                let _ = set_me(me()).await;
                                user_res.set(Some(get_me().await.ok()));
                            },
                            "Apply to me"
                        }
                    }
                }
            }
        }
    }
}
