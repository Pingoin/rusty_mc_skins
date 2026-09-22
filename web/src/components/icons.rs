use dioxus::prelude::*;

// Assets werden über `asset!` gebundelt – garantiert dass die SVG-Files im Build vorhanden sind
const _EYE_ASSET: Asset = asset!("/assets/icons/eye.svg");
const _EYE_SLASH_ASSET: Asset = asset!("/assets/icons/eye-slash.svg");
const _FLAG_DE_ASSET: Asset = asset!("/assets/icons/flag-de.svg");
const _FLAG_GB_ASSET: Asset = asset!("/assets/icons/flag-gb.svg");
const _LANGUAGE_ASSET: Asset = asset!("/assets/icons/language.svg");

/// Eye icon (password visible) – SVG-Daten liegen in `web/assets/icons/eye.svg`
/// Inline gerendert damit `stroke="currentColor"` das daisyUI `text-base-content` erbt.
#[component]
pub fn EyeIcon() -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-5 items-center justify-center [&>svg]:h-5 [&>svg]:w-5",
            dangerous_inner_html: include_str!("../../assets/icons/eye.svg"),
        }
    }
}

/// Eye-slash icon (password hidden) – SVG-Daten liegen in `web/assets/icons/eye-slash.svg`
#[component]
pub fn EyeSlashIcon() -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-5 items-center justify-center [&>svg]:h-5 [&>svg]:w-5",
            dangerous_inner_html: include_str!("../../assets/icons/eye-slash.svg"),
        }
    }
}

/// German flag – `web/assets/icons/flag-de.svg`
#[component]
pub fn FlagDeIcon() -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-7 items-center justify-center overflow-hidden rounded-sm border border-base-300 [&>svg]:h-5 [&>svg]:w-7",
            dangerous_inner_html: include_str!("../../assets/icons/flag-de.svg"),
        }
    }
}

/// British flag (English) – `web/assets/icons/flag-gb.svg`
#[component]
pub fn FlagGbIcon() -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-7 items-center justify-center overflow-hidden rounded-sm border border-base-300 [&>svg]:h-5 [&>svg]:w-7",
            dangerous_inner_html: include_str!("../../assets/icons/flag-gb.svg"),
        }
    }
}

/// Generic language/globe icon – `web/assets/icons/language.svg`
#[component]
pub fn LanguageIcon() -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-5 items-center justify-center [&>svg]:h-5 [&>svg]:w-5",
            dangerous_inner_html: include_str!("../../assets/icons/language.svg"),
        }
    }
}
