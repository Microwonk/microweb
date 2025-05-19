use crate::www::components::*;

use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Layout id="contact".to_string() aria_label="Contact" class_name="flex-col".to_string()>
            // Flex container to align the heading and button next to each other
            <div class="flex flex-col lg:flex-row items-start lg:items-center">
                <h2 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl text-nf-white leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading m-10">
                    <div class="animated-title m-5">
                        <span class="animated-title-element text-nf-white font-regular uppercase">
                            Contact
                        </span>
                    </div>
                    <br />
                    <div class="animated-title">
                        <span class="animated-title-element text-nf-white font-bold uppercase">
                            Me
                        </span>
                    </div>
                </h2>

                <Button
                    href="mailto:contact@nicolas-frey.com".to_string()
                    class_name="mx-0 lg:mx-16 mt-4 lg:mt-0".to_string()
                    label="I can't wait!".to_string()
                />
            </div>

            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col mt-5 md:mt-10 ml-10">
                <p class="lg:col-span-2 order-1 self-center min-w-full lg:order-3 text-xl md:text-2xl lg:text-3xl leading-p lg:leading-largep text-nf-white font-montserrat">
                    If you are looking for someone in your team, "I'm"
                    always looking for new opportunities.
                </p>
            </div>
        </Layout>
    }
}
