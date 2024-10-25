use leptos::*;

#[component]
pub fn BlogCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] link: String,
    #[prop(into)] icon: icondata::Icon,
) -> impl IntoView {
    let title_clone = title.clone();
    view! {
        <a href={format!("posts/{}", link)} class="group relative block h-64 sm:h-80 lg:h-96">
            <span class="rounded-md absolute inset-0 border-2 border-dashed border-black"></span>

            <div
                class="rounded-md relative flex h-full transform items-end border-2 border-solid border-black bg-white transition-transform group-hover:-translate-x-2 group-hover:-translate-y-2"
            >
                <div
                class="p-4 !pt-0 transition-opacity text-black group-hover:absolute group-hover:opacity-0 sm:p-6 lg:p-8"
                >
                <svg
                    x=icon.x
                    y=icon.y
                    width=64
                    height=64
                    viewBox=icon.view_box
                    stroke-linecap=icon.stroke_linecap
                    stroke-linejoin=icon.stroke_linejoin
                    stroke-width=icon.stroke_width
                    stroke=icon.stroke
                    fill=icon.fill.unwrap_or("currentColor")
                    inner_html=icon.data
                ></svg>

                <h2 class="mt-4 text-xl text-black font-medium sm:text-2xl">{title}</h2>
                </div>

                <div
                class="absolute p-4 opacity-0 transition-opacity group-hover:relative group-hover:opacity-100 sm:p-6 lg:p-8"
                >
                <h3 class="mt-4 text-xl text-black font-medium sm:text-2xl">{title_clone}</h3>

                <p class="mt-4 text-sm text-black sm:text-base">
                    {description}
                </p>

                <p class="mt-8 font-bold text-black underline decoration-dashed">Read more</p>
                </div>
            </div>
        </a>
    }
}
