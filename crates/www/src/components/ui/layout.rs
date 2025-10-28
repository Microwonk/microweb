use leptos::prelude::*;

#[component]
pub fn Layout(
    children: Children,
    aria_label: &'static str,
    #[prop(into)] class_name: String,
    #[prop(into)] id: String,
) -> impl IntoView {
    view! {
        <section
            aria-label=aria_label
            id=id
            class=class_name.to_owned()
                + " selection:bg-nf-white selection:text-nf-dark relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6"
        >
            {children()}
        </section>
    }
}
