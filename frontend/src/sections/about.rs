use crate::components::*;

use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout
            id="about".to_string()
            aria_label="About"
            class_name="flex flex-col gap-10 lg:gap-32".to_string()
        >
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in">
                <div class="lg:col-span-3">
                    <div class="text-sm lg:text-md leading-about text-nf-white uppercase">
                        <span class="uppercase">About</span>
                    </div>

                </div>
                <p class="font-montserrat lg:col-span-4 min-w-full text-xl lg:text-3xl leading-p lg:leading-largep text-nf-white">
                I am passionate about creating long-lasting software solutions.
                With a strong foundation in multiple programming languages and paradigms,
                mostly self-taught, I have extensive experience in backend,
                frontend, mobile, game, and graphics programming.
                I am a quick learner with a proven ability to manage and lead projects of varying sizes and complexity.
                Skilled in automation and testing, I prioritize writing readable and maintainable code.
                <br/>
                <br/>
                A dedicated Linux enthusiast and gamer, I have a deep passion for game development.
                </p>
            </div>

            <Experience/>
            <Skills/>
        </Layout>
    }
}
