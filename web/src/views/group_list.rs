use api::get_groups;
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn GroupList() -> Element {
    let groups = use_resource(|| async move { get_groups().await.unwrap_or_default() });
    rsx! {

        article {
            h1 { "Group List" }
            ul {
                for group in groups.cloned().unwrap_or_default() {
                    li {
                        "{group.group_name}"
                        button {
                            onclick: move |_| {
                                let group = group.clone();
                                let nav = navigator();
                                nav.push(Route::GroupEdit {
                                    id: group.clone().id.clone(),
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
