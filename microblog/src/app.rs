use leptos::{context::Provider, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::{
    components::ReRouter,
    models::{IsAdminResponse, Profile, User},
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
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[server(UserAction, "/api", "GetJson")]
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
}

#[component]
pub fn App() -> impl IntoView {
    let (is_admin, set_is_admin) = signal(false);
    let (loaded, set_loaded) = signal(true);
    let (blog_posts, set_blog_posts) = signal(Vec::new());
    let (user, set_user) = signal(None::<Profile>);

    let store = Store::new(GlobalState::default());

    let user: Resource<Result<Option<Profile>, ServerFnError>> = Resource::new(
        move || store.logged_in().get(),
        |_| async { get_user().await },
    );

    provide_context(store);

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/microblog.css" />

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
                                view! {
                                    <Provider value=u>
                                        <Router>
                                            <Routes fallback=Page404>
                                                <ParentRoute
                                                    path=path!("")
                                                    view=move || {
                                                        view! {
                                                            <Show when=move || loaded.get() fallback=LoadingPage>
                                                                <Outlet />
                                                            </Show>
                                                        }
                                                    }
                                                >
                                                    <Route path=path!("") view=HomePage />
                                                    <Route path=path!("register") view=RegisterPage />
                                                    <Route path=path!("login") view=LoginPage />
                                                    <Route path=path!("posts/:slug") view=BlogPostPage />
                                                    // <Route path=path!("feed") view=move || view! { <RSSPage logged_in user/> }/>

                                                    <ParentRoute
                                                        path=path!("admin")
                                                        view=move || {
                                                            view! {
                                                                <Suspense>
                                                                    <Show when=move || !is_admin.get()>
                                                                        <ReRouter route="/" />
                                                                    </Show>
                                                                </Suspense>
                                                                <Outlet />
                                                            }
                                                        }
                                                    >
                                                        <Route
                                                            path=path!("")
                                                            view=move || view! { <AdminPage blog_posts /> }
                                                        />
                                                        <Route
                                                            path=path!("posts/:slug")
                                                            view=move || view! { <EditBlogPostPage blog_posts /> }
                                                        />
                                                    // auth
                                                    </ParentRoute>
                                                </ParentRoute>
                                            </Routes>
                                        </Router>
                                    </Provider>
                                }
                            })
                        })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}
