use crate::www::components::*;

use crate::www::sections::*;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Nicolas Frey" />
        <Header />

        <main class="flex flex-col gap-y-20 md:gap-y-28 lg:gap-y-64 mt-12 md:mt-20 xl:mt-28">
            <Hero />
            <About />
            <Contact />
            <Info />
        </main>
    }
}
