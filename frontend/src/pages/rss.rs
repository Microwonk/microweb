use std::io::Cursor;

use html::Button;
use leptos::*;
use leptos_meta::*;
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::{
    components::header::Header,
    types::Profile,
    util::{Api, API_PATH},
};

#[component]
pub fn RSSPage(logged_in: ReadSignal<bool>, user: ReadSignal<Option<Profile>>) -> impl IntoView {
    let (rss_content, set_rss_content) = create_signal(String::new());
    let (copied, set_copied) = create_signal(false);

    let el = NodeRef::<Button>::new();
    let is_hovered =
        use_element_hover_with_options(el, UseElementHoverOptions::default().delay_leave(2000));

    spawn_local(async move {
        let str_rss = Api::get_rss().await.unwrap_or_default();
        let syntax_set = SyntaxSet::load_defaults_nonewlines();
        let syntax = syntax_set.find_syntax_by_token("xml").unwrap();
        let theme = ThemeSet::load_from_reader(&mut Cursor::new(include_str!(
            "../../public/assets/peel-light.tmTheme"
        )))
        .unwrap();
        let html = highlighted_html_for_string(&str_rss, &syntax_set, syntax, &theme).unwrap();
        set_rss_content(html);
    });

    let copy_to_clipboard = move |_| {
        let _ = window()
            .navigator()
            .clipboard()
            .write_text(format!("{}/rss", API_PATH).as_str());
        set_copied(true);
    };

    let icon = icondata::IoCopy;

    view! {
        <Title text="RSS Feed"/>
        <Header logged_in user/>
        <div class="py-12 md:px-0 md:mx-auto md:w-[96rem]">
            <h1 class="text-4xl font-bold md:tracking-tight md:text-5xl">RSS Feed Viewer</h1>
            <button
                on:click=copy_to_clipboard
                node_ref=el
                class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-nf-white bg-nf-color rounded hover:bg-nf-dark"
            >
                {move || if copied.get() {
                    if is_hovered.get() {
                        set_copied(false);
                    }
                    "Copied!".into()
                } else {
                    format!("{}/rss", API_PATH)
                }}
                <svg
                    class="w-4 h-4 text-nf-white" // Set width and height relative to text size
                    x=icon.x
                    y=icon.y
                    viewBox=icon.view_box
                    stroke-linecap=icon.stroke_linecap
                    stroke-linejoin=icon.stroke_linejoin
                    stroke-width=icon.stroke_width
                    stroke=icon.stroke
                    fill="#dad6ca"
                    inner_html=icon.data
                ></svg>
            </button>
            <div class="markdown">
                <div inner_html=move || rss_content.get()></div>
            </div>
        </div>
    }
}
