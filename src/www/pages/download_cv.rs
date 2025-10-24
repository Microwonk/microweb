use leptos::prelude::*;
use leptos::svg::Svg;
use leptos_meta::*;
use leptos_use::use_element_hover;

use crate::www::components::*;

#[component]
pub fn DownloadCVPage() -> impl IntoView {
    let close_icon = NodeRef::<Svg>::new();
    let is_hovered = use_element_hover(close_icon);

    let (language, set_language) = signal("EN");

    let button_text = move || {
        format!(
            "Download {} PDF",
            match language.get() {
                "EN" => "English",
                _ => "German",
            }
        )
    };

    view! {
        <Title text="Nicolas Frey - Resume" />

        <Close el=close_icon />

        <main class={
            let base_class = "grid gap-20 md:gap-28 lg:gap-64 mt-10 xl:mt-28 delay-75 duration-1000 ease-out";
            move || {
                if is_hovered.get() {
                    format!("{} {}", base_class, "usecase-in")
                } else {
                    format!("{} {}", base_class, "usecase-out")
                }
            }
        }>
            <Layout
                id="resume"
                aria_label="resume"
                class_name="flex-col mb-10 xl:mb-28"
            >
                <div class="flex flex-col lg:flex-row gap-16 md:gap-28">
                    <div class="relative order-2 lg:order-1 basis-[60%] fade-y-trans">
                        <div class="absolute hidden md:block left-1/2 -top-14 transform -translate-x-1/2">
                            <section class="example example--2">
                                <label class="inline-flex items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        on:click=move |_| set_language(
                                            if language.get() == "EN" { "DE" } else { "EN" },
                                        )
                                        value=move || language.get()
                                        class="sr-only peer"
                                    />
                                    <div class="relative w-11 h-6 bg-nf-white rounded-full peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-nf-dark after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-nf-color"></div>
                                    <span class="ms-3 text-sm font-medium text-nf-color">
                                        {move || {
                                            if language.get() == "EN" { "English" } else { "German" }
                                        }}
                                    </span>
                                </label>
                            </section>
                        </div>
                        <iframe
                            class="iframe"
                            allowfullscreen
                            src=move || format!("/assets/CV_{}.pdf", language.get())
                        ></iframe>
                    </div>

                    <div class="basis-[40%] order-1 lg:order-2">
                        <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl text-nf-white leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                            <div class="animated-title">
                                <em class="animated-title-element text-nf-white font-bold uppercase">
                                    My
                                </em>
                            </div>
                            <br />
                            <div class="animated-title">
                                <span class="animated-title-element text-nf-white font-bold uppercase">
                                    Resume
                                </span>
                            </div>
                        </h1>
                        <br />
                        <p class="text-xl md:text-2xl lg:text-3xl leading-p lg:leading-largep text-nf-white fade-y-trans font-montserrat">
                            Made with <a href="https://www.latex-project.org/" target="_blank">
                                LaTeX.
                            </a>
                        </p>
                        <br />
                        <br />
                        <button class="button button-cta">
                            <a
                                target="_blank"
                                href=move || format!("/assets/CV_{}.pdf", language.get())
                                download=move || format!("CV_{}.pdf", language.get())
                            >
                                <span class="button-cta-border"></span>
                                <span class="button-cta-ripple">
                                    <span></span>
                                </span>
                                <span class="button-cta-title">
                                    <span data-text=button_text>{button_text}</span>
                                </span>
                            </a>
                        </button>
                    </div>
                </div>
            </Layout>
        </main>
    }
}
