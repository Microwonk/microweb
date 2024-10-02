use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn Page404() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <h1 class="text-6xl font-bold">"404"</h1>
                <p class="mt-4 text-xl">"Oops! Page Not Found."</p>
                <p class="mt-2">"The page you are looking for might have been removed or is temporarily unavailable."</p>
                <button on:click=move |_| navigate("/", NavigateOptions::default()) class="mt-6 inline-block px-4 py-2 rounded transition duration-300">"Go Back Home"</button>
            </div>
        </div>
    }
}
