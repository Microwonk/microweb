use leptos::*;
use leptos_use::{
    use_mouse_with_options, UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions,
    UseMouseReturn, UseMouseSourceType,
};

// Custom extractor struct needed for UseMouseCoordType
#[derive(Clone)]
struct Extractor;
impl UseMouseEventExtractor for Extractor {}

#[component]
pub fn Cursor() -> impl IntoView {
    let UseMouseReturn {
        x, y, source_type, ..
    } = use_mouse_with_options(
        UseMouseOptions::default().coord_type::<Extractor>(UseMouseCoordType::Client),
    );

    let cursor_style = move || format!("top: {}px; left: {}px;", y.get() - 6., x.get() - 6.,);

    view! {
        {move || {
            if matches!(source_type.get(), UseMouseSourceType::Touch) {
                view! {<div/>}
            } else {
                view! {
                    <div style=cursor_style class="mix-blend-difference w-6 h-6 z-50 fixed pointer-events-none">
                        <img
                            src="/assets/cursor.svg"
                            class="pointer-events-none transition-transform"
                        />
                    </div>
                }
            }
        }}
    }
}
