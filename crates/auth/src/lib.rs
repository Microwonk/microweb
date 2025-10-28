pub mod app;
mod login;
mod register;

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
pub struct ReturnUrlQuery {
    return_url: String,
}

#[component]
pub fn Page404() -> impl IntoView {
    view! {
        <Title text="Page not Found" />
        <div class="grid h-screen place-content-center px-4">
            <div class="text-center">
                <h1 class="text-10xl text-black">404</h1>
                <h2 class="mt-6 text-2xl font-bold tracking-tight text-black sm:text-4xl">
                    Uh-oh!
                </h2>

                <p class="mt-4 text-black">"This page doesn't exist."</p>
                <a
                    href=common::apps::Apps::Www.url()
                    class="mt-6 inline-block no-underline rounded bg-nf-color px-5 py-3 text-sm font-medium text-white focus:outline-none focus:ring"
                >
                    Go Back Home
                </a>
            </div>
        </div>
    }
}
