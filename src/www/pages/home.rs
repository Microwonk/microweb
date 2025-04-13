use crate::www::components::*;

use crate::www::sections::*;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <BackGround />
        <Title text="Nicolas Frey" />
        <Header />
        // <Profile/>
        // <BackGround/>
        <main class="grid gap-20 md:gap-28 lg:gap-64 mt-12 md:mt-20 xl:mt-28">
            <Hero />
            <About />
            <Contact />
            <Info />
        </main>
    }
}
