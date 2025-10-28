#[cfg(feature = "back")]
pub mod api;
pub mod apps;
#[cfg(feature = "back")]
pub mod auth;
#[cfg(feature = "back")]
pub mod db;
#[cfg(feature = "back")]
pub mod trace;
pub use apps::*;
pub mod models;
pub mod ui;

pub static EMAIL: &str = "nicolas.theo.frey@gmail.com";

#[cfg(debug_assertions)]
pub static DOMAIN: &str = "nf.com:3000";
#[cfg(not(debug_assertions))]
pub static DOMAIN: &str = "nicolas-frey.com";

#[cfg(debug_assertions)]
pub static PROTOCOL: &str = "http";
#[cfg(not(debug_assertions))]
pub static PROTOCOL: &str = "https";
