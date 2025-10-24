use crate::models::*;
use crate::{
    apps::Apps,
    blog::{
        app::{GlobalState, GlobalStateStoreFields},
        components::{header::Header, side_menu::SideMenu},
        pages::loading::LoadingPage,
    },
};
use chrono::Utc;
use leptos::prelude::*;
use leptos::{Params, task::spawn_local};
use leptos_meta::*;
use leptos_router::hooks::{use_navigate, use_query};
use leptos_router::params::Params;
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
    .fetch_all(crate::database::db())
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
        .fetch_all(crate::database::db())
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
        .execute(crate::database::db())
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
    .execute(crate::database::db())
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
        .execute(crate::database::db())
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
    .execute(crate::database::db())
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
    .execute(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while releasing post: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not release post.")
    })
    .map(|r| r.rows_affected())
}

#[server(GetFilesAction, "/api/admin", "GetJson", endpoint = "files")]
#[tracing::instrument]
pub async fn get_files() -> Result<Vec<File>, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    if !extract::<Extension<User>>().await.is_ok_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized."));
    };

    let Some(dir) = sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE dir_path = $1")
        .bind("/~/blogs")
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None)
    else {
        return Err(ServerFnError::ServerError("Not found.".into()));
    };

    crate::files::get_directory_contents(Some(dir.id))
        .await
        .map(|dir| dir.files)
        .map_err(|_| ServerFnError::ServerError("Not found.".into()))
}

#[component]
pub fn AdminPage() -> impl IntoView {
    let query = use_query::<TabQuery>();

    let store = expect_context::<Store<GlobalState>>();

    let (updated, set_updated) = signal(0u32);

    let (current_tab, set_current_tab) = signal(String::new());
    let (blog_posts, set_blog_posts) = signal(Vec::new());
    let (users, set_users) = signal(Vec::new());
    let (contained_files, set_files) = signal(Vec::new());

    let posts_res = Resource::new(
        move || updated.get(),
        |_| async move { get_posts().await.unwrap_or(Vec::new()) },
    );

    let users_res = Resource::new(
        move || updated.get(),
        |_| async move { get_users().await.unwrap_or(Vec::new()) },
    );

    let files_res = Resource::new(
        move || updated.get(),
        |_| async move { get_files().await.unwrap_or(Vec::new()) },
    );

    Effect::new(move |_| {
        if !store.user().get().is_some_and(|u| u.is_admin) {
            use_navigate()("/", Default::default());
        }

        if let Some(Some(t)) = query.with(|q| q.as_ref().map(|t| t.tab.clone()).ok()) {
            set_current_tab(t);
        }

        set_blog_posts(posts_res.get().unwrap_or(Vec::new()));
        set_users(users_res.get().unwrap_or(Vec::new()));
        set_files(files_res.get().unwrap_or(Vec::new()));
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
                            "files" => view! { <FilesSection contained_files /> }.into_any(),
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
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
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
pub fn FilesSection(contained_files: ReadSignal<Vec<File>>) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                    <tr>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Name</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Path</th>
                        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                            MIME Type
                        </th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                    <For
                        each=move || contained_files.get()
                        key=|f| f.id.to_string()
                        children=move |f: File| {
                            let path = format!("{}/f{}", Apps::Files.url(), f.file_path);
                            view! {
                                <tr class="odd:bg-gray-50">
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {f.id.to_string()}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {f.file_name}
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <a href=path target="_blank">
                                            {path.clone()}
                                        </a>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        {f.mime_type}
                                    </td>
                                </tr>
                            }
                        }
                    />
                    <tr class="odd:bg-gray-50"></tr>
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
                                            .unwrap_or_default()}
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
