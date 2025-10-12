use api::{get_user_by_id, User};
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn UserEdit(id: String) -> Element {
    let mut user = use_signal(|| User::default());

    let user_id = user.read().clone().id;

    if !(id.len() == 0 || id == "new".to_string() || user_id == id) {
        spawn(async move {
            let u = get_user_by_id(id).await.unwrap_or_default();
            user.set(u);
        });
    }

    rsx!(
        article {
            h1 { "User" }
            input {
                value: "{user.read().username}",
                oninput: move |e| {
                    let mut t = user.read().clone();
                    t.username = e.value().clone();
                    user.set(t);
                }
            }
            input {
                value:"password",
                oninput: move |e| {
                    let mut t = user.read().clone();
                    async move{
                        let pwh=api::hash_password(e.value().clone()).await.unwrap_or_default();
                        t.password_hash = pwh;
                        user.set(t);
                    }

                }
            }

            button {
                onclick: move |_| {
                    async move {
                        let nav = navigator();
                    let t = user.read().clone();
                    api::create_user(t).await.unwrap();
                    nav.push(Route::UserList{});

                }

                },
                "Save"
            }
            button {
                onclick: move|_|{
                    async move {
                        let nav = navigator();
                    let t = user.read().clone();
                    api::del_user_by_id(t).await.unwrap();
                    nav.push(Route::UserList{});

                }
                },
                "delete"
            }
        }
    )
}
