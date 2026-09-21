use api::get_groups;
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::views::Route;

#[component]
pub fn GroupList() -> Element {
    let groups = use_resource(|| async move { get_groups().await.unwrap_or_default() });
    rsx! {

        div { class: "flex flex-col gap-4",
            div { class: "flex flex-wrap items-center justify-between gap-2",
                h1 { class: "text-2xl font-bold", {tid!("group-list-title")} }
                button {
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        let nav = navigator();
                        nav.push(Route::GroupEdit {
                            id: "new".to_string(),
                        });
                    },
                    {tid!("group-list-new")}
                }
            }

            div { class: "card bg-base-100 card-border",
                div { class: "card-body p-0 sm:p-2",
                    match groups.cloned() {
                        None => rsx! {
                            div { class: "flex justify-center p-8",
                                span { class: "loading loading-spinner loading-lg" }
                            }
                        },
                        Some(list) => rsx! {
                            if list.is_empty() {
                                p { class: "p-6 text-center text-base-content/60", {tid!("group-list-empty")} }
                            } else {
                                div { class: "overflow-x-auto",
                                    table { class: "table table-zebra",
                                        thead {
                                            tr {
                                                th { {tid!("group-list-col-name")} }
                                                th { {tid!("group-list-col-id")} }
                                                th { class: "text-right", {tid!("group-list-col-actions")} }
                                            }
                                        }
                                        tbody {
                                            for group in list {
                                                tr {
                                                    td {
                                                        span { class: "font-medium", "{group.group_name}" }
                                                    }
                                                    td {
                                                        span { class: "badge badge-ghost badge-sm font-mono", "{group.id}" }
                                                    }
                                                    td { class: "text-right",
                                                        button {
                                                            class: "btn btn-ghost btn-sm",
                                                            onclick: move |_| {
                                                                let group = group.clone();
                                                                let nav = navigator();
                                                                nav.push(Route::GroupEdit {
                                                                    id: group.clone().id.clone(),
                                                                });
                                                            },
                                                            {tid!("group-list-edit")}
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
