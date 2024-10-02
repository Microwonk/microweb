use leptos::*;

#[component]
pub fn CardLink(
    children: Children,
    class_name: String,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <div
            style=style
            class=class_name.to_owned() + spacer
                + "overflow-hidden relative bg-gray-9 rounded-[20px] md:rounded-[57px] duration-500 transition-shadow hover:shadow-md"
        >
            <div class="w-full h-full rounded-full flex flex-row justify-center items-center">
                {children()}
            </div>
        </div>
    }
}

