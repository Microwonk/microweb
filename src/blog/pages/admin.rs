use crate::blog::{
    app::{GlobalState, GlobalStateStoreFields},
    components::{header::Header, side_menu::SideMenu},
    models::{Media, Post, User},
    pages::loading::LoadingPage,
};
use chrono::Utc;
use leptos::{html::Div, prelude::*};
use leptos::{task::spawn_local, Params};
use leptos_meta::*;
use leptos_router::hooks::{use_navigate, use_query};
use leptos_router::params::Params;
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use reactive_stores::Store;

#[derive(Params, PartialEq)]
struct TabQuery {
    tab: Option<String>,
}

#[server(GetPostsAction, "/api/admin", "GetJson", endpoint = "posts")]
#[tracing::instrument]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
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
        ORDER BY release_date DESC"#,
    )
    .fetch_all(crate::blog::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting posts: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve posts, try again later")
    })
}

#[server(GetUsersAction, "/api/admin", "GetJson", endpoint = "users")]
#[tracing::instrument]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query_as::<_, User>(r#"SELECT * FROM users ORDER BY created_at DESC"#)
        .fetch_all(crate::blog::database::db())
        .await
        .map_err(|e| {
            let err = format!("Error while getting users: {e:?}");
            tracing::error!("{err}");
            ServerFnError::new("Could not retrieve users, try again later")
        })
}

#[server(DeleteUserAction, "/api/admin", endpoint = "delete_user")]
#[tracing::instrument]
pub async fn delete_user(user_id: i32) -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(crate::blog::database::db())
        .await
        .map_err(|e| {
            let err = format!("Error while deleting user: {e:?}");
            tracing::error!("{err}");
            ServerFnError::new("Could not delete user.")
        })
        .map(|r| r.rows_affected())
}

#[server(UpdateUserAction, "/api/admin", endpoint = "update_user")]
#[tracing::instrument]
pub async fn update_user(name: String, email: String, id: i32) -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query(
        r#"
        UPDATE users
        SET name = $1, email = $2
        WHERE id = $3
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(id)
    .execute(crate::blog::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting users: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve users, try again later")
    })
    .map(|r| r.rows_affected())
}

#[server(DeleteBlogPostAction, "/api/admin", endpoint = "delete_post")]
#[tracing::instrument]
pub async fn delete_post(post_id: i32) -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(post_id)
        .execute(crate::blog::database::db())
        .await
        .map_err(|e| {
            let err = format!("Error while deleting post: {e:?}");
            tracing::error!("{err}");
            ServerFnError::new("Could not delete post.")
        })
        .map(|r| r.rows_affected())
}

#[server(ReleasePostAction, "/api/admin", endpoint = "release_post")]
#[tracing::instrument]
pub async fn release_post(release: bool, post_id: i32) -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query(
        r#"
            UPDATE posts
            SET released = $1, release_date = $2
            WHERE id = $3
            "#,
    )
    .bind(release)
    .bind(if release { Some(Utc::now()) } else { None })
    .bind(post_id)
    .execute(crate::blog::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while releasing post: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not release post.")
    })
    .map(|r| r.rows_affected())
}

#[server(CreatePostAction, "/api/admin", endpoint = "create_post")]
#[tracing::instrument]
pub async fn create_post() -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    let Extension(user) = extract::<Extension<User>>()
        .await
        .map_err(|_| ServerFnError::new("Unauthorized."))?;

    if !user.admin {
        return Err(ServerFnError::new("Unauthorized."));
    }

    let post = Post {
        id: 0,
        author: user.id,
        author_name: "".into(),
        title: "Your new Blog Post".into(),
        description: "Some Description".into(),
        slug: "".into(),
        markdown_content: "# Hello World!".into(),
        released: false,
        release_date: None,
        created_at: Utc::now().naive_local(),
        updated_at: None,
    };

    sqlx::query(
        r#"
        INSERT INTO posts (
            author, title, description, slug, markdown_content
        )
        VALUES (
            $1, $2, $3, $4, $5
        )
        "#,
    )
    .bind(post.author)
    .bind(post.title.as_str())
    .bind(post.description)
    .bind(post.title.replace(" ", "_").to_ascii_lowercase())
    .bind(post.markdown_content)
    .execute(crate::blog::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while releasing post: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not release post.")
    })
    .map(|r| r.rows_affected())
}

