use leptos::prelude::*;

#[component]
pub fn Layout(
    children: Children,
    aria_label: &'static str,
    class_name: String,
    id: String,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <section
            aria-label=aria_label
            id=id
            class=class_name.to_owned() + spacer
                + "selection:bg-nf-white selection:text-nf-dark relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6"
        >
            {children()}
        </section>
    }
}
