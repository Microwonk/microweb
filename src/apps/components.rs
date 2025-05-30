use chrono::Datelike;
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

#[component(transparent)]
pub fn Footer() -> impl IntoView {
    let copyright = format!(
        "© Copyright 2024 - {}, Nicolas Frey, All rights reserved",
        chrono::Utc::now().year()
    );

    let (socials, _set_socials) = signal(vec![
        (
            "instagram".to_string(),
            "https://www.instagram.com/nic_ol_ass".to_string(),
        ),
        (
            "bluesky".to_string(),
            "https://bsky.app/profile/nicolas-frey.com".to_string(),
        ),
        (
            "youtube".to_string(),
            "https://www.youtube.com/@microwonk".to_string(),
        ),
        (
            "github".to_string(),
            "https://www.github.com/Microwonk".to_string(),
        ),
        (
            "itch-io".to_string(),
            "https://microwonk.itch.io".to_string(),
        ),
        (
            "discord".to_string(),
            "https://discordapp.com/users/444924590913749002".to_string(),
        ),
    ]);

    view! {
        <footer class="bg-nf-dark md:flex md:items-center md:justify-between shadow rounded-lg p-4 md:p-6 xl:p-8 mt-4 mb-2 mx-2 w-full">
            <ul class="flex items-center flex-wrap mb-6 md:mb-0">
                <li class="text-sm font-normal text-nf-white mr-4 md:mr-6">{copyright}</li>
                <li>
                    <a
                        href=move || format!("{}/privacy-policy", Apps::Www.url())
                        class="text-sm font-normal text-nf-white hover:underline mr-4 md:mr-6"
                    >
                        Privacy Policy
                    </a>
                </li>
                <li>
                    <a
                        href=move || format!("{}/impress", Apps::Www.url())
                        class="text-sm font-normal text-nf-white hover:underline mr-4 md:mr-6"
                    >
                        Impress
                    </a>
                </li>
                <li>
                    <a
                        href="mailto:contact@nicolas-frey.com"
                        class="text-sm font-normal text-nf-white hover:underline"
                    >
                        Contact
                    </a>
                </li>
            </ul>
            <div class="flex sm:justify-center space-x-6 bg-nf-color py-3 rounded-md">
                <For each=socials key=|state| state.0.clone() let:child>
                    <a href=child.1 target="_blank" class="hover:animate-pulse">
                        <img
                            src=move || { format!("/assets/{}.svg", child.0) }
                            class="w-[24px] mx-4"
                        />
                    </a>
                </For>
            </div>
        </footer>
    }
}
