use iter_tools::Itertools;
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    *,
};

use crate::{
    components::ReRouter,
    pages::{
        admin::AdminPage, blog_post::BlogPostPage, edit_blog_post::EditBlogPostPage,
        home::HomePage, loading::LoadingPage, login::LoginPage, logout::LogOut, p404::Page404,
        register::RegisterPage, rss::RSSPage,
    },
    types::{IsAdminResponse, Profile},
    util::Api,
};

// TODO: either through backend or on this, it generates a JSON-LD for SEO

#[component]
pub fn App() -> impl IntoView {
    let (is_admin, set_is_admin) = signal(false);
    let (logged_in, set_logged_in) = signal(false);
    let (loaded, set_loaded) = signal(false);
    let (blog_posts, set_blog_posts) = signal(Vec::new());
    let (user, set_user) = signal(None::<Profile>);

    // Initialize state and check if logged in
    spawn_local(async move {
        Api::initialize().await;
        let l_in = Api::is_logged_in().await;
        set_logged_in(l_in);
        let admin = Api::is_admin()
            .await
            .unwrap_or(IsAdminResponse { admin: false })
            .admin;
        set_is_admin(admin);
        let mut all = Api::all_blog_posts().await.unwrap_or(Vec::new());
        set_blog_posts(if admin {
            let mut comb = Api::admin_blog_posts().await.unwrap_or(Vec::new());
            comb.append(&mut all);
            comb.iter().unique_by(|p| p.id).cloned().collect()
        } else {
            all
        });
        if l_in {
            set_user(Api::get_profile().await.ok());
        }

        set_loaded(true);
    });

    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:class="smooth-scroll"/>

        <Router>
            <Routes fallback=Page404>
                <ParentRoute path=path!("") view=move || view! {
                    <Show when=move || loaded.get() fallback=LoadingPage>
                        <Outlet/>
                    </Show>
                }>
                    <Route path=path!("/") view=move || view! { <HomePage user logged_in blog_posts/> }/>
                    <Route path=path!("register") view=move || view! { <RegisterPage set_user set_logged_in/> }/>
                    <Route path=path!("login") view=move || view! { <LoginPage set_user set_logged_in/> }/>
                    <Route path=path!("logout") view=move || view! { <LogOut set_user set_logged_in logged_in/> }/>
                    <Route path=path!("posts/:slug") view=move || view! { <BlogPostPage user logged_in is_admin blog_posts/> }/>
                    <Route path=path!("feed") view=move || view! { <RSSPage logged_in user/> }/>

                    <ParentRoute path=path!("admin") view=move || {
                        view! {
                            <Suspense>
                                <Show when=move || !is_admin.get()>
                                    <ReRouter route="/admin"/>
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
