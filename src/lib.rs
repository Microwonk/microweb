#![recursion_limit = "256"]
#![feature(impl_trait_in_assoc_type)]

pub mod apps;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use common::Apps;

    use crate::apps::HydrateApps;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    Apps::hydrate();
}
