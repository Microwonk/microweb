use leptos::prelude::*;

#[component]
pub fn DropDown(
    actions: Vec<(
        impl Into<String>,
        impl Into<String>,
        Option<impl Into<String>>,
    )>,
) -> impl IntoView {
    #[derive(Clone)]
    struct Action {
        id: usize,
        name: String,
        link: String,
        target: Option<String>,
    }

    let actions: Vec<Action> = actions
        .into_iter()
        .enumerate()
        .map(|e| Action {
            id: e.0,
            name: e.1 .0.into(),
            link: e.1 .1.into(),
            target: e.1 .2.map(|s| s.into()),
        })
        .collect();

    view! {
        <div
            class="absolute end-6 z-10 mt-2 w-32 rounded-md border border-gray-100 bg-white shadow-lg"
            role="menu"
        >
            <div class="p-2">
                <For
                    each=move || actions.clone()
                    key=|a| a.id
                    children=move |action: Action| {
                        view! {
                            <a
                                href={action.link}
                                class="block rounded-lg px-4 py-2 text-end text-xl text-gray-500 hover:bg-gray-50 hover:text-gray-700"
                                target={
                                    if let Some(target) = action.target {
                                        target
                                    } else {
                                        "_self".to_owned()
                                    }
                                }
                                role="menuitem"
                            >
                                {action.name}
                            </a>
                        }
                    }
                />
            </div>
        </div>
    }
}
