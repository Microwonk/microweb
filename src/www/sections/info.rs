use crate::{
    apps::{Apps, components::Footer},
    www::utils::map_y_to_value,
};

use leptos::{html::Div, prelude::*};
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
            <br />
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
                class="gtkm text-2xl md:text-3xl lg:text-4xl font-[400] py-8 md:py-10 font-montserrat"
                href=move || Apps::Blog.url()
            >
                blog
            </a>
            <div class="w-full h-[2px] bg-nf-dark"></div>
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
pub fn Info() -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let is_element_visible = use_element_visibility(el);
    let (_, y) = use_window_scroll();

    let (y_visible_coord, set_y_visible_coord) = signal::<f64>(0.0);

    Effect::new(move |_| {
        if is_element_visible.get() && y_visible_coord.get() == 0.0 {
            set_y_visible_coord(y.get());
        }
    });

    view! {
        <div
            node_ref=el
            aria_label="Info"
            id="footer".to_string()
            class="bg-nf-white w-full pt-28 pb-4 lg:pt-64 isolate selection:bg-nf-dark selection:text-nf-white px-4 md:px-6"
            style=move || {
                format!(
                    "border-top-left-radius: {}px; border-top-right-radius: {}px;",
                    map_y_to_value(y.get(), y_visible_coord.get() + 240.0),
                    map_y_to_value(y.get(), y_visible_coord.get() + 240.0),
                )
            }
        >
            <div class="w-full max-w-screen-2xl mx-auto">
                <InfoSocials />
                <div class="flex flex-row">
                    <Footer />
                </div>
            </div>
        </div>
    }
}
