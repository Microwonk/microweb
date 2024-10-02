use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let logo_src = "path/to/logo.png"; // Replace with your logo path
    let title = "Your Blog Title"; // Replace with your title

    view! {
        <header class="bg-gray-800 text-white">
            <div class="container mx-auto flex justify-between items-center p-4">
                <div class="flex items-center">
                    <img src={logo_src} alt="Logo" class="h-10 mr-3" />
                    <h1 class="text-xl font-bold">{title}</h1>
                </div>
                <nav>

                </nav>
            </div>
        </header>
    }
}
