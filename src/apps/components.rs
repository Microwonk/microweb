use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::{UseCookieOptions, use_cookie_with_options};

use crate::apps::Apps;

#[component]
pub fn CookiePopup() -> impl IntoView {
    let (cookie_consent, set_cookie_consent) = use_cookie_with_options::<bool, FromToStringCodec>(
        "cookie_consent",
        UseCookieOptions::default().domain(crate::DOMAIN.split(':').next().unwrap()),
    );

    view! {
        <Show when=move || !cookie_consent.get().is_some_and(|c| c)>
            <div class="fixed bottom-12 left-1/2 z-50 -translate-x-1/2 rounded-full bg-nf-white p-2 drop-shadow-2xl max-sm:w-11/12 opacity-0 animate-popup">
                <div class="flex items-center justify-between gap-6 text-sm">
                    <div class="content-left pl-4">
                        This website uses cookies. See the
                        <a
                            class="underline text-nf-color"
                            href=move || format!("{}/privacy-policy", Apps::Www.url())
                        >
                            Privacy Policy
                        </a>for more info.
                    </div>
                    <div class="content-right text-end">
                        <button
                            class="cursor-pointer rounded-full bg-nf-color px-4 py-2 text-white hover:bg-nf-dark"
                            on:click=move |_| set_cookie_consent(Some(true))
                        >
                            Accept
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
