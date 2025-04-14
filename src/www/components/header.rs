use chrono::{DateTime, Local, Offset, Utc};
use chrono_tz::Europe::Vienna;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let utc_now: DateTime<Utc> = Utc::now();
    let vienna_time = utc_now.with_timezone(&Vienna);
    let local_time = Local::now();
    let offset_seconds =
        vienna_time.offset().fix().local_minus_utc() - local_time.offset().fix().local_minus_utc();
    let offset_hours = offset_seconds / 3600;
    let offset_minutes = (offset_seconds % 3600) / 60;
    let diff = format!("{:+03}:{:02}", offset_hours, offset_minutes.abs());
    let time_str = vienna_time.format("%H:%M").to_string();

    let dom = Resource::new(
        || (),
        async |_| format!("http://blog.{}", crate::domain().await.unwrap()),
    );

    view! {
        <header
            id="header"
            class="sticky top-0 mx-auto max-w-full selection:bg-nf-white selection:text-nf-dark relative backdrop-invert-0 z-10"
            style="backdrop-filter: blur(5px)"
        >
            <nav
                class="w-full py-2 md:py-3 bg-nf-dark border-b-nf-white/[0.35] border-b-2 px-3 md:px-5"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex items-center justify-between">
                    <li class="text-nf-white text-md md:text-lg flex uppercase gap-1">
                        <span>Nicolas</span>
                        <span class="block sm:hidden">Frey</span>
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
                    <li class="relative group text-nf-white text-xl md:text-2xl flex gap-2 items-center">
                        <img
                            src="/assets/globe.svg"
                            class="w-6 h-6 animate-[spin_3s_linear_infinite]"
                        />
                        <span class="text-sm sm:text-md uppercase font-montserrat">
                            Graz, {time_str}
                        </span>

                        <span class="font-montserrat absolute left-1/2 -translate-x-1/2 top-full mt-2 hidden group-hover:block px-2 py-1 text-sm text-nf-white bg-nf-color rounded-md opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                            {diff}
                        </span>
                    </li>
                </ul>
            </nav>
            <nav
                class="w-full py-2 md:py-3 bg-nf-white border-b-nf-white/[0.35] border-b-2 px-3 md:px-5 relative overflow-hidden"
                aria-label="Global"
            >
                <div
                    class="absolute inset-0 bg-nf-white z-0"
                    style="filter: url(#rough-paper);"
                ></div>
                <ul class="gap-4 flex-row flex items-center justify-between relative z-10">
                    <li class="hidden sm:block font-bold text-nf-dark text-md md:text-lg flex uppercase">
                        Frey
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
                        <Suspense>
                            <a
                                href=move || dom.get().unwrap_or_default()
                                class="font-montserrat text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color font-bold"
                            >
                                blog
                            </a>
                        </Suspense>
                    </li>
                </ul>
            </nav>

        </header>
    }
}
