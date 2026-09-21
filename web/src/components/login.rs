use api::login;
use dioxus::prelude::*;
use dioxus_i18n::tid;

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
                h2 { class: "card-title", {tid!("login-title")} }
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
                        legend { class: "fieldset-legend", {tid!("login-legend")} }
                        label { class: "label",
                            {tid!("login-username")}
                            input {
                                class: "input",
                                placeholder: "{tid!(\"login-username-placeholder\")}",
                                r#type: "text",
                                name: "username",
                                autocomplete: "username",
                                value: username,
                                oninput: move |evt| username.set(evt.value()),
                            }
                        }

                        label { class: "label",
                            {tid!("login-password")}
                            input {
                                class: "input",
                                placeholder: "{tid!(\"login-password-placeholder\")}",
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
                                        {tid!("login-repeat-password")}
                                        input {
                                            class: "input",
                                            placeholder: "{tid!(\"login-repeat-password-placeholder\")}",
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
                            {tid!("login-register")}
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
                        button { class: "btn btn-primary", r#type: "submit",
                            {
                                if register() {
                                    tid!("login-button-register")
                                } else {
                                    tid!("login-button-login")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
