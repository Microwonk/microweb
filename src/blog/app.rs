use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::blog::{
    models::*,
    pages::{
        admin::AdminPage, blog_post::BlogPostPage, edit_blog_post::EditBlogPostPage,
        home::HomePage, loading::LoadingPage, login::LoginPage, p404::Page404,
        register::RegisterPage, rss::RSSPage,
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[server(UserAction, "/api", "GetJson", endpoint = "profile")]
#[tracing::instrument]
pub async fn get_user() -> Result<Option<Profile>, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    let Some(user) = extract::<Extension<User>>().await.ok() else {
        return Ok(None);
    };

    Ok(Some(user.profile()))
}

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    logged_in: bool,
    user: Option<Profile>,
}

#[component]
pub fn App() -> impl IntoView {
    let store = Store::new(GlobalState::default());

    let user: Resource<Result<Option<Profile>, ServerFnError>> = Resource::new(
        move || store.logged_in().get(),
        |_| async { get_user().await },
    );

    provide_context(store);

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/microblog.css" />

        <Title text="Microwonks Blog" />

        <Suspense fallback=LoadingPage>
            <ErrorBoundary fallback=|_| {
                view! {
                    <p class="error-messages text-xs-center">
                        "Something went wrong, please try again later."
                    </p>
                }
            }>
                {move || {
                    user.get()
                        .map(|us| {
                            us.map(|u| {
                                store.user().set(u);
                                view! {
                                    <Router>
                                        <Routes fallback=Page404>
                                            <Route path=path!("") view=HomePage />
                                            <Route path=path!("register") view=RegisterPage />
                                            <Route path=path!("login") view=LoginPage />
                                            <Route path=path!("posts/:slug") view=BlogPostPage />
                                            <Route path=path!("feed") view=RSSPage />

                                            <ParentRoute
                                                path=path!("admin")
                                                view=move || {
                                                    view! {
                                                        <Suspense>
                                                            <Show when=move || {
                                                                !store.user().get().is_some_and(|u| u.is_admin)
                                                            }>
                                                                <Redirect path="/" />
                                                            </Show>
                                                        </Suspense>
                                                        <Outlet />
                                                    }
                                                }
                                            >
                                                <Route path=path!("") view=AdminPage />
                                                <Route path=path!("posts/:slug") view=EditBlogPostPage />
                                            // auth
                                            </ParentRoute>
                                        </Routes>
                                    </Router>
                                }
                            })
                        })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}
