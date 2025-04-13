use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

use crate::www::components::qrcode::QrCode;

#[component]
pub fn BusinessCard() -> impl IntoView {
    let (items, _) = signal(vec![
        ("https://nicolas-frey.com", "My Website"),
        ("https://www.instagram.com/nic_ol_ass", "Instagram"),
        ("https://bsky.app/profile/nicolas-frey.com", "BlueSky"),
        ("https://www.github.com/Microwonk", "GitHub"),
        ("https://microwonk.itch.io/", "Itch.io"),
        ("https://discordapp.com/users/444924590913749002", "Discord"),
    ]);

    let UseWindowSizeReturn { width, height } = use_window_size();
    let (aspect_divisor, _) = signal(5_f64);
    let aspect_divisor: Signal<f64> = aspect_divisor.into();

    view! {
        <Title text="Nicolas Frey - Business Card" />
        <h1 class="text-center w-full text-4xl sm:text-8xl text-nf-white pt-6 sm:pt-12">
            Nicolas Frey | Microwonk
        </h1>
        <ul class="grid grid-cols-1 space-between md:grid-cols-3 gap-6 w-full p-4 md:p-12">
            <For
                each=move || items.get()
                key=|i| i.0
                children=move |item| {
                    view! {
                        <li class="flex flex-col items-center sm:flex-row">
                            <QrCode
                                data=item.0
                                light_c="#dad6ca"
                                dark_c="#0e0306"
                                width
                                height
                                aspect_divisor
                            />
                            <p class="text-nf-white text-xl sm:text-3xl mt-2 sm:mt-0 sm:ml-4 text-center sm:[writing-mode:vertical-lr]">
                                {item.1}
                            </p>
                        </li>
                    }
                }
            />
        </ul>
    }
}
