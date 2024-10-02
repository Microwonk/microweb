use leptos::svg::Svg;
use leptos::*;

#[component]
pub fn Close(el: NodeRef<Svg>) -> impl IntoView {
    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16">
            <nav class="gap-2 md:flex-row flex-col flex items-center justify-center" aria-label="X">
                <a
                    href="/"
                    class="rounded-full close ring-nf-white ring-2 w-[51px] h-[51px] md:w-[61px] md:h-[61px]"
                >
                    <svg
                        node_ref=el
                        viewBox="0 0 61 61"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <rect width="61" height="61" rx="30.5" class="fill-nf-white"></rect>
                        <g clip-path="url(#clip0_615_114)" class="fill-nf-dark">
                            <path d="M38.7751 24.0174L36.9825 22.2248L29.8756 29.3317L22.7687 22.2248L20.9761 24.0174L28.083 31.1243L20.9761 38.2312L22.7687 40.0238L29.8756 32.9169L36.9825 40.0238L38.7751 38.2312L31.6682 31.1243L38.7751 24.0174Z"></path>
                        </g>
                        <defs>
                            <clipPath id="clip0_615_114">
                                <rect
                                    width="51.7791"
                                    height="51.7791"
                                    class="fill-nf-dark"
                                    transform="translate(4.25586 4.96509)"
                                ></rect>
                            </clipPath>
                        </defs>
                    </svg>
                </a>
            </nav>
        </header>
    }
}
