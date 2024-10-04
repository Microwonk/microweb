use leptos::*;
use leptos_router::use_navigate;

use crate::util::Api;

#[component]
pub fn LogOut(set_logged_in: WriteSignal<bool>) -> impl IntoView {
    spawn_local(async move {
        Api::logout().await;
        set_logged_in(false);
    });
    view! {
        {move || {
            let navigate = use_navigate();
            navigate("/", Default::default());
        }}
    }
}
