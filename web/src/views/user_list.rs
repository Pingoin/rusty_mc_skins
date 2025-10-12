use api::get_users;
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn UserList() -> Element {
    let users = use_resource(|| async move { get_users().await.unwrap_or_default() });
    rsx! {

        article {
            h1 { "User List" }
            ul {
                for user in users.cloned().unwrap_or_default(){
                    li {
                        "{user.username}"
                        button {
                            onclick:move |_| {
                                let user=user.clone();
                                let nav = navigator();
                                nav.push(Route::UserEdit{id:user.clone().id.clone()});
                            },
                            "Edit"
                        }
                    }
                }
            }
        }
    }
}
