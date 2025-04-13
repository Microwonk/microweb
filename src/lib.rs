#![recursion_limit = "256"]

pub mod blog;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::blog::app::*;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
