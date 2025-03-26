use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

pub mod blog_card;
pub mod comment;
pub mod drop_down;
pub mod header;
pub mod links;
pub mod side_menu;

#[component]
pub fn ReRouter(route: &'static str) -> impl IntoView {
    view! {
        {move || {
            let navigate = use_navigate();
            navigate(route, Default::default());
        }}
    }
}
