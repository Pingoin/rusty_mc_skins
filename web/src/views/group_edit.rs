use crate::views::Route;
use api::{Group, Permissions, get_group_by_id, get_users};
use dioxus::prelude::*;

#[component]
pub fn GroupEdit(id: String) -> Element {
    let mut group = use_signal(|| Group::default());
    let users = use_resource(|| async move { get_users().await.unwrap_or_default() });
    let group_id = group.read().clone().id;

    if !(id.is_empty() || id == "new".to_string() || group_id == id) {
        spawn(async move {
            let u = get_group_by_id(id).await.unwrap_or_default();
            group.set(u);
        });
    }

    rsx!(
        div { class: "flex flex-col gap-4 max-w-3xl",
            div { class: "flex flex-wrap items-center justify-between gap-2",
                h1 { class: "text-2xl font-bold",
                    if group.read().id.is_empty() { "New Group" } else { "Edit Group" }
                }
                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| {
                        navigator().push(Route::GroupList {});
                    },
                    "Back to list"
                }
            }

            div { class: "card bg-base-100 card-border",
                div { class: "card-body",
                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", "Group" }
                        label { class: "label", "Group name" }
                        input {
                            class: "input w-full",
                            r#type: "text",
                            placeholder: "Group name",
                            value: "{group.read().group_name}",
                            oninput: move |e| {
                                let mut t = group.read().clone();
                                t.group_name = e.value().clone();
                                group.set(t);
                            },
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", "Permissions" }
                        div { class: "flex flex-col gap-2",
                            for (permission, name) in Permissions::all_named().into_iter() {
                                label { class: "label cursor-pointer justify-start gap-3",
                                    input {
                                        class: "checkbox checkbox-sm",
                                        r#type: "checkbox",
                                        checked: group.read().permissions.contains(permission.clone()),
                                        oninput: {
                                            let perm = permission.clone();
                                            move |e| {
                                                let checked = e.checked();
                                                let mut t = group.read().clone();
                                                t.permissions.set(perm.clone(), checked);
                                                group.set(t);
                                            }
                                        },
                                    }
                                    span { class: "label-text font-mono", "{name}" }
                                }
                            }
                        }
                        p { class: "label text-base-content/60",
                            "Select which permissions members of this group have."
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", "Members" }
                        match users.cloned() {
                            None => rsx! {
                                div { class: "flex justify-center p-4",
                                    span { class: "loading loading-spinner loading-md" }
                                }
                            },
                            Some(list) => rsx! {
                                if list.is_empty() {
                                    p { class: "text-sm text-base-content/60",
                                        "No users found."
                                    }
                                } else {
                                    div { class: "flex flex-wrap gap-2",
                                        for user in list {
                                            span { class: "badge badge-ghost",
                                                "{user.username}"
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }

                    div { class: "card-actions justify-end",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| {
                                navigator().push(Route::GroupList {});
                            },
                            "Cancel"
                        }
                        if !group.read().id.is_empty() {
                            button {
                                class: "btn btn-error btn-outline",
                                "onclick": "del_group_modal.showModal()",
                                "Delete"
                            }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                async move {
                                    let nav = navigator();
                                    let t = group.read().clone();
                                    api::create_group(t).await.unwrap();
                                    nav.push(Route::GroupList {});
                                }
                            },
                            "Save"
                        }
                    }
                }
            }

            dialog { class: "modal", id: "del_group_modal",
                div { class: "modal-box",
                    h3 { class: "text-lg font-bold", "Delete group?" }
                    p { class: "py-4",
                        "Delete group \"{group.read().group_name}\"? This cannot be undone."
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
                                        let t = group.read().clone();
                                        api::del_group_by_id(t.id).await.unwrap();
                                        nav.push(Route::GroupList {});
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
