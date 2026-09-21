use api::{TextureType, User, get_textures_by_type, get_user_by_id};
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn UserEdit(id: String) -> Element {
    let mut user = use_signal(|| User::default());
    let mut password = use_signal(|| String::default());

    let skins = use_resource(|| async move {
        get_textures_by_type(TextureType::Skin.into())
            .await
            .unwrap_or_default()
    });

    let capes = use_resource(|| async move {
        get_textures_by_type(TextureType::Cape.into())
            .await
            .unwrap_or_default()
    });
    let elytra = use_resource(|| async move {
        get_textures_by_type(TextureType::Elytra.into())
            .await
            .unwrap_or_default()
    });
    let user_id = user.read().clone().id;

    if !(id.len() == 0 || id == "new".to_string() || user_id == id) {
        spawn(async move {
            let u = get_user_by_id(id).await.unwrap_or_default();
            user.set(u);
        });
    }

    rsx!(
        div { class: "flex flex-col gap-4 max-w-3xl",
            div { class: "flex flex-wrap items-center justify-between gap-2",
                h1 { class: "text-2xl font-bold",
                    if user.read().id.is_empty() { "New User" } else { "Edit User" }
                }
                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| {
                        navigator().push(Route::UserList {});
                    },
                    "Back to list"
                }
            }

            div { class: "card bg-base-100 card-border",
                div { class: "card-body",
                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", "Account" }
                        label { class: "label", "Username" }
                        input {
                            class: "input w-full",
                            r#type: "text",
                            placeholder: "Username",
                            value: "{user.read().username}",
                            oninput: move |e| {
                                let mut t = user.read().clone();
                                t.username = e.value().clone();
                                user.set(t);
                            },
                        }
                        label { class: "label mt-2", "Password" }
                        input {
                            class: "input w-full",
                            r#type: "password",
                            placeholder: "Leave empty to keep current password",
                            value: "{password.read()}",
                            oninput: move |e| {
                                password.set(e.value().clone());
                            },
                        }
                        p { class: "label text-base-content/60",
                            "Only set a password when creating a user or changing it."
                        }
                    }

                    div { class: "grid gap-4 md:grid-cols-2",
                        fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                            legend { class: "fieldset-legend", "Skin" }
                            select {
                                class: "select w-full",
                                value: "{user.read().selected_skin_id.clone().unwrap_or_else(|| \"None\".to_string())}",
                                onchange: move |e| {
                                    let mut t = user.read().clone();
                                    let mut id: Option<String> = Some(e.value().clone());
                                    if id == Some("None".to_string()) {
                                        id = None;
                                    }
                                    t.selected_skin_id = id;
                                    user.set(t);
                                },
                                option { value: "None", "None" }
                                for skin in skins.cloned().unwrap_or_default() {
                                    option {
                                        value: "{skin.id}",
                                        selected: user.read().selected_skin_id == Some(skin.id.clone()),
                                        "{skin.skin_name}"
                                    }
                                }
                            }
                            {
                                match skins
                                    .cloned()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .find(|s| Some(s.id.clone()) == user.read().selected_skin_id)
                                {
                                    Some(selected) => rsx! {
                                        div { class: "avatar mt-3 justify-center",
                                            div { class: "w-24 rounded",
                                                img {
                                                    src: "data:image/png;base64,{selected.get_preview().unwrap_or_default().as_base64()}",
                                                    alt: "{selected.skin_name}",
                                                }
                                            }
                                        }
                                    },
                                    None => rsx! {
                                        p { class: "mt-3 text-sm text-base-content/60",
                                            "No skin selected."
                                        }
                                    },
                                }
                            }
                        }
                        fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                            legend { class: "fieldset-legend", "Cape" }
                            select {
                                class: "select w-full",
                                value: "{user.read().selected_cape_id.clone().unwrap_or_else(|| \"None\".to_string())}",
                                onchange: move |e| {
                                    let mut t = user.read().clone();
                                    let mut id: Option<String> = Some(e.value().clone());
                                    if id == Some("None".to_string()) {
                                        id = None;
                                    }
                                    t.selected_cape_id = id;
                                    user.set(t);
                                },
                                option { value: "None", "None" }
                                for cape in capes.cloned().unwrap_or_default() {
                                    option {
                                        value: "{cape.id}",
                                        selected: user.read().selected_cape_id == Some(cape.id.clone()),
                                        "{cape.skin_name}"
                                    }
                                }
                            }
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", "Elytra" }
                        select {
                            class: "select w-full",
                            value: "{user.read().selected_elytra_id.clone().unwrap_or_else(|| \"None\".to_string())}",
                            onchange: move |e| {
                                let mut t = user.read().clone();
                                let mut id: Option<String> = Some(e.value().clone());
                                if id == Some("None".to_string()) {
                                    id = None;
                                }
                                t.selected_elytra_id = id;
                                user.set(t);
                            },
                            option { value: "None", "None" }
                            for elytrum in elytra.cloned().unwrap_or_default() {
                                option {
                                    value: "{elytrum.id}",
                                    selected: user.read().selected_elytra_id == Some(elytrum.id.clone()),
                                    "{elytrum.skin_name}"
                                }
                            }
                        }
                    }

                    div { class: "card-actions justify-end",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| {
                                navigator().push(Route::UserList {});
                            },
                            "Cancel"
                        }
                        if !user.read().id.is_empty() {
                            button {
                                class: "btn btn-error btn-outline",
                                "onclick": "del_user_modal.showModal()",
                                "Delete"
                            }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                async move {
                                    let nav = navigator();
                                    let t = user.read().clone();
                                    let pass = password.read().clone();
                                    api::create_user(t, pass).await.unwrap();
                                    nav.push(Route::UserList {});
                                }
                            },
                            "Save"
                        }
                    }
                }
            }

            dialog { class: "modal", id: "del_user_modal",
                div { class: "modal-box",
                    h3 { class: "text-lg font-bold", "Delete user?" }
                    p { class: "py-4",
                        "Delete user \"{user.read().username}\"? This cannot be undone."
                    }
                    div { class: "modal-action",
                        form { method: "dialog",
                            button { class: "btn", "Abort" }
                            button {
                                class: "btn btn-error",
                                onclick: move |evt| {
                                    evt.prevent_default();
                                    async move {
                                        let nav = navigator();
                                        let t = user.read().clone();
                                        api::del_user_by_id(t.id).await.unwrap();
                                        nav.push(Route::UserList {});
                                    }
                                },
                                "Delete"
                            }
                        }
                    }
                }
                form { method: "dialog", class: "modal-backdrop",
                    button { "close" }
                }
            }
        }
    )
}
