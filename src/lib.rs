#![recursion_limit = "256"]
#![feature(impl_trait_in_assoc_type)]

pub mod apps;
pub mod auth;
pub mod blog;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod www;

#[cfg(debug_assertions)]
pub static DOMAIN: &str = dotenvy_macro::dotenv!("DOMAIN");
#[cfg(not(debug_assertions))]
pub static DOMAIN: &str = env!("DOMAIN");

#[cfg(debug_assertions)]
pub static PROTOCOL: &str = "http";
#[cfg(not(debug_assertions))]
pub static PROTOCOL: &str = "https";

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use apps::Apps;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    Apps::hydrate();
}
