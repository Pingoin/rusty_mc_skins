use api::get_textures;
use dioxus::prelude::*;

#[component]
pub fn TextureList() -> Element {
    let textures = use_resource(|| async move { get_textures().await.unwrap_or_default() });
    rsx! {
         
        article {
            h1 { "Texture List" }
            ul {
                for texture in textures.cloned().unwrap_or_default(){
                    li { "{texture.skin_name}" }
                }
            }
        }
    }
}
