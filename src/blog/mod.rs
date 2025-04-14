pub mod app;
#[cfg(feature = "ssr")]
pub mod auth;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod pages;

pub const THEME_STR: &str = include_str!("peel-light.tmTheme");
