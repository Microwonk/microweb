use crate::{
    components::{header::Header, side_menu::SideMenu},
    pages::loading::LoadingPage,
    types::{Media, Post, User, UserUpdate},
    util::Api,
};
use chrono::Utc;
use html::Div;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{
    use_drop_zone_with_options, UseDropZoneEvent, UseDropZoneOptions, UseDropZoneReturn,
};

#[derive(Params, PartialEq)]
struct TabQuery {
    tab: String,
}

#[component]
pub fn AdminPage(logged_in: ReadSignal<bool>, blog_posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let query = use_query::<TabQuery>();

    let (current_tab, set_current_tab) = create_signal(String::new());
    let (users, set_users) = create_signal(Vec::new());
    let (media, set_media) = create_signal(Vec::new());

    create_effect(move |_| {
        if let Some(t) = query.with(|q| q.as_ref().map(|t| t.tab.clone()).ok()) {
            set_current_tab(t);
        } else {
            use_navigate()("/admin?tab=general", Default::default());
            // set_current_tab("general".to_string());
        }
    });

    view! {
        <Title text="Admin Dashboard"/>
        <div class="flex flex-col min-h-screen"> // Container that ensures full screen height
            <Header logged_in/>
            <div class="flex-grow grid grid-cols-6 gap-4"> // Grid takes up the remaining space
                <div class="md:col-span-2 lg:col-span-1">
                    <SideMenu/>
                </div>
                <div class="md:col-span-4 lg:col-span-5">
                    <div class="p-6">
                        {move || match current_tab.get().as_str() {
                            "users" => view! { <UserSection users set_users/>},
                            "media" => view! { <MediaSection media set_media blog_posts/>},
                            "blogs" => view! { <BlogSection blog_posts/>},
                            "general" => view! { <GeneralSection/> },
                            _ => view! { <LoadingPage/> }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn GeneralSection() -> impl IntoView {
    view! {
        TODO
    }
}

#[component]
pub fn UserSection(
    users: ReadSignal<Vec<User>>,
    set_users: WriteSignal<Vec<User>>,
) -> impl IntoView {
    let edit_row = create_rw_signal(None::<i32>);
    let username = create_rw_signal("".to_string());
    let email = create_rw_signal("".to_string());
    spawn_local(async move {
        set_users(Api::all_users().await.unwrap_or_default());
    });
    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                <tr>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Name</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Email</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Admin</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Created At</th>
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
                            <Show when=move || edit_row.get().is_some_and(|row| row == user.id) fallback=move || view! {
                                <UserRow user=user_c.clone() edit_row/>
                            }>
                                <UserEditRow user=user.clone() username email/>
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
    #[prop(into)] user: MaybeSignal<User>,
    #[prop(into)] edit_row: RwSignal<Option<i32>>,
) -> impl IntoView {
    let user = user.get();
    view! {
        <tr class="odd:bg-gray-50">
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.id}</td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{user.name}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.email}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{if user.admin { "yes "} else { "no" }}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.created_at.to_string()}</td>
            <td class="whitespace-nowrap px-4 py-2">
            <button
                class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                on:click=move |_| {
                    spawn_local(async move {
                        if (Api::delete_user(user.id).await).is_ok() {
                            // refresh
                            web_sys::window().unwrap().location().reload().unwrap();
                        };
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
    #[prop(into)] user: MaybeSignal<User>,
    #[prop(into)] username: RwSignal<String>,
    #[prop(into)] email: RwSignal<String>,
) -> impl IntoView {
    let user = user.get();
    username.set(user.name.clone());
    email.set(user.email.clone());

    view! {
        <tr class="bg-nf-color">
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.id}</td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
            <input on:input=move |ev| {
                let new_value = event_target_value(&ev);
                username.set(new_value);
            }
            prop:value=user.name>
            </input>
            </td>
            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
            <input on:input=move |ev| {
                let new_value = event_target_value(&ev);
                email.set(new_value);
            }
            prop:value=user.email>
            </input>
            </td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{if user.admin { "yes "} else { "no" }}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.created_at.to_string()}</td>
            <td class="whitespace-nowrap px-4 py-2">
            <button
                class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                on:click=move |_| {
                    spawn_local(async move {
                        if (Api::delete_user(user.id).await).is_ok() {
                            // refresh
                            web_sys::window().unwrap().location().reload().unwrap();
                        };
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
                        if (Api::update_user(user.id, UserUpdate { name: username.get(), email: email.get() }).await).is_ok() {
                            // refresh
                            web_sys::window().unwrap().location().reload().unwrap();
                        };
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
    set_media: WriteSignal<Vec<Media>>,
    blog_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    spawn_local(async move {
        set_media(Api::all_media().await.unwrap_or_default());
    });

    let (post_id, set_post_id) = create_signal(0);
    let (get_files, set_files) = create_signal(Vec::new());

    let drop_zone_el = NodeRef::<Div>::new();

    let on_drop = move |event: UseDropZoneEvent| {
        set_files(event.files);
    };

    let UseDropZoneReturn { files, .. } =
        use_drop_zone_with_options(drop_zone_el, UseDropZoneOptions::default().on_drop(on_drop));

    view! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
                <thead class="text-left">
                <tr>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">ID</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Post</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Name</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Path</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">MIME Type</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Created At</th>
                    <th class="px-4 py-2"></th>
                </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                <For
                        each=move || media.get()
                        key=|media| media.id
                        children=move |media: Media| {
                            let post = blog_posts.get().iter().find(|&p| p.id == media.post_id).cloned();
                            let path = format!("https://microblog.shuttleapp.rs/upload/{}", media.id);
                            view! {
                                <tr class="odd:bg-gray-50">
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{media.id}</td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{if let Some(p) = post { p.title } else {format!("No Post? ID {}", media.post_id)}}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{media.name}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <a href={path} target="_blank">{format!("/upload/{}", media.id)}</a>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{media.media_type}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{media.created_at.to_string()}</td>
                                    <td class="whitespace-nowrap px-4 py-2">
                                    <button
                                        class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                                        on:click=move |_| {
                                            spawn_local(async move {
                                                if (Api::delete_media(media.id).await).is_ok() {
                                                    // refresh
                                                    web_sys::window().unwrap().location().reload().unwrap();
                                                };
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
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{move || {
                        media
                            .get()
                            .iter()
                            .map(|m| m.id)
                            .max().map(|i| i + 1)
                            .unwrap_or_default()
                    }}</td>
                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                        <select
                            on:change=move |ev| {
                                let new_value = event_target_value(&ev);
                                set_post_id(new_value.parse().unwrap());
                            }
                            prop:value=move || post_id.get().to_string()
                        >
                            <option value=0 hidden>Please select a post</option>
                            {move || {
                                blog_posts.get().iter().cloned().map(|p| view! {
                                    <option value={p.id} on:click=move |ev| {
                                        let new_value = event_target_value(&ev);
                                        set_post_id(new_value.parse().unwrap());
                                    }>
                                        {p.title}
                                    </option>
                                }).collect_view()
                            }}
                        </select>
                    </td>
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                        <div node_ref=drop_zone_el class="flex flex-col w-full min-h-[200px] h-auto bg-gray-400/10 justify-center items-center pt-6">
                            "Drop files here"
                            <For each=files key=|f| f.name() let:file>
                                <div class="w-200px pa-6">
                                    <p>Name: {file.name()}, Size: {file.size()}</p>
                                </div>
                            </For>
                        </div>
                    </td>
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{move || {
                        format!("/upload/{}",
                        media
                                .get()
                                .iter()
                                .map(|m| m.id)
                                .max().map(|i| i + 1)
                                .unwrap_or_default()
                            )
                    }}</td>
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">?</td>
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{move || {
                        Utc::now().naive_local().to_string()
                    }}</td>
                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                        <button
                            class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                            on:click=move |_| {
                                let id = post_id.get();
                                if blog_posts.get().iter().all(|p| p.id != id) {
                                    return;
                                }
                                let files = get_files.get();
                                spawn_local(async move {
                                    match Api::upload(id, files).await {
                                        Ok(_results) => {
                                            // refresh
                                            web_sys::window().unwrap().location().reload().unwrap();
                                        },
                                        Err(_e) => {}
                                    }
                                });
                            }
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
pub fn BlogSection(blog_posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let (max_id, set_max_id) = create_signal(0);
    create_effect(move |_| {
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
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Created At</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Released</th>
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
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{post.id}</td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{post.title}</td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{post.created_at.to_string()}</td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{if post.released { "yes "} else { "no" }}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <a
                                            class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                            href={format!("/admin/posts/{}", post.slug)}
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
                                                    match if post.released { Api::post_unrelease(post.id).await } else { Api::post_release(post.id).await } {
                                                        Ok(_results) => {
                                                            // refresh
                                                            web_sys::window().unwrap().location().reload().unwrap();
                                                        },
                                                        Err(e) => {
                                                            log::error!("{:?}", e);
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            {move || {if post.released { "Unrelease" } else { "Release" } } }
                                        </button>
                                    </td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                        <button
                                            class="border-none inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    match Api::post_delete(post.id).await {
                                                        Ok(_results) => {
                                                            // refresh
                                                            web_sys::window().unwrap().location().reload().unwrap();
                                                        },
                                                        Err(e) => {
                                                            log::error!("{:?}", e);
                                                        }
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
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">{move || max_id.get() + 1}</td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Add a New Blog Post!</td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{move || {
                            Utc::now().naive_local().to_string()
                        }}</td>
                        <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">nope</td>
                        <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                        <button
                            class="border-none inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                            on:click=move |_| {
                                spawn_local(async move {
                                    match Api::create_post().await {
                                        Ok(_results) => {
                                            // refresh
                                            web_sys::window().unwrap().location().reload().unwrap();
                                        },
                                        Err(e) => {
                                            log::error!("{:?}", e);
                                        }
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
