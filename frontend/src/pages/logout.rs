use crate::{components::ReRouter, types::Profile, util::Api};
use leptos::prelude::*;

#[component]
pub fn LogOut(
    set_logged_in: WriteSignal<bool>,
    logged_in: ReadSignal<bool>,
    set_user: WriteSignal<Option<Profile>>,
) -> impl IntoView {
    Effect::new(move || {
        Api::logout();
        set_user(None);
        set_logged_in(false);
    });

    view! {
        <Show when=move || !logged_in.get()>
            <ReRouter route="/"/>
        </Show>
    }
}
