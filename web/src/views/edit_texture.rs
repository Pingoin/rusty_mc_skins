use dioxus::prelude::*;

#[component]
pub fn EditTexture(id: String) -> Element {
    rsx! {
        article {
            h1 { "Edit Texture" }
            p { "Texture ID: {id}" }
        }
    }
}