use crate::components::use_case::UseCase;

use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;

#[component]
pub fn UseCasesPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap());

    view! {
        <Title text=format!("Nicolas Frey - {}", slug())/>
        <UseCase slug=slug()/>
    }
}
