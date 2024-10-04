use leptos::*;

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-screen bg-gray-100">
            <div class="flex items-center justify-center">
                <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-blue-500 border-solid"></div>
            </div>
        </div>
    }
}
