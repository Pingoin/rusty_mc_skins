use api::TextureType;
use dioxus::prelude::*;

use crate::WebNavbar;
mod group_edit;
mod group_list;
mod home;
mod navigation;
mod texture_edit;
mod texture_list;
mod user_edit;
mod user_list;

use group_edit::GroupEdit;
use group_list::GroupList;
use home::Home;
pub use navigation::NavItems;
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
    #[route("/texture/:tex_type/list")]
    TextureList {tex_type:TextureType},
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
