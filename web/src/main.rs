use crate::{components::NavItems, views::Route};
use api::{Permissions, User, get_me};
use components::Navbar;
use dioxus::prelude::*;
use git_version::git_version;

mod components;
mod views;
mod plugins;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");
const GIT_VERSION: &str = git_version!();
//const MAIN_CSS: Asset = asset!("/assets/main.scss");

pub(crate) static USER: GlobalSignal<User> = Signal::global(|| User::default());

pub(crate) fn reload_me() {
    spawn(async move {
        let user = get_me().await.ok();
        if let Some(user) = user {
            *USER.write() = user;
        }
    });
}

pub(crate) fn has_permission(perm: Permissions) -> bool {
    USER.cloned().has_permission(perm)
}

fn main() {
    // Run `serve()` on the server only
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new router for our app using the `router` function
        let router = api::get_router(App).await?;
        // And then return the router
        Ok(router)
    });

    // When not on the server, just run `launch()` like normal
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

static ALERT_TEXT: GlobalSignal<String> = Signal::global(|| "ALERT TEXT".to_string());

#[component]
fn App() -> Element {
    // Build cool things ✌️
    reload_me();
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        //document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND }
        div { class: "bg-base-200",
            header {}

            Router::<Route> {}
            footer {
                "Version: "
                {GIT_VERSION}
            }
        }
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar { NavItems {} }
        main { Outlet::<Route> {} }

        dialog { class: "modal", id: "Alert",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Hello!" }
                p { class: "py-4", {ALERT_TEXT.read().clone()} }
            }
            form { class: "modal-backdrop", method: "dialog",
                button { "close" }
            }
        }
        //footer { "test" }
    }
}

pub fn show_alert(message: String) {
    document::eval("document.getElementById('Alert').open()");
    ALERT_TEXT.write().replace_range(.., &message);
}
