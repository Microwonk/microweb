use leptos::*;

#[component]
pub fn SideMenu(#[prop(into)] tab: ReadSignal<Option<String>>) -> impl IntoView {
    let (tabs, set_tabs) = create_signal(Vec::new());
    create_effect(move |_| {
        set_tabs(
            ["General", "Blogs", "Media", "Manage", "Users"]
                .iter()
                .enumerate()
                .map(|t| {
                    let lowercase = t.1.to_lowercase();
                    let current = lowercase.as_ref() == tab.get().unwrap_or("general".to_string());
                    (t.0, t.1.to_string(), lowercase, current)
                })
                .collect(),
        );
    });
    view! {
        <div class="flex h-screen flex-col justify-between border-e bg-white">
            <div class="px-4 py-6">

                <ul class="mt-6 space-y-1 list-none">
                <For
                    each=move || tabs.get()
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
