use std::io::Cursor;

use chrono::{DateTime, Datelike};
use leptos::{html::Button, prelude::*};
use leptos_meta::*;
use leptos_router::components::A;
use leptos_use::{UseTimeoutFnReturn, use_timeout_fn};
use rss::Channel;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::blog::{THEME_STR, components::header::Header, pages::loading::LoadingPage};
use crate::models::*;

#[server(Rss, "/api", "GetJson", endpoint = "rss.xml")]
#[tracing::instrument]
pub async fn rss() -> Result<String, ServerFnError> {
    Ok(generate_rss(
        "Microwonk's Blog",
        "Ramblings of a (Game) Developer",
        "https://blog.nicolas-frey.com",
        &sqlx::query_as::<_, Post>(
            r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.release_date,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE released = true ORDER BY release_date DESC"#,
        )
        .fetch_all(crate::database::db())
        .await
        .map_err(|e| {
            let err = format!("Error while getting posts: {e:?}");
            tracing::error!("{err}");
            ServerFnError::new("Could not retrieve posts, try again later")
        })?,
    ))
}

#[cfg(feature = "ssr")]
pub fn generate_rss(title: &str, description: &str, link: &str, posts: &[Post]) -> String {
    // Let's generate all those XML tags for Posts and collect them into a single string
    let rss_entries = posts
        .iter()
        .cloned()
        .map(|p| RssEntry::from(p).to_item())
        .collect::<String>();

    format!(
        r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
            <channel>
                <title>{title}</title>
                <description>{description}</description>
                <link>{link}</link>
                <language>en-us</language>
                <ttl>60</ttl>
                <atom:link href="https://blog.nicolas-frey.com/api/rss.xml" rel="self" type="application/rss+xml" />
                {rss_entries}
            </channel>
        </rss>   
     "#,
    )
}

#[component]
pub fn RSSPage() -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let (view_feed, set_view_feed) = signal(true);

    let el = NodeRef::<Button>::new();

    let rss_resource: Resource<Result<(String, RssFeed), ServerFnError>> = Resource::new(
        || (),
        |_| async move {
            let ssr_str = rss().await?;
            let syntax_set = SyntaxSet::load_defaults_nonewlines();
            let syntax = syntax_set.find_syntax_by_token("xml").unwrap();
            let theme = ThemeSet::load_from_reader(&mut Cursor::new(THEME_STR)).unwrap();
            let html = highlighted_html_for_string(&ssr_str, &syntax_set, syntax, &theme).unwrap();

            let channel = Channel::read_from(ssr_str.as_bytes())?;

            Ok((html, channel.into()))
        },
    );

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(move |_| set_copied(false), 3000.0);

    let copy_to_clipboard = move |_| {
        let _ = window()
            .navigator()
            .clipboard()
            .write_text(format!("{}/api/rss.xml", "https://blog.nicolas-frey.com").as_str());
        set_copied(true);
        start(0);
    };

    let c_icon = icondata::IoCopy;
    let l_icon = icondata::IoLink;

    view! {
        <Title text="RSS Feed" />

        <Header />

        <div class="py-12 px-0 mx-auto lg:w-[96rem]">
            <h1 class="text-4xl font-bold md:tracking-tight md:text-5xl">RSS Feed Viewer</h1>
            <div class="flex gap-4 my-4">
                <button
                    on:click=copy_to_clipboard
                    node_ref=el
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-nf-color rounded-lg hover:bg-nf-dark"
                >
                    {move || {
                        if copied.get() {
                            "Copied!".into()
                        } else {
                            format!("{}/api/rss.xml", "blog.nicolas-frey.com")
                        }
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
                    <input
                        type="checkbox"
                        on:click=move |_| set_view_feed(!view_feed.get())
                        value=move || view_feed.get()
                        class="sr-only peer"
                    />
                    <div class="relative w-11 h-6 bg-nf-dark rounded-full peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-nf-dark after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-nf-color"></div>
                    <span class="ms-3 text-sm font-medium text-nf-dark">Toggle XML View</span>
                </label>

            </div>

            <Suspense fallback=LoadingPage>
                <ErrorBoundary fallback=|_| {
                    view! {
                        <p class="error-messages text-xs-center">
                            "Something went wrong, please try again later."
                        </p>
                    }
                }>
                    {move || {
                        rss_resource
                            .get()
                            .map(|r| {
                                r.map(|(html, rss_feed)| {
                                    let (html, _) = signal(html);
                                    let (rss_feed, _) = signal(rss_feed);

                                    view! {
                                        <Show
                                            when=move || view_feed.get()
                                            fallback=move || {
                                                view! {
                                                    <div class="markdown">
                                                        <div inner_html=move || html.get()></div>
                                                    </div>
                                                }
                                            }
                                        >
                                            <div class="lg:w-1/2">
                                                <div class="mx-auto max-w-screen-xl py-12 px-4  rounded-lg bg-white mb-4">
                                                    <div class="flex flex-col items-start gap-4 md:flex-row md:items-center md:justify-between">
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <h1 class="text-2xl font-bold text-nf-dark sm:text-3xl">
                                                                    {move || rss_feed.get().title}
                                                                </h1>
                                                                <A href=move || rss_feed.get().link>
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
                                                                {move || rss_feed.get().description}
                                                            </p>
                                                        </div>
                                                    </div>
                                                </div>
                                                <ul class="list-none space-y-4">
                                                    <For
                                                        each=move || rss_feed.get().items
                                                        key=|i| i.guid.clone()
                                                        children=move |entry: RssEntry| {
                                                            let date = DateTime::parse_from_rfc2822(
                                                                    entry.pub_date.as_str(),
                                                                )
                                                                .unwrap_or_default();
                                                            view! {
                                                                <li>
                                                                    <article class="flex bg-white transition hover:shadow-xl rounded-lg">
                                                                        <div class="rotate-180 p-2 [writing-mode:_vertical-lr]">
                                                                            <time
                                                                                datetime=date.format("%Y-%m-%d").to_string()
                                                                                class="flex items-center justify-between gap-4 text-xs font-bold uppercase text-nf-dark"
                                                                            >
                                                                                <span>{date.year()}</span>
                                                                                <span class="w-px flex-1 bg-gray-900/10"></span>
                                                                                <span>{date.format("%b. %d").to_string()}</span>
                                                                            </time>
                                                                        </div>

                                                                        <div class="flex flex-1 flex-col justify-between">
                                                                            <div class="border-s border-gray-900/10 p-4 sm:border-l-transparent sm:p-6">
                                                                                <A href=entry.link.clone()>
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
                                                                                    href="https://www.nicolas-frey.com"
                                                                                    attr:class="flex px-6 py-3 text-center text-xs text-gray-700"
                                                                                >
                                                                                    {entry.author}
                                                                                </A>
                                                                            </div>

                                                                            <div class="sm:flex sm:items-end sm:justify-end">
                                                                                <A
                                                                                    href=entry.link
                                                                                    attr:class="block bg-nf-color px-5 py-3 text-center text-xs font-bold uppercase text-nf-dark transition hover:bg-nf-dark hover:text-nf-white"
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
                                    }
                                })
                            })
                    }}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
