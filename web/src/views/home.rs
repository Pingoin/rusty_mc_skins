use api::{get_my_texture_type, logout};
use dioxus::prelude::*;

use crate::{USER, reload_me, components::LoginCard};

#[component]
pub fn Home() -> Element {
    rsx! {
        article {
            {
                if USER.cloned().anonymous() {

                    rsx! {
                        LoginCard {}
                    }
                } else {
                    rsx! {
                        UserCard {}
                    }
                }
            }
        }
    }
}

#[component]
fn UserCard() -> Element {
    let skin =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Skin).await.ok() });
    let cape =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Cape).await.ok() });
    let elytra =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Elytra).await.ok() });
    rsx! {
        div { class: "card card-border bg-base-100 w-96",
            div { class: "card-body",
                h2 { class: "card-title", "Ich" }
                {
                    if let Some(Some(skin)) = skin.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                {
                    if let Some(Some(skin)) = elytra.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                {
                    if let Some(Some(skin)) = cape.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                div { class: "card-actions justify-end",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| async move {
                            let _ = logout().await;
                            reload_me();
                        },
                        {"Logout"}
                    }
                }
            }
        }
    }
}

