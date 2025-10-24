use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    let jobs = [
        (
            "https://www.proxmox.com",
            "Rust Software Developer",
            "ProxMox Software Solutions",
            "2025 - present",
        ),
        (
            "https://at.linkedin.com/in/ronald-macek-1b8a398b",
            "Software Developer",
            "ICOTEC GmbH",
            "2024 - 2025",
        ),
        (
            "https://www.cookis.at",
            "Intern Software Developer",
            "Cookis GmbH",
            "2023 - 2024",
        ),
    ];

    view! {
        <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in w-full">
            <div class="lg:col-span-2">
                <div class="text-md lg:text-lg leading-about text-nf-white uppercase">
                    <span class="uppercase">"Experience —"</span>
                </div>

            </div>
            <div class="lg:col-span-5 min-w-full text-xl lg:text-3xl leading-largep text-nf-white font-[400]">
                {move || {
                    jobs.iter()
                        .map(|j| {
                            view! {
                                <a href=j.0 target="_blank">
                                    <div class="experience experience-cta">
                                        <span class="experience-cta-border"></span>
                                        <span class="experience-cta-ripple">
                                            <span></span>
                                        </span>
                                        <span class="experience-cta-title">
                                            <span data-text=j.1 class="justify-between flex-row w-full">
                                                {j.2}
                                                <small class="font-montserrat text-md text-nf-color font-[400]">
                                                    {j.3}
                                                </small>
                                            </span>
                                        </span>
                                    </div>
                                </a>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}
