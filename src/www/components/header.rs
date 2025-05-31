use chrono::{Local, Offset, Utc};
use chrono_tz::Europe::Vienna;
use leptos::prelude::*;

use crate::apps::Apps;

#[component]
pub fn Header() -> impl IntoView {
    let vienna_time = Utc::now().with_timezone(&Vienna);

    let (time, _) = signal(vienna_time.format("%H:%M").to_string());
    let (diff, set_diff) = signal("".to_string());

    Effect::new(move |_| {
        let offset_seconds = vienna_time.offset().fix().local_minus_utc()
            - Local::now().offset().fix().local_minus_utc();
        set_diff(format!(
            "{:+03}:{:02}",
            offset_seconds / 3600,                // hours
            ((offset_seconds % 3600) / 60).abs()  // minutes
        ));
    });

    view! {
        <header id="header" class="sticky top-0 mx-auto max-w-full relative z-10">
            <nav class="w-full py-2 md:py-3 bg-nf-dark px-3 md:px-5" aria-label="Global">
                <ul class="gap-4 flex-row flex items-center justify-between">
                    <li class="text-nf-white text-md md:text-lg flex uppercase gap-1">
                        <a href="/">Nicolas</a>
                        <a href="/" class="block sm:hidden">
                            Frey
                        </a>
                    </li>
                    <li class="hidden lg:flex text-nf-white text-xl md:text-2xl font-bold font-montserrat">
                        <span class="text-sm sm:text-md flex items-center gap-2">
                            <svg
                                width="8"
                                height="8"
                                viewBox="0 0 8 8"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                                class="animate-pulse"
                            >
                                <circle cx="4" cy="4" r="4" fill="var(--nf-color)"></circle>
                            </svg>
                            Making Mods and Games
                        </span>
                    </li>
                    <li class="z-10 relative group text-nf-white text-xl md:text-2xl flex gap-2 items-center">
                        <img
                            src="/assets/globe.svg"
                            class="w-6 h-6 animate-[spin_3s_linear_infinite]"
                        />
                        <span class="text-sm sm:text-md uppercase font-montserrat">
                            Graz, {move || time.get()}
                        </span>

                        <span class="font-montserrat absolute left-1/2 -translate-x-1/2 top-full mt-2 hidden group-hover:block px-2 py-1 text-sm text-nf-white bg-nf-color rounded-md opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                            {move || diff.get()}
                        </span>
                    </li>
                </ul>
            </nav>
            <nav
                class="w-full py-2 md:py-3 px-3 md:px-5 relative overflow-hidden bg-gradient-to-b from-black from-50% via-transparent via-75% to-transparent"
                aria-label="Global"
            >
                <div class="absolute inset-0 z-0 tornpaper-effect"></div>
                <ul class="gap-4 flex-row flex items-center justify-between relative z-1">
                    <li class="hidden sm:block font-bold text-nf-dark text-md md:text-lg flex uppercase">
                        <a href="/">Frey</a>
                    </li>

                    <li class="font-montserrat flex gap-4 md:gap-8 items-center w-full md:justify-end justify-center">
                        <a
                            href="/resume"
                            class="text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color font-bold"
                        >
                            resume
                        </a>
                        <a
                            href="https://github.com/Microwonk"
                            target="_blank"
                            class="font-montserrat text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color font-bold"
                        >
                            github
                        </a>
                        <a
                            href=Apps::Blog.url()
                            class="font-montserrat text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color font-bold"
                        >
                            blog
                        </a>
                    </li>
                </ul>
            </nav>

        </header>
    }
}
