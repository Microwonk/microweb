use leptos::*;
use leptos_router::use_navigate;

use crate::{types::Profile, util::Api};

#[component]
pub fn LogOut(
    set_logged_in: WriteSignal<bool>,
    set_user: WriteSignal<Option<Profile>>,
) -> impl IntoView {
    spawn_local(async move {
        Api::logout().await;
        set_user(None);
        set_logged_in(false);
    });
    view! {
        {move || {
            let navigate = use_navigate();
            navigate("/", Default::default());
        }}
    }
}
