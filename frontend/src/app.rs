use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::*;

use crate::pages::{home::HomePage, p404::Page404};

// TODO: either through backend or on this, it generates a JSON-LD for SEO

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let UseColorModeReturn { mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Title text="Nicolas Frey Blog"/>
        <Html lang="en" class=move || format!("{} smooth-scroll", mode.get())/>

        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/*any" view=Page404/>
            </Routes>
        </Router>
    }
}
