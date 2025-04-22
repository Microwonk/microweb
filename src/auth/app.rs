use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    auth::{login::LoginPage, register::RegisterPage},
    blog::pages::p404::Page404,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html class="blog" lang="en">
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

        <Title text="Authentication" />

        <Router>
            <Routes fallback=Page404>
                <Route path=path!("register") view=RegisterPage />
                <Route path=path!("login") view=LoginPage />
            </Routes>
        </Router>
    }
}
