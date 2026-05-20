use dioxus::prelude::*;

//const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.scss");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        //document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav {
            ul { class: "menu menu-vertical lg:menu-horizontal bg-base-200 rounded-box",
                {children}
            }
        }
    }
}
