use crate::www::components::*;

use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout
            id="about".to_string()
            aria_label="About"
            class_name="flex flex-col gap-10 lg:gap-32".to_string()
        >
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in">
                <div class="lg:col-span-2">
                    <div class="text-md lg:text-lg leading-about text-nf-white uppercase">
                        <span class="uppercase">About</span>
                    </div>

                </div>
                <p class="font-montserrat lg:col-span-5 min-w-full text-xl lg:text-3xl leading-p lg:leading-largep text-nf-white">
                    "I am dedicated to developing robust and enduring software solutions.
                    With a strong foundation in multiple programming languages and paradigms—many of which I have self-taught—I bring extensive experience across backend, frontend, mobile, game, and graphics programming.
                    I am a fast learner with a demonstrated ability to manage and lead projects of varying scale and complexity.
                    My focus is on writing clean, maintainable, and efficient code in modern, safe, and high-performance languages such as Rust and C#, with an interest in exploring Go in the future."
                    <br /> <br />
                    As a dedicated Linux enthusiast and gamer, I have a deep passion for game development and the open source philosphy.
                </p>
            </div>

            <Experience />
            <Skills />
        </Layout>
    }
}
