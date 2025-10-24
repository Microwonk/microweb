use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(into)] class_name: String,
    #[prop(into)] href: RwSignal<String>,
) -> impl IntoView {
    let after_label = label.clone();

    view! {
        <button class=format!("button button-cta {}", class_name) role="button">
            <a target="_blank" href=href>
                <span class="button-cta-border"></span>
                <span class="button-cta-ripple">
                    <span></span>
                </span>
                <span class="button-cta-title">
                    <span data-text=label>{after_label}</span>
                </span>
            </a>
        </button>
    }
}
