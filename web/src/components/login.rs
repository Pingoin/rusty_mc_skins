use api::login;
use dioxus::prelude::*;

use api::create_user;

use crate::reload_me;
use api::User;

#[component]
pub fn LoginCard() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut password2 = use_signal(|| "".to_string());
    let mut register = use_signal(|| false);

    rsx! {
        div { class: "card card-border bg-base-100 w-96",
            div { class: "card-body",
                h2 { class: "card-title", "Login/Register" }
                form {
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        let u = username();
                        let p = password();
                        let p2 = password2();
                        let r = register();
                        spawn(async move {
                            if r {
                                if p == p2 {
                                    let mut user = User::default();
                                    user.username = u.clone();
                                    let _ = create_user(user.clone(), p.clone()).await;
                                    let _ = login(u, p).await;
                                }
                            } else {
                                let _ = login(u, p).await;
                            }
                            reload_me();
                        });
                    },
                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                        legend { class: "fieldset-legend", "Login" }
                        label { class: "label",
                            "Username"
                            input {
                                class: "input",
                                placeholder: "UserName",
                                r#type: "text",
                                name: "username",
                                autocomplete: "username",
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
                                name: "password",
                                autocomplete: if register() { "new-password" } else { "current-password" },
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
                                            name: "password2",
                                            autocomplete: "new-password",
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
                    div { class: "card-actions justify-end mt-4",
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            {if register() { "register" } else { "login" }}
                        }
                    }
                }
            }
        }
    }
}
