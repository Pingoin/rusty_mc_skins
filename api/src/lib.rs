//! This crate contains all shared fullstack server functions.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod db;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::get_router;

#[cfg(feature = "server")]
mod app_error;

mod users;
use strum_macros::{AsRefStr, EnumIter};
pub use users::*;

mod textures;
pub use textures::*;

mod groups;
pub use groups::*;
#[cfg(feature = "server")]
pub mod auth;

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
    pub selected_elytra_id: Option<String>,
    pub anonymous:bool,
    pub permissions: HashSet<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default, Serialize)]
pub struct Group {
    pub id: String,
    pub group_name: String,
    pub permissions: Permissions,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Serialize)]
pub struct Permissions(i64);

#[repr(u8)]
#[derive(EnumIter, AsRefStr, Clone, PartialEq, Debug)]
pub enum Permission {
    EditGroups = 0,
    EditOtherUser = 1,
}

impl Permissions {
    pub const ALL: [Permission; 2] = [Permission::EditGroups, Permission::EditOtherUser];

    pub fn has_permission(&self, perm: Permission) -> bool {
        (self.0 & (1 << perm as i64)) != 0
    }

    pub fn set_permission(&mut self, perm: Permission, state: bool) {
        if state {
            self.0 |= 1 << perm as i64;
        } else {
            self.0 &= !(1 << perm as i64);
        }
    }
}

impl From<i64> for Permissions {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