#[server(GetMediaAction, "/api/admin", "GetJson", endpoint = "media")]
#[tracing::instrument]
pub async fn get_media() -> Result<Vec<Media>, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    sqlx::query_as::<_, Media>(
        "SELECT id, post_id, name, media_type, created_at FROM media ORDER BY created_at DESC",
    )
    .fetch_all(crate::blog::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while getting users: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not retrieve users, try again later")
    })
}

#[component]
pub fn AdminPage() -> impl IntoView {
    let query = use_query::<TabQuery>();

    let store = expect_context::<Store<GlobalState>>();

    let (updated, set_updated) = signal(0u32);

    let (current_tab, set_current_tab) = signal(String::new());
    let (blog_posts, set_blog_posts) = signal(Vec::new());
    let (users, set_users) = signal(Vec::new());
    let (media, set_media) = signal(Vec::new());

    let posts_res = Resource::new(
        move || updated.get(),
        |_| async move { get_posts().await.unwrap_or(Vec::new()) },
    );

    let users_res = Resource::new(
        move || updated.get(),
        |_| async move { get_users().await.unwrap_or(Vec::new()) },
    );

    let media_res = Resource::new(
        move || updated.get(),
        |_| async move { get_media().await.unwrap_or(Vec::new()) },
    );

    Effect::new(move |_| {
        if !store.user().get().is_some_and(|u| u.is_admin) {
            use_navigate()("/", Default::default());
        }

        if let Some(Some(t)) = query.with(|q| q.as_ref().map(|t| t.tab.clone()).ok()) {
            set_current_tab(t);
        } else {
            use_navigate()("/admin?tab=blogs", Default::default());
        }

        set_blog_posts(posts_res.get().unwrap_or(Vec::new()));
        set_users(users_res.get().unwrap_or(Vec::new()));
        set_media(media_res.get().unwrap_or(Vec::new()));
    });

    view! {
        <Title text="Admin Dashboard" />

        <Header />
        // Container that ensures full screen height
        <div class="flex flex-col min-h-screen">
            // Grid takes up the remaining space
            <div class="flex-grow grid grid-cols-6 gap-4">
                <div class="md:col-span-2 lg:col-span-1">
                    <SideMenu />
                </div>
                <div class="md:col-span-4 lg:col-span-5">
                    <div class="p-6">
                        {move || match current_tab.get().as_str() {
                            "users" => view! { <UserSection users set_updated /> }.into_any(),
                            "media" => {
                                view! { <MediaSection media set_updated blog_posts /> }.into_any()
                            }
                            "blogs" => view! { <BlogSection blog_posts set_updated /> }.into_any(),
                            _ => view! { <LoadingPage /> }.into_any(),
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn UserSection(users: ReadSignal<Vec<User>>, set_updated: WriteSignal<u32>) -> impl IntoView {
    let edit_row = RwSignal::new(None::<i32>);
    let username = RwSignal::new("".to_string());
    let email = RwSignal::new("".to_string());
    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                    <tr>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Name</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Email</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Admin</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Created At
                        </th>
                        <th class="px-4 py-2"></th>
                        <th class="px-4 py-2"></th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                    <For
                        each=move || users.get()
                        key=|user| user.id
                        children=move |user: User| {
                            let user_c = user.clone();
                            view! {
                                <Show
                                    when=move || edit_row.get().is_some_and(|row| row == user.id)
                                    fallback=move || {
                                        view! {
                                            <UserRow user=user_c.clone() edit_row set_updated />
                                        }
                                    }
                                >
                                    <UserEditRow user=user.clone() username email set_updated />
                                </Show>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn UserRow(
    #[prop(into)] user: Signal<User>,
    #[prop(into)] edit_row: RwSignal<Option<i32>>,
    set_updated: WriteSignal<u32>,
) -> impl IntoView {
    let user = user.get();
    view! {
        <tr class="odd:bg-gray-50">
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.id}</td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{user.name}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.email}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                {if user.admin { "yes " } else { "no" }}
            </td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.created_at.to_string()}</td>
            <td class="whitespace-nowrap px-4 py-2">
                <button
                    class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                    on:click=move |_| {
                        spawn_local(async move {
                            if delete_user(user.id).await.is_ok() {
                                set_updated.update(|i| *i += 1);
                            }
                        });
                    }
                >
                    Delete
                </button>
            </td>
            <td class="whitespace-nowrap px-4 py-2">
                <button
                    class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                    on:click=move |_| {
                        edit_row.set(Some(user.id));
                    }
                >
                    Edit
                </button>
            </td>
        </tr>
    }
}

#[component]
pub fn UserEditRow(
    #[prop(into)] user: Signal<User>,
    #[prop(into)] username: RwSignal<String>,
    #[prop(into)] email: RwSignal<String>,
    set_updated: WriteSignal<u32>,
) -> impl IntoView {
    let user = user.get();
    username.set(user.name.clone());
    email.set(user.email.clone());

    view! {
        <tr class="bg-nf-color">
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.id}</td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                <input
                    on:input=move |ev| {
                        let new_value = event_target_value(&ev);
                        username.set(new_value);
                    }
                    prop:value=user.name
                />
            </td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                <input
                    on:input=move |ev| {
                        let new_value = event_target_value(&ev);
                        email.set(new_value);
                    }
                    prop:value=user.email
                />
            </td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                {if user.admin { "yes " } else { "no" }}
            </td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.created_at.to_string()}</td>
            <td class="whitespace-nowrap px-4 py-2">
                <button
                    class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                    on:click=move |_| {
                        spawn_local(async move {
                            if delete_user(user.id).await.is_ok() {
                                set_updated.update(|i| *i += 1);
                            }
                        });
                    }
                >
                    Delete
                </button>
            </td>
            <td class="whitespace-nowrap px-4 py-2">
                <button
                    class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                    on:click=move |_| {
                        spawn_local(async move {
                            if update_user(username.get_untracked(), email.get_untracked(), user.id)
                                .await
                                .is_ok()
                            {
                                set_updated.update(|i| *i += 1);
                            }
                        });
                    }
                >
                    Save
                </button>
            </td>
        </tr>
    }
}

#[component]
pub fn MediaSection(
    media: ReadSignal<Vec<Media>>,
    set_updated: WriteSignal<u32>,
    blog_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    let (post_id, set_post_id) = signal(0);

    let _ = set_updated;

    let drop_zone_el = NodeRef::<Div>::new();

    let UseDropZoneReturn { files, .. } =
        use_drop_zone_with_options(drop_zone_el, UseDropZoneOptions::default());

    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                    <tr>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Post</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Name</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Path</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            MIME Type
                        </th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Created At
                        </th>
                        <th class="px-4 py-2"></th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                    <For
                        each=move || media.get()
                        key=|media| media.id
                        children=move |media: Media| {
                            let post = blog_posts
                                .get()
                                .iter()
                                .find(|&p| p.id == media.post_id)
                                .cloned();
                            let path = format!(
                                "https://microblog.shuttleapp.rs/upload/{}",
                                media.id,
                            );
                            view! {
                                <tr class="odd:bg-gray-50">
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {media.id}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                                        {if let Some(p) = post {
                                            p.title
                                        } else {
                                            format!("No Post? ID {}", media.post_id)
                                        }}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {media.name}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <a href=path target="_blank">
                                            {format!("/upload/{}", media.id)}
                                        </a>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {media.media_type}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {media.created_at.to_string()}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2">
                                        <button
                                            class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                                            on:click=move |_| {}
                                        >
                                            Delete
                                        </button>
                                    </td>
                                </tr>
                            }
                        }
                    />
                    <tr class="odd:bg-gray-50">
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            {move || {
                                media
                                    .get()
                                    .iter()
                                    .map(|m| m.id)
                                    .max()
                                    .map(|i| i + 1)
                                    .unwrap_or_default()
                            }}
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            <select
                                on:change=move |ev| {
                                    let new_value = event_target_value(&ev);
                                    set_post_id(new_value.parse().unwrap());
                                }
                                prop:value=move || post_id.get().to_string()
                            >
                                <option value=0 hidden>
                                    Please select a post
                                </option>
                                {move || {
                                    blog_posts
                                        .get()
                                        .iter()
                                        .cloned()
                                        .map(|p| {
                                            view! {
                                                <option
                                                    value=p.id
                                                    on:click=move |ev| {
                                                        let new_value = event_target_value(&ev);
                                                        set_post_id(new_value.parse().unwrap());
                                                    }
                                                >
                                                    {p.title}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            <div
                                node_ref=drop_zone_el
                                class="flex flex-col w-full min-h-[200px] h-auto bg-gray-400/10 justify-center items-center pt-6"
                            >
                                "Drop files here"
                                <For each=files key=|f| f.name() let:file>
                                    <div class="w-200px pa-6">
                                        <p>Name: {file.name()}, Size: {file.size()}</p>
                                    </div>
                                </For>
                            </div>
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            {move || {
                                format!(
                                    "/upload/{}",
                                    media
                                        .get()
                                        .iter()
                                        .map(|m| m.id)
                                        .max()
                                        .map(|i| i + 1)
                                        .unwrap_or_default(),
                                )
                            }}
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">?</td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            {move || { Utc::now().naive_local().to_string() }}
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            <button
                                class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                on:click=move |_| {}
                            >
                                Add
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn BlogSection(
    blog_posts: ReadSignal<Vec<Post>>,
    set_updated: WriteSignal<u32>,
) -> impl IntoView {
    let (max_id, set_max_id) = signal(0);
    Effect::new(move |_| {
        set_max_id(
            blog_posts
                .get()
                .iter()
                .map(|m| m.id)
                .max()
                .map(|i| i + 1)
                .unwrap_or_default(),
        )
    });

    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                    <tr>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Title</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Created At
                        </th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Released
                        </th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Released At
                        </th>
                        <th class="px-4 py-2"></th>
                        <th class="px-4 py-2"></th>
                        <th class="px-4 py-2"></th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                    <For
                        each=move || blog_posts.get()
                        key=|post| post.id
                        children=move |post: Post| {
                            view! {
                                <tr class="odd:bg-gray-50">
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {post.id}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                                        {post.title}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                                        {post.created_at.to_string()}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                                        {if post.released { "yes " } else { "no" }}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                                        {post
                                            .release_date
                                            .map(|r| r.to_string())
                                            .unwrap_or("".to_string())}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <a
                                            class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                            href=format!("/admin/posts/{}", post.slug)
                                            target="_blank"
                                        >
                                            Edit
                                        </a>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <button
                                            class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    if release_post(!post.released, post.id).await.is_ok() {
                                                        set_updated.update(|i| *i += 1);
                                                    }
                                                });
                                            }
                                        >
                                            {move || {
                                                if post.released { "Unrelease" } else { "Release" }
                                            }}
                                        </button>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <button
                                            class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    if delete_post(post.id).await.is_ok() {
                                                        set_updated.update(|i| *i += 1);
                                                    }
                                                });
                                            }
                                        >
                                            Delete
                                        </button>
                                    </td>
                                </tr>
                            }
                        }
                    />
                    <tr class="odd:bg-gray-50">
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            {move || max_id.get() + 1}
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            Add a New Blog Post!
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            {move || { Utc::now().naive_local().to_string() }}
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">nope</td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            <button
                                class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                on:click=move |_| {
                                    spawn_local(async move {
                                        if create_post().await.is_ok() {
                                            set_updated.update(|i| *i += 1);
                                        }
                                    });
                                }
                            >
                                Create
                            </button>
                        </td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700"></td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700"></td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

// TODO
// use std::{io::BufWriter, num::NonZeroU32};

// use crate::blog::{
//     admin_check, logs_handler::Log, ok, ApiError, ApiResult, Media, MediaNoData, ServerState, User,
// };
// use axum::{
//     body::Bytes,
//     extract::{Multipart, Path, State},
//     http::StatusCode,
//     response::IntoResponse,
//     Extension, Json,
// };

// use fast_image_resize as fr;
// use image::{
//     codecs::{jpeg::JpegEncoder, png::PngEncoder},
//     ColorType, ImageEncoder, ImageReader,
// };
// use uuid::Uuid;

// pub async fn upload(
//     Extension(identity): Extension<User>,
//     Path(post_id): Path<i32>,
//     State(state): State<ServerState>,
//     mut multipart: Multipart,
// ) -> ApiResult<impl IntoResponse> {
//     admin_check(&identity, &state).await?;

//     let mut successes = vec![];
//     let mut failures = vec![];

//     while let Ok(Some(field)) = multipart.next_field().await {
//         // Grab the name of the file
//         let file_name = match field.file_name().map(|f| {
//             let orig_name: Vec<&str> = f.split('.').collect();
//             format!(
//                 "{}_post{}_{}.{}",
//                 orig_name[0],
//                 post_id,
//                 Uuid::new_v4(),
//                 orig_name[1]
//             )
//         }) {
//             Some(name) => name,
//             None => {
//                 failures.push("Failed to read file name.".to_string());
//                 continue;
//             }
//         };

//         let content_type = match field.content_type().map(String::from) {
//             Some(content_type) => content_type,
//             None => {
//                 failures.push(format!(
//                     "Failed to read content type for file: {}",
//                     file_name
//                 ));
//                 continue;
//             }
//         };

//         if field.name().unwrap() == "file_upload" {
//             // Unwrap the incoming bytes
//             let data = match field.bytes().await {
//                 Ok(data) => data.to_vec(), // Convert Bytes to Vec<u8>
//                 Err(_) => {
//                     failures.push(format!("Could not read bytes for file: {}", file_name));
//                     continue;
//                 }
//             };

//             let compressed_data = match content_type.as_str() {
//                 "image/jpeg" | "image/png" => {
//                     match compress_image(&data, content_type.as_str(), 720, 720) {
//                         Ok(compressed_img) => compressed_img,
//                         Err(err) => {
//                             failures.push(format!(
//                                 "Image compression failed for file: {} with error: {}",
//                                 file_name, err
//                             ));
//                             continue;
//                         }
//                     }
//                 }
//                 "image/gif" => {
//                     // TODO: implement gif compression
//                     data
//                 }
//                 "video/mp4" => {
//                     // TODO: implement video compression
//                     data
//                 }
//                 _ => {
//                     failures.push(format!("Unsupported content type for file: {}", file_name));
//                     continue;
//                 }
//             };

//             // Try to insert media into the database
//             match sqlx::query_as::<_, Media>(
//                 r#"
//                 INSERT INTO media (post_id, name, data, media_type)
//                 VALUES ($1, $2, $3, $4)
//                 RETURNING id, post_id, name, data, media_type, created_at
//                 "#,
//             )
//             .bind(post_id)
//             .bind(file_name.clone())
//             .bind(compressed_data) // Store the compressed data
//             .bind(content_type) // directly store MIME
//             .fetch_one(&state.pool)
//             .await
//             {
//                 Ok(media) => successes.push(media),
//                 Err(e) => failures.push(format!("Database error {} for file: {}", e, file_name)),
//             }
//         }
//     }

//     Log::warn(
//         format!(
//             "Upload failed for one or multiple files. Errors: {:?}",
//             failures
//         ),
//         &state,
//     )
//     .await?;

//     // Prepare response with both successes and failures
//     let response = serde_json::json!({
//         "success": successes,
//         "failure": failures
//     });

//     // Return the response
//     ok!(response)
// }

// fn compress_image(
//     data: &[u8],
//     content_type: &str,
//     max_width: u32,
//     max_height: u32,
// ) -> Result<Vec<u8>, String> {
//     // Convert DynamicImage to an RGBA buffer
//     let img = ImageReader::new(std::io::Cursor::new(data))
//         .with_guessed_format()
//         .map_err(|e| e.to_string())?
//         .decode()
//         .map_err(|e| e.to_string())?;
//     let original_width = img.width();
//     let original_height = img.height();

//     // Calculate aspect ratio and the new dimensions while keeping the aspect ratio
//     let aspect_ratio = original_width as f32 / original_height as f32;
//     let (new_width, new_height) = if original_width > original_height {
//         let adjusted_height = (max_width as f32 / aspect_ratio).round() as u32;
//         (max_width, adjusted_height.min(max_height))
//     } else {
//         let adjusted_width = (max_height as f32 * aspect_ratio).round() as u32;
//         (adjusted_width.min(max_width), max_height)
//     };

//     // Convert the image to RGBA (or RGB if it's for JPEG)
//     let mut src_image = if content_type == "image/jpeg" {
//         // Strip the alpha channel for JPEG by converting the image to RGB
//         fr::Image::from_vec_u8(
//             NonZeroU32::new(original_width).unwrap(),
//             NonZeroU32::new(original_height).unwrap(),
//             img.to_rgb8().into_raw(), // Convert to RGB8 for JPEG
//             fr::PixelType::U8x3,      // RGB has 3 channels (U8x3)
//         )
//         .unwrap()
//     } else {
//         // Use RGBA8 for other formats like PNG
//         fr::Image::from_vec_u8(
//             NonZeroU32::new(original_width).unwrap(),
//             NonZeroU32::new(original_height).unwrap(),
//             img.to_rgba8().into_raw(), // RGBA for PNG
//             fr::PixelType::U8x4,       // RGBA has 4 channels (U8x4)
//         )
//         .unwrap()
//     };

//     // Multiple RGB channels of source image by alpha channel
//     // (not required for the Nearest algorithm)
//     let alpha_mul_div = fr::MulDiv::default();
//     if content_type != "image/jpeg" {
//         // Only multiply by alpha if it's not JPEG (which doesn't have alpha)
//         alpha_mul_div
//             .multiply_alpha_inplace(&mut src_image.view_mut())
//             .unwrap();
//     }

//     // Create container for data of destination image with new dimensions
//     let new_width_non_zero = NonZeroU32::new(new_width).unwrap();
//     let new_height_non_zero = NonZeroU32::new(new_height).unwrap();
//     let mut dst_image = fr::Image::new(
//         new_width_non_zero,
//         new_height_non_zero,
//         src_image.pixel_type(),
//     );

//     // Get mutable view of destination image data
//     let mut dst_view = dst_image.view_mut();

//     // Create Resizer instance and resize source image
//     let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
//     resizer
//         .resize(&src_image.view(), &mut dst_view)
//         .map_err(|e| e.to_string())?;

//     // Divide RGB channels of destination image by alpha
//     if content_type != "image/jpeg" {
//         // Only divide by alpha if it's not JPEG
//         alpha_mul_div
//             .divide_alpha_inplace(&mut dst_view)
//             .map_err(|e| e.to_string())?;
//     }

//     // Write destination image as PNG/JPEG-file
//     let mut result_buf = BufWriter::new(Vec::new());

//     match content_type {
//         "image/jpeg" => JpegEncoder::new(&mut result_buf)
//             .write_image(
//                 dst_image.buffer(),
//                 new_width_non_zero.get(),
//                 new_height_non_zero.get(),
//                 ColorType::Rgb8.into(), // Use RGB for JPEG
//             )
//             .map_err(|e| e.to_string())?,
//         "image/png" => PngEncoder::new(&mut result_buf)
//             .write_image(
//                 dst_image.buffer(),
//                 new_width_non_zero.get(),
//                 new_height_non_zero.get(),
//                 ColorType::Rgba8.into(), // Use RGBA for PNG
//             )
//             .map_err(|e| e.to_string())?,
//         _ => return Err("Unsupported image format".to_string()),
//     }

//     let image_bytes = result_buf.into_inner().map_err(|e| e.to_string())?;

//     Ok(image_bytes)
// }

// pub async fn get_upload(
//     Path(id): Path<i32>,
//     State(state): State<ServerState>,
// ) -> ApiResult<impl IntoResponse> {
//     let media: Media = match sqlx::query_as::<_, Media>("SELECT * FROM media WHERE id = $1 ")
//         .bind(id)
//         .fetch_one(&state.pool)
//         .await
//     {
//         Ok(media) => media,
//         Err(e) => {
//             return Err(ApiError::werr("Asset not Found.", StatusCode::NOT_FOUND, e));
//         }
//     };

//     Ok((
//         StatusCode::OK,
//         [
//             (
//                 "Content-Disposition",
//                 format!("inline; filename=\"{}\"", media.name),
//             ),
//             // Cache Control for 7 days
//             ("Cache-Control", "public, max-age=604800".to_owned()),
//             // unique identifier for caching
//             ("ETag", media.name),
//             // last modified with datetime value of HTTP date format RFC 7231
//             (
//                 "Last-Modified",
//                 media
//                     .created_at
//                     .format("%a, %d %b %Y %H:%M:%S GMT")
//                     .to_string(),
//             ),
//             ("Content-Type", media.media_type),
//         ],
//         Bytes::from(media.data),
//     ))
// }

// pub async fn delete_media(
//     Extension(identity): Extension<User>,
//     Path(id): Path<i32>,
//     State(state): State<ServerState>,
// ) -> ApiResult<impl IntoResponse> {
//     admin_check(&identity, &state).await?;
//     match sqlx::query("DELETE FROM media WHERE id = $1")
//         .bind(id)
//         .execute(&state.pool)
//         .await
//     {
//         Ok(_) => ok!(),
//         Err(e) => Err(ApiError::werr(
//             "Could not delete Media.",
//             StatusCode::BAD_REQUEST,
//             e,
//         )),
//     }
// }

// pub async fn get_all_media(
//     Extension(identity): Extension<User>,
//     State(state): State<ServerState>,
// ) -> ApiResult<impl IntoResponse> {
//     admin_check(&identity, &state).await?;

//     match sqlx::query_as::<_, MediaNoData>(
//         "SELECT id, post_id, name, media_type, created_at FROM media ORDER BY created_at DESC",
//     )
//     .fetch_all(&state.pool)
//     .await
//     {
//         Ok(response) => ok!(response),
//         Err(e) => Err(ApiError::werr(
//             "Error retrieving all media.",
//             StatusCode::BAD_REQUEST,
//             e,
//         )),
//     }
// }

// pub async fn get_all_media_by_post(
//     Extension(identity): Extension<User>,
//     Path(post_id): Path<i32>,
//     State(state): State<ServerState>,
// ) -> ApiResult<impl IntoResponse> {
//     admin_check(&identity, &state).await?;
//     match sqlx::query_as::<_, MediaNoData>(
//         "SELECT id, post_id, name, media_type, created_at FROM media WHERE post_id = $1",
//     )
//     .bind(post_id)
//     .fetch_all(&state.pool)
//     .await
//     {
//         Ok(response) => ok!(response),
//         Err(e) => Err(ApiError::werr(
//             "Error retrieving all media.",
//             StatusCode::BAD_REQUEST,
//             e,
//         )),
//     }
// }

// pub async fn get_media(
//     Path(media_id): Path<i32>,
//     State(state): State<ServerState>,
// ) -> ApiResult<impl IntoResponse> {
//     match sqlx::query_as::<_, MediaNoData>(
//         "SELECT id, post_id, name, media_type, created_at FROM media WHERE id = $1",
//     )
//     .bind(media_id)
//     .fetch_one(&state.pool)
//     .await
//     {
//         Ok(response) => ok!(response),
//         Err(e) => Err(ApiError::werr(
//             "Error retrieving all media.",
//             StatusCode::BAD_REQUEST,
//             e,
//         )),
//     }
// }
