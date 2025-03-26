use iter_tools::Itertools;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

use crate::{
    components::ReRouter,
    models::{IsAdminResponse, Profile},
    pages::{
        admin::AdminPage, blog_post::BlogPostPage, edit_blog_post::EditBlogPostPage,
        home::HomePage, loading::LoadingPage, login::LoginPage, logout::LogOut, p404::Page404,
        register::RegisterPage, rss::RSSPage,
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (is_admin, set_is_admin) = signal(false);
    let (logged_in, set_logged_in) = signal(false);
    let (loaded, set_loaded) = signal(true);
    let (blog_posts, set_blog_posts) = signal(Vec::new());
    let (user, set_user) = signal(None::<Profile>);

    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/microblog.css"/>

        <Router>
            <Routes fallback=Page404>
                <ParentRoute path=path!("") view=move || view! {
                    <Show when=move || loaded.get() fallback=LoadingPage>
                        <Outlet/>
                    </Show>
                }>
                    <Route path=path!("") view=move || view! { <HomePage user logged_in/> }/>
                    <Route path=path!("register") view=move || view! { <RegisterPage set_user set_logged_in/> }/>
                    <Route path=path!("login") view=move || view! { <LoginPage set_user set_logged_in/> }/>
                    <Route path=path!("logout") view=move || view! { <LogOut set_user set_logged_in logged_in/> }/>
                    <Route path=path!("posts/:slug") view=move || view! { <BlogPostPage user logged_in is_admin blog_posts/> }/>
                    // <Route path=path!("feed") view=move || view! { <RSSPage logged_in user/> }/>

                    <ParentRoute path=path!("admin") view=move || {
                        view! {
                            <Suspense>
                                <Show when=move || !is_admin.get()>
                                    <ReRouter route="/"/>
                                </Show>
                            </Suspense>
                            <Outlet/>
                        }
                    }>
                        <Route path=path!("") view=move || view! { <AdminPage blog_posts/> } />
                        <Route path=path!("posts/:slug") view=move || view! { <EditBlogPostPage blog_posts/> } />
                    </ParentRoute> // auth
                </ParentRoute>
            </Routes>
        </Router>
    }
}
