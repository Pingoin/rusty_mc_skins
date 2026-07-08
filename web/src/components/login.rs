use api::login;
use dioxus::prelude::*;

use api::create_user;

use api::User;
use crate::plugins::alert;
use crate::reload_me;
use crate::show_alert;

#[component]
pub fn LoginCard() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut password2 = use_signal(|| "".to_string());
    let mut register = use_signal(|| false);

    let login_register = move || async move {
        show_alert("penis".to_string());

        if register() {
            if password() == password2() {
                let mut user = User::default();
                user.username = username();
                let _ = create_user(user.clone(), password()).await;
                let _ = login(username(), password()).await;
            } else {
            }
        } else {
            let _ = login(username(), password()).await;
        }
    };

    rsx! {
        div { class: "card card-border bg-base-100 w-96",
            div { class: "card-body",
                h2 { class: "card-title", "Login/Register" }
                button { onclick: |_| alert("pups".to_string()), "furz" }
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
                            checked: register(),
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
                            login_register().await;
                            reload_me();

                        },
                        {if register() { "register" } else { "login" }}
                    }
                }
            }
        }
    }
}
