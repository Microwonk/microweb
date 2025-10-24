use crate::www::components::ui::layout::Layout;

use leptos::prelude::*;
use leptos_use::use_interval_fn;

#[component]
pub fn Hero() -> impl IntoView {
    let (ascii_art, write_ascii_art) = signal(r#"¯\_(ツ)_/¯"#);

    let items = [
        r#"≽^•⩊•^≼"#,
        r#"ᵕ̈"#,
        r#"(つ╥﹏╥)つ"#,
        r#"¯\_(ツ)_/¯"#,
        r#"(づ๑•ᴗ•๑)づ♡"#,
        r#"ʕ•ﻌ•`ʔ"#,
        r#"ദ്ദി（• ˕ •マ.ᐟ"#,
        r#"(•؎ •)"#,
        r#"*ฅ^•ﻌ•^ฅ*"#,
        r#"ᕙ(⇀‸↼‶)ᕗ"#,
        r#"( • ᴗ - ) ✧"#,
        r#"(·•᷄∩•᷅ )"#,
        r#"( ◡̀_◡́)ᕤ"#,
        r#"◝(ᵔᗜᵔ)◜"#,
        r#"ʕ º ᴥ ºʔ"#,
        r#"₍^. .^₎Ⳋ"#,
        r#"૮⍝• ᴥ •⍝ა"#,
        r#"૮₍´｡ᵔ ꈊ ᵔ｡`₎ა"#,
        r#"≽(•⩊ •マ≼"#,
        r#"(•˕ •マ.ᐟ"#,
        r#"( •⌄• )"#,
        r#"/ᐠ - ˕ -マ"#,
    ];

    let _ = use_interval_fn(
        move || {
            if let Some(upcoming_skill) = items
                .into_iter()
                .cycle()
                .skip_while(|s| *s != ascii_art.get_untracked())
                .nth(1)
            {
                write_ascii_art(upcoming_skill);
            }
        },
        1000,
    );

    view! {
        <Layout id="home" aria_label="Hero" class_name="">
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
                                {ascii_art}
                            </span>
                        }
                    }}
                </div>
                <br />
                <div class="animated-title rotate-[0.1rad]">
                    <span class="p-3 md:p-5 tornpaper-effect text-5xl font-kamikaze sm:text-8xl lg:text-9xl xl:text-10xl text-nf-dark animated-hero-element uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading relative">
                        "That Works"
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
