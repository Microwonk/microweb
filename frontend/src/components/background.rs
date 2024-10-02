use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn BackGround() -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    let mask_style = move || {
        let x = x.get();
        let y = y.get();
        format!(
            "mask-image: radial-gradient(ellipse at {}px {}px, transparent 20%, #0e0306);",
            x, y
        )
    };

    view! {
        <div class="fixed backdrop-blur z-0 top-0 h-full w-full bg-nf-dark bg-dot-nf-color/[0.7] flex items-center justify-center">
            <div class="fixed pointer-events-all inset-0 flex items-center justify-center bg-nf-dark" style=mask_style></div>
        </div>
    }
}
