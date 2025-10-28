use chrono::Datelike;
use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::{UseCookieOptions, use_cookie_with_options};

use crate::{EMAIL, apps::Apps};

// credit to crate leptos_obfuscate:
pub fn obfuscate_email() -> RwSignal<String> {
    let mailto = RwSignal::new("mailto:honeypot@example.com".to_string());

    Effect::new(move |_| {
        let mail = format!("mailto:{EMAIL}");
        set_timeout(move || mailto.set(mail), core::time::Duration::from_secs(3));
    });

    mailto
}

#[component(transparent)]
pub fn ObfuscateEmailSpan() -> impl IntoView {
    let one = move || {
        EMAIL
            .split_once('@')
            .map(|(one, _)| one.chars().rev().collect::<String>())
            .unwrap_or_default()
    };

    let two = move || {
        EMAIL
            .split_once('@')
            .map(|(_, two)| two.chars().rev().collect::<String>())
            .unwrap_or_default()
    };

    view! {
        <span aria-label="E-Mail" class="obfuscate">
            {two}
            <i>"%/#"</i>
            <span></span>
            {one}
        </span>
    }
}

#[component]
pub fn CookiePopup() -> impl IntoView {
    let (cookie_consent, set_cookie_consent) = use_cookie_with_options::<bool, FromToStringCodec>(
        "cookie_consent",
        UseCookieOptions::default().domain(crate::DOMAIN.split(':').next().unwrap()),
    );

    view! {
        <Show when=move || !cookie_consent.get().is_some_and(|c| c)>
            <div
                id="gdpr"
                class="fixed bottom-24 left-1/2 z-50 -translate-x-1/2 rounded-full bg-nf-white p-2 drop-shadow-2xl max-sm:w-11/12 opacity-0 animate-popup"
            >
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
                            class="cursor-pointer rounded-full bg-nf-color px-4 py-2 text-nf-white hover:bg-nf-dark"
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
        "©️ Copyright 2024 - {} Nicolas Frey",
        chrono::Utc::now().year()
    );

    let email = obfuscate_email();

    let (socials, _set_socials) = signal(vec![
        ("instagram", "https://www.instagram.com/nic_ol_ass"),
        ("bluesky", "https://bsky.app/profile/nicolas-frey.com"),
        ("youtube", "https://www.youtube.com/@microwonk"),
        ("github", "https://www.github.com/Microwonk"),
        ("itch-io", "https://microwonk.itch.io"),
        ("discord", "https://discordapp.com/users/444924590913749002"),
    ]);

    view! {
        <footer class="tornpaper-effect md:flex md:items-center md:justify-between shadow rounded-lg p-4 md:p-6 xl:p-8 w-full">
            <ul class="flex items-center flex-wrap mb-6 md:mb-0">
                <li class="text-sm font-bold mr-4 md:mr-6">{copyright}</li>
                <li>
                    <a
                        href=move || format!("{}/privacy-policy", Apps::Www.url())
                        class="text-sm font-bold hover:underline mr-4 md:mr-6"
                    >
                        Privacy Policy
                    </a>
                </li>
                <li>
                    <a href=email class="text-sm font-bold hover:underline mr-4 md:mr-6">
                        Contact
                    </a>
                </li>
            </ul>
            <div class="flex sm:justify-center space-x-6">
                <For each=socials key=|state| state.0 let:child>
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
