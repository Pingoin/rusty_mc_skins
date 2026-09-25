use api::{AppError, User, create_user, login};
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::components::{EyeIcon, EyeSlashIcon};
use crate::{reload_me, show_alert};

#[component]
pub fn LoginCard() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut password2 = use_signal(|| "".to_string());
    let mut register = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut show_password = use_signal(|| false);
    let mut show_password2 = use_signal(|| false);

    // derived validation (cloned so we don't hold borrows across renders)
    let username_trimmed = username.read().trim().to_string();
    let username_valid = !username_trimmed.is_empty();
    let password_val = password.read().clone();
    let password2_val = password2.read().clone();
    let is_register = register();
    let passwords_match = password_val == password2_val;
    let password2_touched = !password2_val.is_empty();
    let password_weak = is_register && !password_val.is_empty() && password_val.len() < 4;

    let can_submit = if is_register {
        username_valid
            && !password_val.is_empty()
            && !password_weak
            && password2_touched
            && passwords_match
            && !loading()
    } else {
        username_valid && !password_val.is_empty() && !loading()
    };

    // inline hints for mismatch
    let repeat_hint = if is_register && password2_touched {
        if passwords_match {
            Some((true, tid!("login-error-passwords-match")))
        } else {
            Some((false, tid!("login-error-passwords-mismatch")))
        }
    } else {
        None
    };

    // pre-translate alert messages during render so handlers can capture them
    let msg_empty_user = tid!("login-error-empty-username").to_string();
    let msg_empty_pass = tid!("login-error-empty-password").to_string();
    let msg_mismatch = tid!("login-error-passwords-mismatch").to_string();
    let msg_invalid = tid!("login-error-invalid-credentials").to_string();
    let msg_taken = tid!("login-error-username-taken").to_string();
    let msg_weak = tid!("login-error-weak-password").to_string();
    let msg_generic = tid!("login-error-generic").to_string();

    rsx! {
        div { class: "card card-border bg-base-100 w-full max-w-96 shadow-sm",
            div { class: "card-body",
                h2 { class: "card-title", {tid!("login-title")} }
                form {
                    novalidate: true,
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        if loading() {
                            return;
                        }
                        let u = username.read().trim().to_string();
                        let p = password.read().clone();
                        let p2 = password2.read().clone();
                        let r = register();

                        let msg_empty_user = msg_empty_user.clone();
                        let msg_empty_pass = msg_empty_pass.clone();
                        let msg_mismatch = msg_mismatch.clone();
                        let msg_invalid = msg_invalid.clone();
                        let msg_taken = msg_taken.clone();
                        let msg_weak = msg_weak.clone();
                        let msg_generic = msg_generic.clone();

                        if u.is_empty() {
                            show_alert(msg_empty_user);
                            return;
                        }
                        if p.is_empty() {
                            show_alert(msg_empty_pass);
                            return;
                        }
                        if r {
                            if p != p2 {
                                show_alert(msg_mismatch);
                                return;
                            }
                            if p.len() < 4 {
                                show_alert(msg_weak);
                                return;
                            }
                        }

                        loading.set(true);
                        spawn(async move {
                            if r {
                                // register flow: create then login
                                let mut user = User::default();
                                user.username = u.clone();
                                match create_user(user, p.clone()).await {
                                    Ok(_) => {
                                        match login(u.clone(), p.clone()).await {
                                            Ok(_) => {
                                                reload_me();
                                            }
                                            Err(e) => {
                                                let msg = match e {
                                                    AppError::InvalidCredentials | AppError::WrongPassword => msg_invalid.clone(),
                                                    AppError::UsernameTaken => msg_taken.clone(),
                                                    AppError::WeakPassword => msg_weak.clone(),
                                                    AppError::PasswordMismatch => msg_mismatch.clone(),
                                                    _ => format!("{msg_generic}: {e}"),
                                                };
                                                show_alert(msg);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let msg = match e {
                                            AppError::UsernameTaken => msg_taken.clone(),
                                            AppError::WeakPassword => msg_weak.clone(),
                                            AppError::InvalidCredentials => msg_invalid.clone(),
                                            AppError::PasswordMismatch => msg_mismatch.clone(),
                                            _ => format!("{msg_generic}: {e}"),
                                        };
                                        show_alert(msg);
                                    }
                                }
                            } else {
                                match login(u.clone(), p.clone()).await {
                                    Ok(_) => {
                                        reload_me();
                                    }
                                    Err(e) => {
                                        let msg = match e {
                                            AppError::InvalidCredentials | AppError::WrongPassword => msg_invalid.clone(),
                                            AppError::WeakPassword => msg_weak.clone(),
                                            _ => format!("{msg_generic}: {e}"),
                                        };
                                        show_alert(msg);
                                    }
                                }
                            }
                            loading.set(false);
                        });
                    },
                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                        legend { class: "fieldset-legend", {tid!("login-legend")} }

                        // username
                        label { class: "label", {tid!("login-username")} }
                        input {
                            class: if !username_valid && !username.read().is_empty() { "input input-error w-full" } else { "input w-full" },
                            placeholder: "{tid!(\"login-username-placeholder\")}",
                            r#type: "text",
                            name: "username",
                            autocomplete: "username",
                            required: true,
                            disabled: loading(),
                            value: username,
                            oninput: move |evt| username.set(evt.value()),
                        }

                        // password with icon toggle inside input (daisyUI input wrapper + btn)
                        label { class: "label mt-2", {tid!("login-password")} }
                        label { class: if password_weak { "input input-warning w-full" } else { "input w-full" },
                            input {
                                class: "grow",
                                placeholder: "{tid!(\"login-password-placeholder\")}",
                                r#type: if show_password() { "text" } else { "password" },
                                name: "password",
                                autocomplete: if is_register { "new-password" } else { "current-password" },
                                required: true,
                                disabled: loading(),
                                value: password,
                                oninput: move |evt| password.set(evt.value()),
                            }
                            button {
                                class: "btn btn-ghost btn-xs btn-circle",
                                r#type: "button",
                                tabindex: -1,
                                disabled: loading(),
                                aria_label: if show_password() { tid!("login-hide-password").to_string() } else { tid!("login-show-password").to_string() },
                                title: if show_password() { tid!("login-hide-password").to_string() } else { tid!("login-show-password").to_string() },
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() {
                                    EyeSlashIcon {}
                                } else {
                                    EyeIcon {}
                                }
                            }
                        }
                        if password_weak {
                            p { class: "label text-warning text-xs", {tid!("login-error-weak-password")} }
                        }

                        // repeat password (only in register mode)
                        if is_register {
                            label { class: "label mt-2", {tid!("login-repeat-password")} }
                            label { class: if password2_touched && !passwords_match { "input input-error w-full" } else if password2_touched && passwords_match { "input input-success w-full" } else { "input w-full" },
                                input {
                                    class: "grow",
                                    placeholder: "{tid!(\"login-repeat-password-placeholder\")}",
                                    r#type: if show_password2() { "text" } else { "password" },
                                    name: "password2",
                                    autocomplete: "new-password",
                                    required: true,
                                    disabled: loading(),
                                    value: password2,
                                    oninput: move |evt| password2.set(evt.value()),
                                }
                                button {
                                    class: "btn btn-ghost btn-xs btn-circle",
                                    r#type: "button",
                                    tabindex: -1,
                                    disabled: loading(),
                                    aria_label: if show_password2() { tid!("login-hide-password").to_string() } else { tid!("login-show-password").to_string() },
                                    title: if show_password2() { tid!("login-hide-password").to_string() } else { tid!("login-show-password").to_string() },
                                    onclick: move |_| show_password2.set(!show_password2()),
                                    if show_password2() {
                                        EyeSlashIcon {}
                                    } else {
                                        EyeIcon {}
                                    }
                                }
                            }
                            if let Some((is_match, hint)) = repeat_hint {
                                p { class: if is_match { "label text-success text-xs" } else { "label text-error text-xs" },
                                    "{hint}"
                                }
                            } else {
                                p { class: "label text-base-content/60 text-xs", {tid!("login-hint-password-repeat")} }
                            }
                        }

                        // register toggle
                        label { class: "label mt-3 cursor-pointer justify-start gap-3",
                            input {
                                checked: register(),
                                class: "toggle toggle-primary",
                                r#type: "checkbox",
                                disabled: loading(),
                                oninput: move |evt| {
                                    let checked = evt.checked();
                                    register.set(checked);
                                    if !checked {
                                        // optional: clear repeat when switching back to login
                                        password2.set("".to_string());
                                    }
                                },
                            }
                            span { class: "label-text", {tid!("login-register")} }
                        }
                    }
                    div { class: "card-actions justify-end mt-4",
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            disabled: !can_submit,
                            aria_disabled: if !can_submit { "true" } else { "false" },
                            if loading() {
                                span { class: "loading loading-spinner loading-sm" }
                            }
                            {
                                if is_register {
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
