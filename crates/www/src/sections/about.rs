use crate::components::*;

use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout
            id="about"
            aria_label="About"
            class_name="flex flex-col gap-10 lg:gap-32"
        >
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in">
                <div class="lg:col-span-2">
                    <div class="text-md lg:text-lg leading-about text-nf-white uppercase">
                        <span class="uppercase">"About —"</span>
                    </div>

                </div>
                <p class="font-montserrat lg:col-span-5 min-w-full text-xl lg:text-3xl leading-p lg:leading-largep text-nf-white">
                    "Hello there! I'm Nicolas, a Software Developer and Linux fan, over all I love anything that has to do with computers."
                    <br /><br />
                    "From developing graphics engines, operating systems, websites and games to tinkering with my home lab and building PCs - I love to do it all."
                    <br /> <br />
                    "I also love modding and hacking retro games and like to speedrun them too. 👾"
                    <br /> <br />
                    "Currently I'm working as a Software Developer at "
                    <a href="https://www.proxmox.com/" class="nf-color">Proxmox</a>
                    " where I focus on writing clean, maintainable, and efficient code in my favorite language: Rust."
                    <br /> <br />
                    "I'm an open sourcerer and Archᵇᵗʷ Linux user through and through."
                </p>
            </div>

            <Experience />
            <Skills />
        </Layout>
    }
}
