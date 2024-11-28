use std::io::Cursor;

use chrono::{DateTime, Datelike};
use html::Button;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};
use rss::Channel;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::{
    components::header::Header,
    types::{Profile, RssEntry, RssFeed},
    util::{Api, API_PATH},
};

#[component]
pub fn RSSPage(logged_in: ReadSignal<bool>, user: ReadSignal<Option<Profile>>) -> impl IntoView {
    let (rss_content, set_rss_content) = create_signal(None::<String>);
    let (copied, set_copied) = create_signal(false);
    let (rss_feed, set_rss_feed) = create_signal(None::<RssFeed>);
    let (view_feed, set_view_feed) = create_signal(true);

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
        set_rss_content(Some(html));

        let channel = Channel::read_from(str_rss.as_bytes());

        if let Ok(c) = channel {
            set_rss_feed(Some(c.into()))
        }
    });

    let copy_to_clipboard = move |_| {
        let _ = window()
            .navigator()
            .clipboard()
            .write_text(format!("{}/rss", API_PATH).as_str());
        set_copied(true);
    };

    let c_icon = icondata::IoCopy;
    let l_icon = icondata::IoLink;

    view! {
        <Title text="RSS Feed"/>
        <Header logged_in user/>
        <div class="py-12 md:px-0 md:mx-auto md:w-[96rem]">
            <h1 class="text-4xl font-bold md:tracking-tight md:text-5xl">RSS Feed Viewer</h1>
            <div class="flex gap-4 my-4">
                <button
                    on:click=copy_to_clipboard
                    node_ref=el
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-nf-color rounded-lg hover:bg-nf-dark"
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
                        class="w-4 h-4 text-white"
                        x=c_icon.x
                        y=c_icon.y
                        viewBox=c_icon.view_box
                        stroke-linecap=c_icon.stroke_linecap
                        stroke-linejoin=c_icon.stroke_linejoin
                        stroke-width=c_icon.stroke_width
                        stroke=c_icon.stroke
                        fill="white"
                        inner_html=c_icon.data
                    ></svg>
                </button>

                <label class="inline-flex items-center cursor-pointer">
                    <input type="checkbox" on:click=move |_| set_view_feed(!view_feed.get()) value=move || view_feed.get() class="sr-only peer"/>
                    <div class="relative w-11 h-6 bg-nf-dark rounded-full peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-nf-dark after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-nf-color">
                    </div>
                    <span class="ms-3 text-sm font-medium text-nf-dark">
                        Toggle XML View
                    </span>
                </label>

            </div>

            <Show when=move || view_feed.get() && rss_feed.get().is_some() fallback=move || view! {
                <div class="markdown">
                    <div inner_html=move || rss_content.get().unwrap_or("Loading . . .".into())></div>
                </div>
            }>
                <div class="w-1/2">
                    <div class="mx-auto max-w-screen-xl py-12 px-4  rounded-lg bg-white mb-4">
                        <div class="flex flex-col items-start gap-4 md:flex-row md:items-center md:justify-between">
                            <div>
                                <div class="flex items-center gap-2">
                                    <h1 class="text-2xl font-bold text-nf-dark sm:text-3xl">{move || {
                                        rss_feed.get().unwrap().title

                                    }}</h1>
                                    <A href=move || rss_feed.get().unwrap().link>
                                        <svg
                                            class="w-8 h-8 text-nf-dark"
                                            x=l_icon.x
                                            y=l_icon.y
                                            viewBox=l_icon.view_box
                                            stroke-linecap=l_icon.stroke_linecap
                                            stroke-linejoin=l_icon.stroke_linejoin
                                            stroke-width=l_icon.stroke_width
                                            stroke=l_icon.stroke
                                            fill="white"
                                            inner_html=l_icon.data
                                        ></svg>
                                    </A>
                                </div>

                                <p class="mt-1.5 text-sm text-nf-dark">
                                    {move || rss_feed.get().unwrap().description}
                                </p>
                            </div>
                        </div>
                    </div>
                    <ul class="list-none space-y-4">
                    <For
                        each=move || rss_feed.get().unwrap().items
                        key=|i| i.guid.clone()
                        children=move |entry: RssEntry| {
                            let date = DateTime::parse_from_rfc2822(entry.pub_date.as_str()).unwrap_or_default();
                            view! {
                                <li>
                                    <article class="flex bg-white transition hover:shadow-xl rounded-lg">
                                        <div class="rotate-180 p-2 [writing-mode:_vertical-lr]">
                                            <time
                                                pubdate
                                                datetime={date.format("%Y-%m-%d").to_string()}
                                                class="flex items-center justify-between gap-4 text-xs font-bold uppercase text-nf-dark"
                                            >
                                                <span>{date.year()}</span>
                                                <span class="w-px flex-1 bg-gray-900/10"></span>
                                                <span>{date.format("%b. %d").to_string()}</span>
                                            </time>
                                        </div>

                                        <div class="flex flex-1 flex-col justify-between">
                                            <div class="border-s border-gray-900/10 p-4 sm:border-l-transparent sm:p-6">
                                            <A href={entry.link.clone()}>
                                                <h3 class="font-bold uppercase text-nf-dark">
                                                {entry.title}
                                                </h3>
                                            </A>

                                            <p class="mt-2 line-clamp-3 text-sm/relaxed text-gray-700">
                                                {entry.description}
                                            </p>
                                            </div>

                                            <div class="sm:flex sm:items-end sm:justify-start">
                                                <A
                                                    // TODO: fetch somewhere else? nah
                                                    href="https://www.nicolas-frey.com"
                                                    class="flex px-6 py-3 text-center text-xs text-gray-700"
                                                >
                                                    {entry.author}
                                                </A>
                                            </div>

                                            <div class="sm:flex sm:items-end sm:justify-end">
                                                <A
                                                    href={entry.link}
                                                    class="block bg-nf-color px-5 py-3 text-center text-xs font-bold uppercase text-nf-dark transition hover:bg-nf-dark hover:text-nf-white"
                                                >
                                                    Read Blog
                                                </A>
                                            </div>
                                        </div>
                                    </article>
                                </li>
                            }
                        }
                    />
                    </ul>
                </div>
            </Show>
        </div>
    }
}
