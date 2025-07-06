use leptos::prelude::*;
use leptos_meta::*;

use crate::{apps::components::Footer, www::components::*};

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <Title text="Nicolas Frey - Privacy Policy" />
        <Header />

        <main class="flex flex-col gap-y-10 md:gap-y-18 mt-12 mb-4 relative w-full isolate text-center">
            <Content />
            <div class="mx-6 mt-12">
                <Footer />
            </div>
        </main>
    }
}

#[component(transparent)]
fn Content() -> impl IntoView {
    view! {
        <section class="text-nf-white mx-4">
            <p class="mb-1 text-3xl font-bold">
                This is a personal website. <br />
                I respect your privacy and collect only the data needed for functionality.
            </p>
            <p class="mb-1 text-xs">
                {move || {
                    format!(
                        "This privacy policy covers all websites/subdomains under the domain {}",
                        crate::DOMAIN,
                    )
                }}
            </p>
        </section>

        <hr />

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">1. What I Collect</h2>
            <p>
                When you create an account (e.g., to comment), I collect and store your email address and chosen username. If you choose to create an account, you agree to this privacy policy.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">2. Cookies</h2>
            <p>
                Cookies are used only for authentication and session management. No third-party or tracking cookies are used.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">3. How Your Data Is Used</h2>
            <p>
                Your data is used solely to enable account features like commenting. It is not shared with third parties.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">4. Data Deletion</h2>
            <p>
                You may request deletion of your account and associated data at any time by contacting me at
                <a href="mailto:contact@nicolas-frey.com" class="hover:underline">
                    contact@nicolas-frey.com
                </a>.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">5. Security</h2>
            <p>
                Reasonable technical measures are in place to protect your data from unauthorized access.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold">6. Legal Basis and Your Rights</h2>
            <p class="mb-2">
                "Data is processed based on your consent and for fulfilling your request to use account features (Art. 6(1)(a) & (b) GDPR)."
            </p>
            <p class="mb-2">
                You have the right to access, correct, or delete your data, object to processing, and lodge a complaint with the Austrian Data Protection Authority:
                <a href="https://www.dsb.gv.at/" target="_blank" class="hover:underline">
                    "https://www.dsb.gv.at/"
                </a>.
            </p>
        </section>

        <section class="text-nf-white mx-4">
            <h2 class="text-2xl font-semibold mb-2">7. Data Controller</h2>
            <p class="mb-1">Nicolas Frey</p>
            <p class="mb-1">Vienna, Austria</p>
            <p>
                Contact: <a href="mailto:contact@nicolas-frey.com" class="hover:underline">
                    contact@nicolas-frey.com
                </a>
            </p>
        </section>
    }
}
