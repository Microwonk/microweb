use inkjet::{
    formatter::ThemedHtml,
    theme::{vendored, Theme},
    Highlighter, Language,
};
use leptos::*;
use leptos_router::use_params_map;
use pulldown_cmark::*;

use crate::{components::header::Header, pages::loading::LoadingPage, types::Post};

#[component]
pub fn BlogPostPage(
    logged_in: ReadSignal<bool>,
    blog_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    let (blog_post, set_blog_post) = create_signal(None::<Post>);
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap());

    // filter slug to find blog post
    set_blog_post(blog_posts.get().iter().find(|&b| b.slug == slug()).cloned());

    let code = r#"
```rust
fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 10;
    println!("x + y = {}", x + y);
}
```"#
        .to_string();

    view! {
        <Header logged_in/>
        // <Show when=move || blog_post.get().is_some() fallback=LoadingPage>
        //     <BlogPost content=blog_post.get().unwrap().markdown_content />
        // </Show>
        <BlogPost content=code />
    }
}

#[component]
pub fn BlogPost(#[prop(into)] content: String) -> impl IntoView {
    view! {
        <BodyToHtml content />
    }
}

#[component]
fn BodyToHtml(#[prop(into)] content: String) -> impl IntoView {
    leptos::leptos_dom::html::div()
        .attr("class", "markdown")
        .inner_html(markdown_to_html(content.as_str()).clone())
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
                    "<a href=\"#{}\" id=\"{}\"><span class=\"anchor-icon\">#</span></a>",
                    heading_id, heading_id
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

    let mut language = Language::Plaintext;
    let mut highlighter = Highlighter::new();
    let theme: Theme = Theme::from_helix(vendored::AYU_LIGHT).unwrap();
    let formatter = ThemedHtml::new(theme);

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        language = Language::from_token(lang).unwrap_or(Language::Plaintext);
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                if !in_code_block {
                    panic!("this should never happen");
                }

                let html_out = highlighter
                    .highlight_to_string(language, &formatter, &to_highlight)
                    .unwrap();

                to_highlight.clear();
                in_code_block = false;

                // Push the generated HTML
                out_events.push(Event::Html(CowStr::from(html_out)));
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
