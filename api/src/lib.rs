//! This crate contains all shared fullstack server functions.

#[cfg(feature = "server")]
mod db;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::get_router;

mod app_error;
mod users;
pub use users::*;

mod textures;
pub use textures::*;

mod groups;
pub use groups::*;

mod permissions;
pub use permissions::Permissions;

#[cfg(feature = "server")]
pub mod auth;
