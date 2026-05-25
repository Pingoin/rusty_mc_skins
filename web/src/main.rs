use crate::views::{NavItems, Route};
use components::Navbar;
use dioxus::prelude::*;

mod components;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");
//const MAIN_CSS: Asset = asset!("/assets/main.scss");


fn main() {
    // Run `serve()` on the server only
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new router for our app using the `router` function
        let router=api::get_router(App).await?;
        // And then return the router
        Ok(router)
    });

    // When not on the server, just run `launch()` like normal
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        //document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND }
        div { class: "bg-base-200 h-screen",
            header {}

            Router::<Route> {}
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

        footer { "test" }
    }
}
