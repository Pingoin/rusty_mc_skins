use crate::{has_permission, i18n::LanguageSwitcher, views::Route};
use api::{Permissions, TextureType};
use dioxus::prelude::*;
use dioxus_i18n::tid;

const LOGO: Asset = asset!("/assets/logo-ferris.svg");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav { class: "mx-auto flex w-full max-w-5xl items-center justify-between gap-2 px-4 py-2",
            Link { to: Route::Home {}, class: "flex items-center gap-2 px-2",
                img { src: LOGO, alt: "Rusty MC Skins", class: "h-8 w-8" }
                span { class: "font-bold whitespace-nowrap hidden sm:inline", "Rusty MC Skins" }
            }
            // Horizontales Menü ab lg, darunter Hamburger-Dropdown
            ul { class: "menu menu-horizontal hidden lg:flex bg-base-200 rounded-box",
                {children.clone()}
            }
            div { class: "flex items-center gap-2",
                LanguageSwitcher {}
                div { class: "dropdown dropdown-end lg:hidden",
                    div {
                        tabindex: "0",
                        role: "button",
                        class: "btn btn-ghost btn-square",
                        aria_label: "Menu",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6h16M4 12h16M4 18h16",
                            }
                        }
                    }
                    ul {
                        tabindex: "0",
                        class: "dropdown-content menu bg-base-200 rounded-box z-10 mt-2 w-52 p-2 shadow",
                        {children}
                    }
                }
            }
        }
    }
}

#[component]
pub fn NavItems() -> Element {
    rsx! {
        li {
            Link { to: Route::Home {}, {tid!("nav-home")} }
        }
        if has_permission(Permissions::TEXTURE_USE) {
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Skin,
                    },
                    {tid!("nav-skins")}
                }
            }
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Cape,
                    },
                    {tid!("nav-capes")}
                }
            }
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Elytra,
                    },
                    {tid!("nav-elytra")}
                }
            }
        }

        if has_permission(Permissions::USER_EDIT) {
            li {
                Link { to: Route::UserList {}, {tid!("nav-users")} }
            }

            li {
                Link {
                    to: Route::UserEdit {
                        id: "new".to_string(),
                    },
                    {tid!("nav-new-user")}
                }
            }
        }
        if has_permission(Permissions::GROUP_EDIT) {
            li {
                Link { to: Route::GroupList {}, {tid!("nav-groups")} }
            }
            li {
                Link {
                    to: Route::GroupEdit {
                        id: "new".to_string(),
                    },
                    {tid!("nav-new-group")}
                }
            }
        }

    }
}
