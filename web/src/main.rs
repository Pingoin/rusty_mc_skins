use dioxus::prelude::*;
use components::Navbar;
use crate::views::{NavItems, Route};

mod views;
mod components;



const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");

fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::launch(App);

    // Launch axum on the server
    #[cfg(feature = "server")]
    {
        api::init(App);
    }  
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        header {  }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {  
            NavItems {}
        }
        main { Outlet::<Route> {} }
        
        footer {  }
    }
}
