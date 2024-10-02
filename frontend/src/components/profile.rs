use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class="fade-in w-full h-full lg:w-56 lg:h-56 hidden lg:flex justify-center items-center fixed right-10 bottom-10 z-10">
            <div class="w-48 h-48 rounded-full bg-nf-color flex justify-center items-center">
                <picture class="absolute flex items-center">
                <source
                        type="image/webp"
                        srcset="assets/profile.png"
                    />
                    <img
                        width="400"
                        height="400"
                        loading="lazy"
                        class="rounded-full object-cover w-[140px] h-[140px] will-change-auto bg-nf-color"
                        decoding="async"
                        alt="nicolas frey"
                        src="assets/profile.png"
                    />
                </picture>
            </div>
        </div>
    }
}
