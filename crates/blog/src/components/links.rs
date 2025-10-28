use leptos::prelude::*;

#[component]
pub fn Links() -> impl IntoView {
    let links = [
        ("www.instagram.com/nic_ol_ass", "instagram.svg"),
        ("bsky.app/profile/nicolas-frey.com", "bluesky.svg"),
        ("www.youtube.com/@microwonk", "youtube.svg"),
        ("www.github.com/Microwonk", "github.svg"),
        ("microwonk.itch.io", "itch-io.svg"),
    ];

    view! {
        <div class="flex flex-row">
            {
                links.map(|link| view! {
                    <a
                        href=format!("https://{}", link.0)
                        target="_blank"
                        class="hover:animate-pulse"
                    >
                        <img src=format!("/assets/{}", link.1) style="width: 24px; margin: 0px 10px 0px" />
                    </a>
                }).collect_view()
            }
        </div>
    }
}
