use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        article {
            h2 { "Test" }
        }
    }
}
