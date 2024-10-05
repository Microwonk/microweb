use leptos::*;
use leptos_router::use_navigate;

pub mod blog_card;
pub mod drop_down;
pub mod header;
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
