#![recursion_limit = "256"]
#![feature(impl_trait_in_assoc_type)]

use leptos::{prelude::ServerFnError, server};

pub mod apps;
pub mod auth;
pub mod blog;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod www;

#[cfg(feature = "ssr")]
pub static DOMAIN: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| std::env::var("DOMAIN").expect("Env var DOMAIN must be set."));

#[server(Domain, "/api", "GetJson", endpoint = "domain")]
#[tracing::instrument]
pub async fn domain() -> Result<String, ServerFnError> {
    Ok(DOMAIN.clone())
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use apps::Apps;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    Apps::hydrate();
}
