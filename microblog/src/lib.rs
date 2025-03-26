pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod server;

pub const THEME_STR: &str = include_str!("peel-light.tmTheme");

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
