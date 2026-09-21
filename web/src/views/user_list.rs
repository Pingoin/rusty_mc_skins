use api::get_users;
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn UserList() -> Element {
    let users = use_resource(|| async move { get_users().await.unwrap_or_default() });
    rsx! {

        div { class: "flex flex-col gap-4",
            div { class: "flex flex-wrap items-center justify-between gap-2",
                h1 { class: "text-2xl font-bold", "User List" }
                button {
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        let nav = navigator();
                        nav.push(Route::UserEdit { id: "new".to_string() });
                    },
                    "New User"
                }
            }

            div { class: "card bg-base-100 card-border",
                div { class: "card-body p-0 sm:p-2",
                    match users.cloned() {
                        None => rsx! {
                            div { class: "flex justify-center p-8",
                                span { class: "loading loading-spinner loading-lg" }
                            }
                        },
                        Some(list) => rsx! {
                            if list.is_empty() {
                                p { class: "p-6 text-center text-base-content/60",
                                    "No users found."
                                }
                            } else {
                                div { class: "overflow-x-auto",
                                    table { class: "table table-zebra",
                                        thead {
                                            tr {
                                                th { "Username" }
                                                th { "ID" }
                                                th { class: "text-right", "Actions" }
                                            }
                                        }
                                        tbody {
                                            for user in list {
                                                tr {
                                                    td {
                                                        span { class: "font-medium",
                                                            "{user.username}"
                                                        }
                                                    }
                                                    td {
                                                        span { class: "badge badge-ghost badge-sm font-mono",
                                                            "{user.id}"
                                                        }
                                                    }
                                                    td { class: "text-right",
                                                        button {
                                                            class: "btn btn-ghost btn-sm",
                                                            onclick: move |_| {
                                                                let user = user.clone();
                                                                let nav = navigator();
                                                                nav.push(Route::UserEdit {
                                                                    id: user.clone().id.clone(),
                                                                });
                                                            },
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
