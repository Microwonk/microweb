use crate::utils::map_y_to_value;

use leptos::{html::Footer, *};
use leptos_use::{use_element_visibility, use_window_scroll};

#[component]
pub fn InfoSocials() -> impl IntoView {
    view! {
        <h2
            class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl mb-28 leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading"
            id="socials"
        >
            <div class="animated-title">
                <span class="animated-title-element text-nf-dark font-bold uppercase">Get</span>
            </div>
            {' '}
            <div class="animated-title">
                <span class="animated-title-element text-nf-dark font-bold uppercase">to</span>
            </div>
            {' '}
            <br/>
            <div class="animated-title">
                <span class="animated-title-element text-nf-dark font-bold uppercase">know</span>
            </div>
            {' '}
            <div class="animated-title">
                <span class="animated-title-element font-light text-nf-dark font-regular uppercase">
                    me
                </span>
            </div>
        </h2>
        <div class="flex flex-col">
            <a
                target="_blank"
                class="gtkm text-2xl md:text-3xl lg:text-4xl font-[400] py-8 md:py-10 font-montserrat"
                href="https://github.com/Microwonk"
            >
                github
            </a>
            <div class="w-full h-[2px] bg-nf-dark"></div>
            <a
                target="_blank"
                class="gtkm text-2xl md:text-3xl lg:text-4xl font-[400] py-8 md:py-10 font-montserrat"
                href="https://www.linkedin.com/in/nicolas-frey-28bb1a257/"
            >
                linkedIn
            </a>
            <div class="w-full h-[2px] bg-nf-dark"></div>
            <a
                class="gtkm text-2xl md:text-3xl lg:text-4xl font-[400] py-8 md:py-10 font-montserrat"
                href="/resume"
            >
                resume
            </a>
            <div class="w-full h-[2px] bg-nf-dark"></div>
        </div>
    }
}

#[component]
pub fn SocialsImages() -> impl IntoView {
    let (socials, _set_socials) = create_signal(vec![
        (
            "instagram".to_string(),
            "https://www.instagram.com/nic_ol_ass".to_string(),
        ),
        ("twitter".to_string(), "https://x.com/microw0nk".to_string()),
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
        (
            "pgp_key".to_string(),
            "/resources/public-key.asc".to_string(),
        ),
    ]);

    view! {
        <div class="flex flex-row mx-4">
            <For each=socials
                 key=|state| state.0.clone()
                 let:child>
                <a href=child.1 target="_blank" class="hover:animate-pulse">
                    <img src=move || {format!("/assets/{}.svg", child.0)} class="w-[24px] mx-4" />
                </a>
            </For>

        </div>
    }
}

#[component]
pub fn Info() -> impl IntoView {
    let el = create_node_ref::<Footer>();
    let is_element_visible = use_element_visibility(el);
    let (_, y) = use_window_scroll();

    let (y_visible_coord, set_y_visible_coord) = create_signal::<f64>(0.0);

    create_effect(move |_| {
        if is_element_visible.get() && y_visible_coord.get() == 0.0 {
            set_y_visible_coord(y.get());
        }
    });

    view! {
        <footer
            node_ref=el
            aria_label="Info"
            id="footer".to_string()
            class="selection:bg-nf-dark selection:text-nf-white bg-nf-white max-w-full pt-28 pb-4 md:py-28 lg:pt-64 lg:pb-32 relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6"
            style=move || {
                format!(
                    "border-top-left-radius: {}px;border-top-right-radius: {}px",
                    map_y_to_value(y.get(), y_visible_coord.get() + 240.0),
                    map_y_to_value(y.get(), y_visible_coord.get() + 240.0),
                )
            }
        >

            <div class="flex-col relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6">
                <InfoSocials/>
            </div>
            <div class="font-montserrat text-left justify-center mt-20 md:absolute md:left-5 md:bottom-5 text-sm sm:text-md text-nf-dark flex items-center">
                <span>"© Copyright 2024, Nicolas Frey, All rights reserved"</span>
                <SocialsImages/>
            </div>
        </footer>
    }
}
