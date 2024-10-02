use crate::components::Header;
use leptos::*;
use leptos_use::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let UseColorModeReturn { set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Header/>
        <ul>
            <li><button on:click=move |_| set_mode.set(ColorMode::Light)>Change to Light</button></li>
            <li><button on:click=move |_| set_mode.set(ColorMode::Dark)>Change to Dark</button></li>
            <li><button on:click=move |_| set_mode.set(ColorMode::Auto)>Change to Auto</button></li>
        </ul>
        <img src="https://microblog.shuttleapp.rs/upload/3" alt="My Image"/>
        <video controls>
            <source type="video/mp4" src="https://microblog.shuttleapp.rs/upload/2"/>
        </video>
    }
}
