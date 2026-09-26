use crate::views::Route;
use api::{Group, Permissions, get_group_by_id, get_users};
use dioxus::prelude::*;
use dioxus_i18n::tid;

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
        div { class: "flex flex-col gap-4 max-w-3xl mx-auto w-full",
            div { class: "flex flex-wrap items-center justify-between gap-2",
                h1 { class: "text-2xl font-bold",
                    if group.read().id.is_empty() {
                        {tid!("group-edit-new")}
                    } else {
                        {tid!("group-edit-edit")}
                    }
                }
                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| {
                        navigator().push(Route::GroupList {});
                    },
                    {tid!("group-edit-back")}
                }
            }

            div { class: "card bg-base-100 card-border",
                div { class: "card-body",
                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("group-edit-group")} }
                        label { class: "label", {tid!("group-edit-name")} }
                        input {
                            class: "input w-full",
                            r#type: "text",
                            placeholder: "{tid!(\"group-edit-name-placeholder\")}",
                            value: "{group.read().group_name}",
                            oninput: move |e| {
                                let mut t = group.read().clone();
                                t.group_name = e.value().clone();
                                group.set(t);
                            },
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("group-edit-permissions")} }
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
                            {tid!("group-edit-permissions-hint")}
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("group-edit-quota")} }
                        label { class: "label", {tid!("group-edit-max-textures")} }
                        input {
                            class: "input w-full",
                            r#type: "number",
                            min: "0",
                            value: "{group.read().max_textures}",
                            oninput: move |e| {
                                let mut t = group.read().clone();
                                t.max_textures = e.value().parse().unwrap_or(0).max(0);
                                group.set(t);
                            },
                        }
                        p { class: "label text-base-content/60",
                            {tid!("group-edit-max-textures-hint")}
                        }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("group-edit-members")} }
                        match users.cloned() {
                            None => rsx! {
                                div { class: "flex justify-center p-4",
                                    span { class: "loading loading-spinner loading-md" }
                                }
                            },
                            Some(list) => rsx! {
                                if list.is_empty() {
                                    p { class: "text-sm text-base-content/60", {tid!("group-edit-no-users")} }
                                } else {
                                    div { class: "flex flex-wrap gap-2",
                                        for user in list {
                                            span { class: "badge badge-ghost", "{user.username}" }
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
                            {tid!("group-edit-cancel")}
                        }
                        if !group.read().id.is_empty() {
                            button {
                                class: "btn btn-error btn-outline",
                                "onclick": "del_group_modal.showModal()",
                                {tid!("group-edit-delete")}
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
                            {tid!("group-edit-save")}
                        }
                    }
                }
            }

            dialog { class: "modal", id: "del_group_modal",
                div { class: "modal-box",
                    h3 { class: "text-lg font-bold", {tid!("group-edit-delete-title")} }
                    p { class: "py-4",
                        {tid!("group-edit-delete-message", name : group.read().group_name.clone())}
                    }
                    div { class: "modal-action",
                        form { method: "dialog",
                            button { class: "btn", {tid!("group-edit-abort")} }
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
                                {tid!("group-edit-delete")}
                            }
                        }
                    }
                }
                form { method: "dialog", class: "modal-backdrop",
                    button { {tid!("group-edit-close")} }
                }
            }
        }
    )
}
