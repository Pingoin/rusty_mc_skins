use api::get_textures;
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn TextureList() -> Element {
    let textures = use_resource(|| async move { get_textures().await.unwrap_or_default() });
    rsx! {
         
        article {
            h1 { "Texture List" }
            ul {
                for texture in textures.cloned().unwrap_or_default(){
                    li { 
                        "{texture.skin_name}" 
                        img { src: "data:image/png;base64,{texture.image_data.as_base64()}",width: "100"}
                        button {
                            onclick:move |_| {
                                let texture=texture.clone();
                                let nav = navigator();
                                nav.push(Route::EditTexture{id:texture.clone().id.clone()});
                            },
                            "edit"
                        }
                    }
                }
            }
        }
    }
}
