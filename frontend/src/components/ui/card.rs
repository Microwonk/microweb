use leptos::*;

#[component]
pub fn Card(
    children: Children,
    name: impl Into<String>,
    #[prop(into)] class_name: String,
    #[prop(into)] style: String,
) -> impl IntoView {
    let name: String = name.into();
    view! {
        <a
            href=format!("projects/{}", &name)
            style=style
            id=&name
            class=format!("{}  overflow-hidden relative bg-nf-white rounded-[16px] md:rounded-[16px] duration-500 transition-shadow hover:shadow-md min-h-[200px]", class_name)
        >
            {children()}
        </a>
    }
}
