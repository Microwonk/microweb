use crate::{
    components::{header::Header, side_menu::SideMenu},
    types::Post,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use logging::debug_warn;

#[derive(Params, PartialEq)]
struct TabQuery {
    tab: String,
}

#[component]
pub fn AdminPage(logged_in: ReadSignal<bool>, blog_posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let query = use_query::<TabQuery>();

    // Reactive signal for the current tab
    let (current_tab, set_current_tab) = create_signal(String::new());

    // Update the current_tab signal whenever the query changes
    create_effect(move |_| {
        if let Some(t) = query.with(|q| q.as_ref().map(|t| t.tab.clone()).ok()) {
            set_current_tab(t);
        } else {
            set_current_tab("general".to_string());
        }
    });

    let tabs = move || {
        ["General", "Blogs", "Media", "Manage", "Users"]
            .iter()
            .enumerate()
            .map(|t| {
                let lowercase = t.1.to_lowercase();
                let current = lowercase == current_tab.get(); // Compare with reactive signal
                (t.0, t.1.to_string(), lowercase, current)
            })
            .collect::<Vec<(usize, String, String, bool)>>()
    };

    view! {
        <Title text="Admin Dashboard"/>
        <Header logged_in/>
        <div class="flex h-screen flex-col justify-between border-e bg-white">
            <div class="px-4 py-6">
                <ul class="mt-6 space-y-1 list-none">
                <For
                    each=tabs
                    key=|t| t.0
                    children=move |t| {
                        let selected = if t.3 { "bg-gray-100" } else { "" };
                        let classes = format!("block rounded-lg px-4 py-2 text-sm font-medium text-gray-700 {}", selected);
                        view! {
                            <li>
                                <a
                                href=move || format!("?tab={}", t.2)
                                class={classes}
                                >
                                {move || t.1.clone()}
                                </a>
                            </li>
                        }
                    }
                />
                </ul>
            </div>
        </div>
    }
}
