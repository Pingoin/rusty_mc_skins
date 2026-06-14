use api::{Permissions, TextureType};
use dioxus::prelude::*;

use crate::{has_permission, views::Route};

#[component]
pub fn NavItems() -> Element {
    rsx! {
        li {
            Link { to: Route::Home {}, "Home" }
        }
        if has_permission(Permissions::TEXTURE_USE) {
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Skin,
                    },
                    "Skins"
                }
            }
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Cape,
                    },
                    "Capes"
                }
            }
            li {
                Link {
                    to: Route::TextureList {
                        tex_type: TextureType::Elytra,
                    },
                    "Elytra"
                }
            }
        }

        li {
            Link {
                to: Route::TextureEdit {
                    id: "new".to_string(),
                },
                "New Texture"
            }
        }
        if has_permission(Permissions::USER_EDIT) {
            li {
                Link { to: Route::UserList {}, "User" }
            }

            li {
                Link {
                    to: Route::UserEdit {
                        id: "new".to_string(),
                    },
                    "New User"
                }
            }
        }
        if has_permission(Permissions::GROUP_EDIT) {
            li {
                Link { to: Route::GroupList {}, "Groups" }
            }
            li {
                Link {
                    to: Route::GroupEdit {
                        id: "new".to_string(),
                    },
                    "New Group"
                }
            }
        }

    }
}
