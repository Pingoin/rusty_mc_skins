use dioxus::prelude::*;

use crate::WebNavbar;
mod home;
mod texture_edit;
mod texture_list;
mod user_edit;
mod user_list;
mod group_list;
mod group_edit;

use home::Home;
use texture_edit::TextureEdit;
use texture_list::TextureList;
use user_edit::UserEdit;
use user_list::UserList;
use group_edit::GroupEdit;
use group_list::GroupList;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/texture/list")]
    TextureList {},
    #[route("/texture/:id/edit")]
    TextureEdit { id: String },
    #[route("/user/list")]
    UserList {},
    #[route("/user/:id/edit")]
    UserEdit { id: String },
    #[route("/group/list")]
    GroupList{},
    #[route("/group/:id/edit")]
    GroupEdit{id:String},
}

#[component]
pub fn NavItems() -> Element {
    rsx! {
        li {
            Link { to: Route::Home {}, "Home" }
        }
        li {
            Link { to: Route::TextureList {}, "Textures" }
        }
        li {
            Link {
                to: Route::TextureEdit {
                    id: "new".to_string(),
                },
                "New Texture"
            }
        }
        li {
            Link { to: Route::UserList {}, "User" }
        }
        li {
            Link {
                to: Route::UserEdit {
                    id: "new".to_string(),
                },
                "New User"
            }
        }
        li {
            Link { to: Route::GroupList {}, "Groups" }
        }
        li {
            Link {
                to: Route::GroupEdit {
                    id: "new".to_string(),
                },
                "New Group"
            }
        }
    }
}
