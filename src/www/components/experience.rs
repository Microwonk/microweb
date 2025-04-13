use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in w-full">
            <div class="lg:col-span-2">
                <div class="text-md lg:text-lg leading-about text-nf-white uppercase">
                    <span class="uppercase">Experience</span>
                </div>

            </div>
            <div class="lg:col-span-5 min-w-full text-xl lg:text-3xl leading-largep text-nf-white font-[400]">

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Software Engineer"
                            class="justify-between flex-row w-full"
                        >
                            ICOTEC GmbH
                            <small class="font-montserrat text-md text-nf-color font-[400]">
                                "2024 - present"
                            </small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Intern Software Developer"
                            class="justify-between flex-row w-full"
                        >
                            Cookis GmbH
                            <small class="font-montserrat text-md text-nf-color font-[400]">
                                "2024"
                            </small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Intern Software Developer"
                            class="justify-between flex-row w-full"
                        >
                            Cookis GmbH
                            <small class="font-montserrat text-md text-nf-color font-[400]">
                                "2023"
                            </small>
                        </span>
                    </span>
                </div>

            </div>
        </div>
    }
}
