use leptos::*;

#[component]
pub fn ItchEmbed(embed_link: String, link: String, title: String) -> impl IntoView {
    view! {
        <iframe frameborder="0" src=embed_link class="mt-10 fade-y-trans rounded-xl"
                width="960"
                height="540">
            <a href=link>
                {title}
            </a>
        </iframe>
    }
}
