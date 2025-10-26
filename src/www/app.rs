use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
};

use crate::{
    apps::components::CookiePopup,
    www::{components::BackGround, pages::*},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/microweb.css" />

        <Title text="Nicolas Frey" />

        <CookiePopup />

        <BackGround />

        <Router>
            <FlatRoutes fallback=NotFound>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/resume") view=DownloadCVPage />
                <Route path=path!("/privacy-policy") view=PrivacyPolicy />
            </FlatRoutes>
        </Router>

        <script src="https://cdn.jsdelivr.net/gh/happy358/TornPaper@v0.0.3/tornpaper.min.js"></script>
        <script>
            document.addEventListener("DOMContentLoaded", function () {
                new Tornpaper();
            });
        </script>
    }
}
