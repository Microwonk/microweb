use leptos::*;
use leptos_router::use_navigate;

pub mod header;

#[component]
pub fn ReRouter(route: &'static str) -> impl IntoView {
    view! {
        {move || {
            let navigate = use_navigate();
            navigate(route, Default::default());
        }}
    }
}
