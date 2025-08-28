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
                        <span class="uppercase">"About —"</span>
                    </div>

                </div>
                <p class="font-montserrat lg:col-span-5 min-w-full text-xl lg:text-3xl leading-p lg:leading-largep text-nf-white">
                    "Hello there! My name is Nicolas and I love anything that has to do with computers."
                    <br /><br /> "Literally." <br /><br />
                    "From developing graphics engines, operating systems, websites and games to tinkering with microcontrollers and building PCs - I love to do it all.
                    With a strong foundation in multiple programming languages and paradigms and knowhow of operations systems, virtually nothing stands in my way to make my ideas into reality."
                    <br /> <br />
                    "I also love modding and reverse engineering games (like minecraft and mario odyssey) and have a passion for speedrunning."
                    <br /> <br />
                    "As a fast learner with a demonstrated ability to manage and lead projects of varying scale and complexity I'm fit for any obstacle in my way. My focus is on writing clean, maintainable, and efficient code in modern, safe, and high-performance languages such as Rust."
                    <br /> <br />
                    "I am a strong believer in the open source philosophy and a Linux enthusiast."
                </p>
            </div>

            <Experience />
            <Skills />
        </Layout>
    }
}
