use crate::components::ui::layout::Layout;

use gloo_timers::callback::Interval;
use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    let (skill, write_skill) = create_signal("Rust");

    create_effect(move |_| {
        let items = vec![
            "Rust", "Java", "Docker", "Bevy", "C++", "C#", "Vulkan", "PHP", "JS", "Vue",
        ];
        let clone_items = items.clone();
        let timer = Interval::new(2000, move || {
            let upcoming_skill = clone_items
                .iter()
                .cycle()
                .skip_while(|&s| s != &skill.get_untracked())
                .nth(1)
                .unwrap_or(&"Rust");

            write_skill(upcoming_skill);
        });

        move || timer.forget()
    });

    view! {
        <Layout id="home".to_string() aria_label="Hero" class_name="".to_string()>
            <h1 class="text-center w-full">
                <div class="animated-title">
                    <span class="text-6xl sm:text-8xl lg:text-9xl xl:text-10xl text-nf-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading  sm:tracking-heading">
                        "Making"
                    </span>
                </div>
                <br/>
                <div class="animated-title text-center">
                    <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-nf-color animated-title-element font-bold break-all uppercase leading-smallheading sm:leading-mediumheading  xl:leading-heading tracking-smallheading sm:tracking-heading">
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
                <br/>

                <div class="animated-title">
                    <span class="text-6xl sm:text-8xl lg:text-9xl xl:text-10xl text-nf-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading relative">
                        "That Lasts"
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <a class="font-montserrat text-sms sm:text-sm lg:text-sm xl:text-base text-nf-color animated-title-element uppercase relative"
                        href="https://microwonk.itch.io/"
                        target="_blank">
                        "(And Games)🔗"
                    </a>
                </div>
            </h1>
            <div class="absolute hidden md:block left-5 -bottom-20">
                <span class="scroll-icon hero">
                    <span class="scroll-icon__dot"></span>
                </span>
            </div>
        </Layout>
    }
}
