use business_card::BusinessCard;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
};

use crate::www::pages::*;

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
        <Meta attr:lang="en" />

        <defs>
            <svg width="0" height="0">
                <filter id="rough-paper">
                    <feTurbulence
                        type="fractalNoise"
                        baseFrequency="0.04"
                        result="noise"
                        numOctaves="5"
                    />

                    <feDiffuseLighting in="noise" lighting-color="#dad6ca" surfaceScale="2">
                        <feDistantLight azimuth="90" elevation="60" />
                    </feDiffuseLighting>
                </filter>
            </svg>
        </defs>

        <Router>
            <FlatRoutes fallback=NotFound>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/businesscard") view=BusinessCard />
                <Route path=path!("/resume") view=DownloadCVPage />
            </FlatRoutes>
        </Router>
    }
}
