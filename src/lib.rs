#![recursion_limit = "256"]

pub mod blog;
pub mod www;

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
