use leptos::prelude::*;
use leptos_meta::*;

use crate::components::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Nicolas Frey - Page Not Found" />
        <Header />

        <main class="grid gap-28 lg:gap-64 mt-10 md:mt-28">
            <h2 class="font-montserrat text-[520px] text-nf-color/[.75] absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 font-medium">
                404
            </h2>
            <Layout
                id="notfound"
                aria_label="Not Found"
                class_name="flex-col"
            >
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl text-nf-white font-bold uppercase">
                    Page <br /> <span class="font-[400]">not</span>found
                </h1>
                <p class="text-xl md:text-2xl lg:text-3xl text-nf-white mt-10 lg:mt-20">
                    "Sorry, we couldn’t find the page you’re looking for" <br />
                    <a
                        class="text-md md:text-lg lg:text-xl text-nf-white mt-2 lg:mt-4 font-bold"
                        href="/"
                    >
                        "Let's"
                        go home
                    </a>
                </p>
            </Layout>
        </main>
    }
}
