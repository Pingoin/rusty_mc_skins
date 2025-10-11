use dioxus::prelude::*;

use crate::WebNavbar;
mod home;
mod texture_list;
mod edit_texture;

use texture_list::TextureList;
use edit_texture::EditTexture;
use home::Home;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/texture/list")]
    TextureList {},
    #[route("/texture/:id/edit")]
    EditTexture { id: String },
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
            to: Route::EditTexture { id: "new".to_string() },
            "New Texture"
        }} 
    }
}