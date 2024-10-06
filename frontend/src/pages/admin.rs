use crate::{
    components::{header::Header, side_menu::SideMenu},
    pages::loading::LoadingPage,
    types::{Post, User},
    util::Api,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct TabQuery {
    tab: String,
}

#[component]
pub fn AdminPage(logged_in: ReadSignal<bool>, blog_posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let query = use_query::<TabQuery>();

    let (current_tab, set_current_tab) = create_signal(String::new());
    let (users, set_users) = create_signal(Vec::new());

    create_effect(move |_| {
        if let Some(t) = query.with(|q| q.as_ref().map(|t| t.tab.clone()).ok()) {
            set_current_tab(t);
        } else {
            set_current_tab("general".to_string());
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
                            _ => view! { <LoadingPage/> }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn UserSection(
    users: ReadSignal<Vec<User>>,
    set_users: WriteSignal<Vec<User>>,
) -> impl IntoView {
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
                    // <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">PWHash</th>
                    <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">Created At</th>
                    // <th class="px-4 py-2"></th>
                </tr>
                </thead>

                <tbody class="divide-y divide-gray-200">
                <For
                        each=move || users.get()
                        key=|user| user.id
                        children=move |user: User| {
                            view! {
                                <tr class="odd:bg-gray-50">
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.id}</td>
                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{user.name}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.email}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{if user.admin { "yes "} else { "no" }}</td>
                                    // <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.passwordhash}</td>
                                    <td class="whitespace-nowrap px-4 py-2 text-gray-700">{user.created_at.to_string()}</td>
                                    <td class="whitespace-nowrap px-4 py-2">
                                    // <a
                                    //     href="#"
                                    //     class="inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                                    // >
                                    //     View
                                    // </a>
                                    </td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
