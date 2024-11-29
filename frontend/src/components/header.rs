use leptos::*;

use crate::types::Profile;

#[component]
pub fn Header(logged_in: ReadSignal<bool>, user: ReadSignal<Option<Profile>>) -> impl IntoView {
    view! {
        <header
            id="header"
            class="sticky top-0 mx-auto max-w-full selection:bg-nf-white selection:text-nf-dark relative backdrop-invert-0 z-10"
            style="backdrop-filter: blur(5px)"
        >
            <nav
                class="w-full bg-nf-dark py-2 md:py-3 border-b-nf-white/[0.35] border-b-2 px-3 md:px-5"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex items-center justify-between">
                    <li class="text-nf-white font-rosmatika text-md md:text-lg flex uppercase gap-1">
                        <a href="/">
                            <span>"Nicolas'"</span>
                            <span class="block sm:hidden font-rosmatika">Blog</span>
                        </a>
                    </li>
                </ul>
            </nav>
            <nav
                class="w-full py-2 md:py-3 bg-transparent border-b-nf-white/[0.35] border-b-2 px-3 md:px-5"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex justify-between">
                    <li class="font-rosmatika hidden sm:block text-nf-dark text-md md:text-lg flex uppercase hover:text-nf-color transition">
                        <a href="/">
                            Blog
                        </a>
                    </li>

                    <li class="font-montserrat flex gap-4 md:gap-8 items-center w-full md:justify-end justify-center">
                        <a
                            href="https://www.nicolas-frey.com"
                            class="text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color"
                            target="_blank"
                        >
                            about
                        </a>
                        <a
                            href="/feed"
                            class="font-montserrat text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color "
                        >
                            feed
                        </a>
                        <Show when=move || !logged_in.get() fallback=move || view!{
                            <a
                                class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color"
                                href="/logout"
                            >
                                <span class="absolute inset-0 border border-current"></span>
                                <span
                                class="block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur"
                                >
                                logout
                                </span>
                            </a>

                            <div class="group relative inline-block w-1/6">
                                <div class="experience experience-cta">
                                    <span class="experience-cta-border"></span>
                                    <span class="experience-cta-ripple">
                                        <span></span>
                                    </span>
                                    <span class="experience-cta-title">
                                        <span
                                            data-text=move || user.get().unwrap_or_default().email
                                            class="justify-between flex-row w-full"
                                        >
                                            {move || {
                                                user.get().unwrap_or_default().name
                                            }}
                                        </span>
                                    </span>
                                </div>
                            </div>
                            // <a class="text-black text-sm sm:text-lg px-5 py-2.5">
                            //     {move || {
                            //         user.get().unwrap_or_default().name
                            //     }}
                            // </a>
                        }>
                            <a
                                class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color"
                                href="/login"
                            >
                                <span class="absolute inset-0 border border-current"></span>
                                <span
                                class="block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur"
                                >
                                login
                                </span>
                            </a>

                            <a
                                class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color"
                                href="/register"
                            >
                                <span class="absolute inset-0 border border-current"></span>
                                <span
                                class="block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur"
                                >
                                register
                                </span>
                            </a>
                        </Show>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
