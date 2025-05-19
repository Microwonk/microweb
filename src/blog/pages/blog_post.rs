use flate2::Compression;
use flate2::write::DeflateEncoder;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use pulldown_cmark::*;
use regex::Regex;
use std::io::{Cursor, Write};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::blog::{
    THEME_STR,
    components::{comment::CommentSection, header::Header, links::Links},
    pages::loading::LoadingPage,
};
use crate::models::*;

#[server(GetPostAction, "/api", "GetJson", endpoint = "post")]
#[tracing::instrument]
pub async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    sqlx::query_as::<_, Post>(
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
        WHERE released = true
        AND posts.slug = $1"#,
    )
    .bind(slug)
    .fetch_one(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting posts: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve posts, try again later")
    })
}

#[server(GetCommentsAction, "/api", "GetJson", endpoint = "comments")]
#[tracing::instrument]
pub async fn get_comments(post_id: i32) -> Result<Vec<Comment>, ServerFnError> {
    sqlx::query_as::<_, Comment>(
        r#"
        SELECT 
            comments.id,
            users.name AS author_name,
            comments.author AS author_id,
            comments.content,
            comments.replying_to,
            comments.created_at
        FROM comments
        JOIN posts ON comments.post = posts.id
        LEFT JOIN users ON comments.author = users.id
        WHERE comments.post = $1
        ORDER BY comments.created_at DESC
        "#,
    )
    .bind(post_id)
    .fetch_all(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting posts: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve posts, try again later")
    })
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with_untracked(|params| params.get("slug").clone().unwrap());

    let res: Resource<Result<(Post, Vec<Comment>), ServerFnError>> =
        Resource::new(slug, |slug| async {
            let post = get_post(slug).await?;
            let comments = get_comments(post.id).await?;
            Ok((post, comments))
        });

    view! {
        <Suspense fallback=LoadingPage>
            <ErrorBoundary fallback=|_| {
                view! {
                    <p class="error-messages text-xs-center">
                        "Something went wrong, please try again later."
                    </p>
                }
            }>
                <Header />
                {move || {
                    res.get()
                        .map(move |r| {
                            r.map(move |(blog_post, comments)| {
                                view! {
                                    <Title text=blog_post.title.clone() />

                                    <article class="py-12 px-4 md:px-0 md:mx-auto md:w-[48rem]">
                                        <BlogPostHeader
                                            blog_post=blog_post.clone()
                                            num_comments=comments.len()
                                        />
                                        <BlogPost content=blog_post.markdown_content.clone() />
                                        <Links />
                                    </article>

                                    <CommentSection comments blog_post_id=blog_post.id />
                                }
                            })
                        })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn BlogPostHeader(
    #[prop(into)] blog_post: Post,
    #[prop(into)] num_comments: usize,
) -> impl IntoView {
    let read_time = calculate_read_time(&blog_post.markdown_content);

    view! {
        <div class="space-y-6 mb-12">
            <h1 class="text-4xl font-bold md:tracking-tight md:text-5xl">{blog_post.title}</h1>
            <div class="flex flex-col items-start justify-between w-full md:flex-row md:items-center dark:text-gray-600">
                <div class="flex items-center md:space-x-2">
                    <p class="text-sm">
                        {move || {
                            format!(
                                "{} • {}{}",
                                blog_post.author_name,
                                blog_post
                                    .release_date
                                    .unwrap_or(blog_post.updated_at.unwrap_or(blog_post.created_at))
                                    .format("%b. %d, %Y"),
                                blog_post
                                    .updated_at
                                    .and_then(|d| {
                                        if blog_post.release_date.unwrap_or_default() < d {
                                            Some(d)
                                        } else {
                                            None
                                        }
                                    })
                                    .map_or(
                                        "".into(),
                                        |d| format!(" • Last Updated: {}", d.format("%b. %d, %Y")),
                                    ),
                            )
                        }}
                    </p>
                </div>
                <a
                    href="#comment_section"
                    class="flex-shrink-0 mt-3 text-sm md:mt-0 hover:underline"
                >
                    {format!("{read_time} min read • {num_comments} comments")}
                </a>
            </div>
        </div>
    }
}

pub fn calculate_read_time(markdown_content: &str) -> usize {
    // Parse Markdown and strip to plain text
    let parser = pulldown_cmark::Parser::new(markdown_content);
    let mut plain_text = String::new();
    for event in parser {
        if let Event::Text(text) = event {
            plain_text.push_str(&text);
        }
    }

    // Count words in the plain text using regex
    let re = Regex::new(r"\b\w+\b").unwrap();
    let word_count = re.find_iter(&plain_text).count();

    // Calculate read time (200 words per minute)
    (word_count as f64 / 200.0).ceil() as usize
}

#[component]
pub fn BlogPost(#[prop(into)] content: String) -> impl IntoView {
    view! { <div class="markdown" inner_html=markdown_to_html(content.as_str())></div> }
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let events = add_markdown_heading_ids(parser.into_iter().collect());
    let events = highlight_code(events);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

    html_output
}

fn add_markdown_heading_ids(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut parsing_header = false;
    let mut heading_id = String::new();
    let mut events_to_return = Vec::new();

    for event in events {
        match event {
            Event::Start(pulldown_cmark::Tag::Heading { .. }) => {
                parsing_header = true;
                heading_id.clear();
            }
            Event::End(pulldown_cmark::TagEnd::Heading { .. }) => {
                parsing_header = false;
                heading_id = heading_id.replace(" ", "_");

                events_to_return.push(Event::Text(CowStr::from(" ")));
                events_to_return.push(Event::Html(CowStr::from(format!(
                    "<a href=\"#{heading_id}\" id=\"{heading_id}\"><span class=\"anchor-icon\">#</span></a>"
                ))));
            }
            Event::Text(ref text) => {
                if parsing_header {
                    heading_id.push_str(text);
                }
            }
            _ => {}
        }
        events_to_return.push(event);
    }

    events_to_return
}

fn highlight_code(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut in_code_block = false;
    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let mut syntax = syntax_set.find_syntax_plain_text();

    let theme = ThemeSet::load_from_reader(&mut Cursor::new(THEME_STR)).unwrap();

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    let mut plantuml = false;

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        plantuml = lang == "plantuml".into();
                        syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(syntax);
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                if !in_code_block {
                    panic!("this should never happen");
                }

                if plantuml {
                    let diagram_url = generate_plantuml_diagram_url(&to_highlight);
                    let img_tag = format!("<img src=\"{diagram_url}\" alt=\"PlantUML Diagram\" />");
                    out_events.push(Event::Html(CowStr::from(img_tag)));
                } else {
                    // Regular code block, highlight syntax
                    let html =
                        highlighted_html_for_string(&to_highlight, &syntax_set, syntax, &theme)
                            .unwrap();
                    out_events.push(Event::Html(CowStr::from(html)));
                }

                to_highlight.clear();
                in_code_block = false;
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }

    out_events
}

fn generate_plantuml_diagram_url(plantuml_code: &str) -> String {
    let encoded = encode64(&compress_data(plantuml_code));
    let url = format!("http://www.plantuml.com/plantuml/png/{encoded}");
    url
}

fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn encode6bit(b: u8) -> char {
    match b {
        0..=9 => (b + 48) as char,        // '0'..'9'
        10..=35 => (b - 10 + 65) as char, // 'A'..'Z'
        36..=61 => (b - 36 + 97) as char, // 'a'..'z'
        62 => '-',                        // '-'
        63 => '_',                        // '_'
        _ => '?',                         // Fallback (should not happen)
    }
}

fn append3bytes(b1: u8, b2: u8, b3: u8) -> String {
    let c1 = b1 >> 2;
    let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
    let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
    let c4 = b3 & 0x3F;

    let mut r = String::new();
    r.push(encode6bit(c1 & 0x3F));
    r.push(encode6bit(c2 & 0x3F));
    r.push(encode6bit(c3 & 0x3F));
    r.push(encode6bit(c4 & 0x3F));

    r
}

fn encode64(c: &[u8]) -> String {
    let mut str = String::new();
    let len = c.len();

    let mut i = 0;
    while i < len {
        if i + 2 == len {
            str.push_str(&append3bytes(c[i], c[i + 1], 0));
        } else if i + 1 == len {
            str.push_str(&append3bytes(c[i], 0, 0));
        } else {
            str.push_str(&append3bytes(c[i], c[i + 1], c[i + 2]));
        }
        i += 3;
    }
    str
}
