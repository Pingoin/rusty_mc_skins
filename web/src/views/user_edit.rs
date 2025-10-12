use api::{get_textures_by_type, get_user_by_id, SkinType, User};
use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub fn UserEdit(id: String) -> Element {
    let mut user = use_signal(|| User::default());

    let skins = use_resource(|| async move {
        get_textures_by_type(SkinType::Skin)
            .await
            .unwrap_or_default()
    });

    let capes = use_resource(|| async move {
        get_textures_by_type(SkinType::Cape)
            .await
            .unwrap_or_default()
    });
    let elytra = use_resource(|| async move {
        get_textures_by_type(SkinType::Elytra)
            .await
            .unwrap_or_default()
    });
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
            select {
                onchange: move |e|{
                    let mut t = user.read().clone();
                    let mut id:Option<String>=Some(e.value().clone());
                    if id==Some("None".to_string()){
                        id=None;
                    }
                    t.selected_skin_id=id;
                    user.set(t);
                    async{api::hash_password("heinz".to_string()).await.unwrap_or_default();}

                },
                option{value:"None", "None"}
                for skin in skins.cloned().unwrap_or_default(){
                    option { value:skin.id, "{skin.skin_name}" }
                }
            }
            select {
                onchange: move |e|{
                    let mut t = user.read().clone();
                    let mut id:Option<String>=Some(e.value().clone());
                    if id==Some("None".to_string()){
                        id=None;
                    }
                    t.selected_cape_id=id;
                    user.set(t);
                },
                option{value:"None", "None"}
                for cape in capes.cloned().unwrap_or_default(){
                    option { value:cape.id, "{cape.skin_name}" }
                }
            }
            select {
                onchange: move |e|{
                    let mut t = user.read().clone();
                    let mut id:Option<String>=Some(e.value().clone());
                    if id==Some("None".to_string()){
                        id=None;
                    }
                    t.selected_elytra_id=id;
                    user.set(t);

                },
                option{value:"None", "None"}
                for elytrum in elytra.cloned().unwrap_or_default(){
                    option { value:elytrum.id, "{elytrum.skin_name}" }
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
