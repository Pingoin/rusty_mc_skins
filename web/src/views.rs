use dioxus::prelude::*;

use crate::WebNavbar;
mod home;
mod texture_edit;
mod texture_list;
mod user_edit;
mod user_list;

use home::Home;
use texture_edit::TextureEdit;
use texture_list::TextureList;
use user_edit::UserEdit;
use user_list::UserList;

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
}

#[component]
pub fn NavItems() -> Element {
    rsx! {
        li{Link {
            to: Route::Home {},
            "Home"
        }}
        li{Link {
            to: Route::TextureList {},
            "Textures"
        }}
        li { Link {
            to: Route::TextureEdit { id: "new".to_string() },
            "New Texture"
        }}
        li{Link {
            to: Route::UserList  {},
            "User"
        }}
        li { Link {
            to: Route::UserEdit { id: "new".to_string() },
            "New User"
        }}
    }
}
