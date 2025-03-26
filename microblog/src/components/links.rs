use leptos::prelude::*;

#[component]
pub fn links() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <a href="https://www.instagram.com/nic_ol_ass" target="_blank" class="hover:animate-pulse">
                <img src="https://www.nicolas-frey.com//assets/instagram.svg" style="width: 24px; margin: 0px 10px 0px"/>
            </a>
            <a href="https://bsky.app/profile/nicolas-frey.com" target="_blank" class="hover:animate-pulse">
                <img src="https://www.nicolas-frey.com/assets/bluesky.svg" style="width: 24px; margin: 0px 10px 0px"/>
            </a>
            <a href="https://www.youtube.com/@microwonk" target="_blank" class="hover:animate-pulse">
                <img src="https://www.nicolas-frey.com/assets/youtube.svg" style="width: 24px; margin: 0px 10px 0px"/>
            </a>
            <a href="https://www.github.com/Microwonk" target="_blank" class="hover:animate-pulse">
                <img src="https://www.nicolas-frey.com/assets/github.svg" style="width: 24px; margin: 0px 10px 0px"/>
            </a>
            <a href="https://microwonk.itch.io" target="_blank" class="hover:animate-pulse">
                <img src="https://www.nicolas-frey.com/assets/itch-io.svg" style="width: 24px; margin: 0px 10px 0px"/>
            </a>
        </div>
    }
}
