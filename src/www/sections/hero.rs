use crate::www::components::ui::layout::Layout;

use leptos::prelude::*;
use leptos_use::use_interval_fn;

#[component]
pub fn Hero() -> impl IntoView {
    let (skill, write_skill) = signal("Rust");

    let items = [
        "Rust",
        "Java",
        "Javascript",
        "C",
        "C++",
        "C#",
        "Vulkan",
        "PHP",
    ];

    let _ = use_interval_fn(
        move || {
            let upcoming_skill = items
                .iter()
                .cycle()
                .skip_while(|&s| s != &skill.get_untracked())
                .nth(1)
                .unwrap_or(&"Rust");

            write_skill(upcoming_skill);
        },
        1000,
    );

    view! {
        <Layout id="home".to_string() aria_label="Hero" class_name="".to_string()>
            <h1 class="text-center w-full">
                <div class="animated-title -rotate-[0.142rad]">
                    <span class="p-3 md:p-5 tornpaper-effect text-5xl font-kamikaze sm:text-8xl lg:text-9xl xl:text-10xl text-nf-dark animated-hero-element uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading">
                        "Making"
                    </span>
                </div>
                <br />
                <div class="animated-title text-center">
                    <span class="p-5 md:p-7 tornpaper-effect text-5xl font-kamikaze sm:text-8xl lg:text-9xl xl:text-10xl text-nf-color animated-hero-element break-all uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading">
                        "Software"
                    </span>
                    {move || {
                        view! {
                            <span class="font-montserrat sm:block hidden animated-flip-up absolute text-nf-color top-[-30px] z-100 right-0 text-sm md:text-md lg:text-xl">
                                {format!("</{}>", skill.get())}
                            </span>
                        }
                    }}
                </div>
                <br />
                <div class="animated-title rotate-[0.1rad]">
                    <span class="p-3 md:p-5 tornpaper-effect text-5xl font-kamikaze sm:text-8xl lg:text-9xl xl:text-10xl text-nf-dark animated-hero-element uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading relative">
                        "That Lasts"
                    </span>
                </div>
                <br />
            </h1>

            <div class="absolute hidden md:block left-5 -bottom-20">
                <span class="scroll-icon hero">
                    <span class="scroll-icon__dot"></span>
                </span>
            </div>
        </Layout>
    }
}
