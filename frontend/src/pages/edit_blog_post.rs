use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Timeout;
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

use crate::{
    pages::{blog_post::BlogPost, loading::LoadingPage},
    types::{NewPost, Post},
    util::Api,
};

#[component]
pub fn EditBlogPostPage(blog_posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let (blog_post, set_blog_post) = signal(None::<Post>);
    let (markdown_content, set_markdown_content) = signal(String::new());
    let (debounced_content, set_debounced_content) = signal(String::new());
    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").clone().unwrap());

    let debounce_timer: Rc<RefCell<Option<Timeout>>> = Rc::new(RefCell::new(None));

    // filter slug to find blog post
    Effect::new(move |_| {
        set_blog_post(blog_posts.get().iter().find(|&b| b.slug == slug()).cloned());
    });

    Effect::new(move |_| {
        if let Some(post) = blog_post.get() {
            set_markdown_content(post.markdown_content.clone());
            set_title(post.title.clone());
            set_description(post.description.clone());
        }
    });

    Effect::new(move |_| {
        let current_content = markdown_content.get().clone();
        let debounce_timer_clone = Rc::clone(&debounce_timer);

        if let Some(timer) = debounce_timer_clone.borrow_mut().take() {
            timer.cancel();
        }

        // Set a new debounce timer (500ms delay)
        let timer = Timeout::new(500, move || {
            set_debounced_content(current_content.clone());
        });

        // Store the timer reference
        *debounce_timer_clone.borrow_mut() = Some(timer);
    });

    let save = move || {
        let new_post = NewPost {
            title: title.get(),
            description: description.get(),
            markdown_content: markdown_content.get(),
        };
        spawn_local(async move {
            if (Api::update_blog_post(blog_post.get_untracked().map_or(0, |p| p.id), new_post)
                .await)
                .is_ok()
            {
                log::debug!("Saved!");
            }
        });
    };

    window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.ctrl_key() && ev.key() == "s" {
            ev.prevent_default();
            log::debug!("Saving!");
            save();
        }
    });

    view! {
        <Title text="Edit Blog Post"/>
        <div class="flex flex-col p-4">
            <div class="flex mb-4">
                // Title Input Field
                <div class="w-1/2 p-4">
                    <label for="title" class="block text-sm font-medium text-gray-700"> Title </label>
                    <textarea
                        id="title"
                        class="w-full p-2 border rounded"
                        placeholder="Title"
                        prop:value=move || title.get()
                        on:input=move |ev| set_title(event_target_value(&ev))
                    />
                </div>

                // Description Textarea
                <div class="w-1/2 p-4">
                    <label for="desc" class="block text-sm font-medium text-gray-700"> Description </label>
                    <textarea
                        id="desc"
                        class="w-full p-2 border rounded"
                        placeholder="Description"
                        prop:value=move || description.get()
                        on:input=move |ev| set_description(event_target_value(&ev))
                    />
                </div>
            </div>

            <div class="flex">
                // Textarea on the left side
                <div class="w-1/2 p-4">
                    <textarea
                        class="w-full h-screen p-2 border rounded"
                        prop:value=move || markdown_content.get()
                        on:input=move |ev| set_markdown_content(event_target_value(&ev))
                    />
                </div>

                // Rendered BlogPost on the right side
                <div class="w-1/2 p-4">
                    <Show when=move || blog_post.get().is_some() fallback=LoadingPage>
                        <BlogPost content=debounced_content.get() />
                    </Show>
                </div>
            </div>
        </div>
    }
}
