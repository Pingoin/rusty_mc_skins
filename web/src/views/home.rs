use api::{User, create_user, get_my_texture_type, login, logout};
use dioxus::prelude::*;

use crate::{USER, reload_me};

#[component]
pub fn Home() -> Element {
    rsx! {
        article {
            {
                if USER.cloned().anonymous() {

                    rsx! {
                        LoginCard {}
                    }
                } else {
                    rsx! {
                        UserCard {}
                    }
                }
            }
        }
    }
}

#[component]
fn UserCard() -> Element {
    let skin =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Skin).await.ok() });
    let cape =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Cape).await.ok() });
    let elytra =
        use_resource(|| async move { get_my_texture_type(api::TextureType::Elytra).await.ok() });
    rsx! {
        div { class: "card card-border bg-base-100 w-96",
            div { class: "card-body",
                h2 { class: "card-title", "Ich" }
                {
                    if let Some(Some(skin)) = skin.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                {
                    if let Some(Some(skin)) = elytra.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                {
                    if let Some(Some(skin)) = cape.cloned() {
                        rsx! {
                            img {
                                src: "data:image/png;base64,{skin.get_preview().unwrap_or_default().as_base64()}",
                                width: "100",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                div { class: "card-actions justify-end",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| async move {
                            let _ = logout().await;
                            reload_me();
                        },
                        {"Logout"}
                    }
                }
            }
        }
    }
}

#[component]
fn LoginCard() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut password2 = use_signal(|| "".to_string());
    let mut register = use_signal(|| false);
    rsx! {
        div { class: "card card-border bg-base-100 w-96",
            div { class: "card-body",
                h2 { class: "card-title", "Login/Register" }

                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend", "Login" }
                    label { class: "label",
                        "Username"
                        input {
                            class: "input",
                            placeholder: "UserName",
                            r#type: "text",
                            value: username,
                            oninput: move |evt| username.set(evt.value()),
                        }
                    }

                    label { class: "label",
                        "Password"
                        input {
                            class: "input",
                            placeholder: "Passoword",
                            r#type: "password",
                            value: password,
                            oninput: move |evt| password.set(evt.value()),
                        }
                    }
                    {
                        if register() {
                            rsx! {
                                label { class: "label",
                                    "Repeat Password"
                                    input {
                                        class: "input",
                                        placeholder: "Name",
                                        r#type: "password",
                                        value: password2,
                                        oninput: move |evt| password2.set(evt.value()),
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                    label { class: "label",
                        "Register"
                        input {
                            checked: "checked",
                            class: "toggle",
                            r#type: "checkbox",
                            value: register,
                            oninput: move |evt| register.set(evt.checked()),
                        }
                    
                    }
                
                }
                div { class: "card-actions justify-end",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| async move {
                            login_register(username(), password(), password2(), register()).await;
                            reload_me();

                        },
                        {if register() { "register" } else { "login" }}
                    }
                }
            }
        }
    }
}

async fn login_register(user_name: String, password1: String, password2: String, register: bool) {
    if register {
        if password1 == password2 {
            let mut user = User::default();
            user.username = user_name.clone();
            let _ = create_user(user.clone(), password1.clone()).await;
            let _ = login(user_name, password1).await;
        } else {
        }
    } else {
        let _ = login(user_name, password1).await;
    }
}
