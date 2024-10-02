use crate::components::*;

use crate::sections::*;

use leptos::*;
use leptos_meta::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Nicolas Frey"/>
        <Header/>
        <Profile/>
        <BackGround/>
        <main class="grid gap-20 md:gap-28 lg:gap-64 mt-12 md:mt-20 xl:mt-28">
            <Hero/>
            <About/>
            <Features/>
            <Contact/>
            <Info/>
        </main>
    }
}
