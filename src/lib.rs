#![recursion_limit = "256"]

use leptos::{prelude::ServerFnError, server};

pub mod blog;
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
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    let hostname = leptos::prelude::window().location().hostname().unwrap();

    if hostname.starts_with("blog.") {
        leptos::mount::hydrate_body(crate::blog::app::App);
    } else if hostname.starts_with("www.") {
        leptos::mount::hydrate_body(crate::www::app::App);
    }
}
