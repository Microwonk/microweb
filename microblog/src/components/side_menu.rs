use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn SideMenu() -> impl IntoView {
    let (tabs, set_tabs) = signal(Vec::new());

    set_tabs(
        ["Blogs", "Media", "Users"]
            .iter()
            .enumerate()
            .map(|t| (t.0, t.1.to_string(), t.1.to_lowercase()))
            .collect(),
    );

    view! {
        <div class="flex h-full flex-col justify-between">
            <div class="px-4 py-6">
                <ul class="mt-6 space-y-1 list-none">
                    <For
                        // Reactively fetch the current tabs
                        each=move || tabs.get()
                        key=|t| t.0
                        children=move |t| {
                            let navigate = use_navigate();
                            view! {
                                <li>
                                    <button
                                        on:click=move |_| navigate(
                                            format!("/admin?tab={}", t.2).as_str(),
                                            Default::default(),
                                        )
                                        class="text-left w-full block rounded-lg px-4 py-2 border-none text-sm font-medium hover:bg-gray-100 hover:text-gray-700 text-gray-500 bg-white"
                                    >
                                        {move || t.1.clone()}
                                    </button>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>
    }
}
