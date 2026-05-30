use crate::views::Route;
use api::{Group, Permissions, User, get_group_by_id, get_users};
use dioxus::prelude::*;

#[component]
pub fn GroupEdit(id: String) -> Element {
    let mut group = use_signal(|| Group::default());
    let mut users = use_signal(|| Vec::<User>::new());
    let group_id = group.read().clone().id;

    if !(id.len() == 0 || id == "new".to_string() || group_id == id) {
        spawn(async move {
            let u = get_group_by_id(id).await.unwrap_or_default();
            group.set(u);
            let user_list = get_users().await.unwrap_or_default();
            users.set(user_list)
        });
    }

    rsx!(
        article {
            h1 { "Group" }
            input {
                value: "{group.read().group_name}",
                oninput: move |e| {
                    let mut t = group.read().clone();
                    t.group_name = e.value().clone();
                    group.set(t);
                },
            }
            div {
                for permission in Permissions::all().iter() {
                    input {
                        r#type: "checkbox",
                        checked: "{group.read().permissions.contains(permission.clone())}",
                        onchange: move |val| {
                            let mut t = group.read().clone();
                            t.permissions.set(permission.clone(), val.value() == "true".to_string());
                            group.set(t);
                        },
                    }
                    label { "{permission.to_str()}" }
                    br {}
                }
            }
            for user in users.read().clone() {
                p { "{user.username}" }
            }
            button {
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
            button {
                onclick: move |_| {
                    async move {
                        let nav = navigator();
                        let t = group.read().clone();
                        api::del_group_by_id(t.id).await.unwrap();
                        nav.push(Route::GroupList {});
                    }
                },
                "delete"
            }
        }
    )
}
