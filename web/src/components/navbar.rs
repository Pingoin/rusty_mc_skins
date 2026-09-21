use crate::{has_permission, i18n::LanguageSwitcher, views::Route};
use api::{Permissions, TextureType};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        //document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav { class: "flex items-center justify-between gap-2",
            ul { class: "menu menu-vertical lg:menu-horizontal bg-base-200 rounded-box",
                {children}
            }
            LanguageSwitcher {}
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
