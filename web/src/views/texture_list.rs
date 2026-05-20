use api::get_textures;
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn TextureList() -> Element {
    let textures = use_resource(|| async move { get_textures().await.unwrap_or_default() });
    rsx! {

        article {
            h1 { "Texture List" }
            ul { class: "list bg-base-100 rounded-box shadow-md",
                for texture in textures.cloned().unwrap_or_default() {
                    li { class: "list-row",

                        img {
                            src: "data:image/png;base64,{texture.get_preview().unwrap_or_default().as_base64()}",
                            width: "100",
                        }
                        "{texture.skin_name}"
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let texture = texture.clone();
                                let nav = navigator();
                                nav.push(Route::TextureEdit {
                                    id: texture.clone().id.clone(),
                                });
                            },
                            "edit"
                        }
                    }
                }
            }
        }
    }
}
