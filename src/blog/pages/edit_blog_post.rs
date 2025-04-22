use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
use leptos_use::use_debounce_fn;

use crate::blog::pages::{blog_post::BlogPost, loading::LoadingPage};
use crate::models::*;

#[server(GetPostAction, "/api/admin", "GetJson", endpoint = "post")]
#[tracing::instrument]
pub async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

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

#[server(UpdatePostAction, "/api", endpoint = "post")]
#[tracing::instrument]
pub async fn update_post(post: NewPost, post_id: i32) -> Result<u64, ServerFnError> {
    use chrono::Utc;

    sqlx::query(
        r#"
        UPDATE posts
        SET title = $1, slug = $2, description = $3, markdown_content = $4, updated_at = $5
        WHERE id = $6
        "#,
    )
    .bind(post.title.as_str())
    .bind(post.title.replace(" ", "_").to_ascii_lowercase())
    .bind(post.description)
    .bind(post.markdown_content)
    .bind(Utc::now())
    .bind(post_id)
    .execute(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting posts: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve posts, try again later")
    })
    .map(|r| r.rows_affected())
}

#[component]
pub fn EditBlogPostPage() -> impl IntoView {
    let (blog_post, set_blog_post) = signal(None::<Post>);
    let (markdown_content, set_markdown_content) = signal(String::new());
    let (debounced_content, set_debounced_content) = signal(String::new());
    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").clone().unwrap());

    let post: Resource<Option<Post>> =
        Resource::new(slug, |slug| async { get_post(slug).await.ok() });

    Effect::new(move |_| {
        set_blog_post(post.get().flatten());
    });

    Effect::new(move |_| {
        if let Some(post) = blog_post.get() {
            set_markdown_content(post.markdown_content.clone());
            set_debounced_content(post.markdown_content.clone());
            set_title(post.title.clone());
            set_description(post.description.clone());
        }
    });

    let save = move || {
        let new_post = NewPost {
            title: title.get(),
            description: description.get(),
            markdown_content: markdown_content.get(),
        };
        spawn_local(async move {
            if update_post(new_post, blog_post.get_untracked().unwrap_or_default().id)
                .await
                .is_err()
            {
                let _ = window().alert_with_message("Could not save!");
            }
        });
    };

    let debounced_fn = use_debounce_fn(
        move || {
            set_debounced_content(markdown_content.get());
            save();
        },
        500.0,
    );

    window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.ctrl_key() && ev.key() == "s" {
            ev.prevent_default();
            save();
        }
    });

    view! {
        <Title text="Edit Blog Post" />
        <div class="flex flex-col p-4">
            <div class="flex mb-4">
                // Title Input Field
                <div class="w-1/2 p-4">
                    <label for="title" class="block text-sm font-medium text-gray-700">
                        Title
                    </label>
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
                    <label for="desc" class="block text-sm font-medium text-gray-700">
                        Description
                    </label>
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
                        on:input=move |ev| {
                            set_markdown_content(event_target_value(&ev));
                            debounced_fn();
                        }
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
