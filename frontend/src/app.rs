use iter_tools::Itertools;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::ReRouter,
    pages::{
        admin::AdminPage, blog_post::BlogPostPage, edit_blog_post::EditBlogPostPage,
        home::HomePage, loading::LoadingPage, login::LoginPage, logout::LogOut, p404::Page404,
        register::RegisterPage,
    },
    types::{IsAdminResponse, Profile},
    util::Api,
};

// TODO: either through backend or on this, it generates a JSON-LD for SEO

#[component]
pub fn App() -> impl IntoView {
    let (is_admin, set_is_admin) = create_signal(false);
    let (logged_in, set_logged_in) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);
    let (blog_posts, set_blog_posts) = create_signal(Vec::new());
    let (user, set_user) = create_signal(None::<Profile>);

    // Initialize state and check if logged in
    spawn_local(async move {
        Api::initialize().await;
        set_logged_in(Api::is_logged_in().await);
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
        set_user(Api::get_profile().await.ok());

        set_loaded(true);
    });

    provide_meta_context();

    view! {
        <Html lang="en" class="smooth-scroll"/>

        <Router>
            <Routes>
                <Route path="/" view=move || view! {
                    <Show when=move || loaded.get() fallback=LoadingPage>
                        <Outlet/>
                    </Show>
                }>
                    // Public routes
                    <Route path="/" view=move || view! { <HomePage user logged_in blog_posts/> }/>
                    <Route path="/register" view=move || view! { <RegisterPage set_user set_logged_in/> }/>
                    <Route path="/login" view=move || view! { <LoginPage set_user set_logged_in/> }/>
                    <Route path="/logout" view=move || view! { <LogOut set_user set_logged_in/> }/>
                    <Route path="/posts/:slug" view=move || view! { <BlogPostPage user logged_in is_admin blog_posts/> }/>
                    <Route path="/*any" view=Page404/>

                    // Admin routes
                    <Route path="/admin" view=move || {
                        view! {
                            <Show when=move || is_admin.get() fallback=|| view! {
                                <ReRouter route="/"/>
                            }>
                                <Outlet/>
                            </Show>
                        }
                    }>
                        <Route path="/" view=move || view! { <AdminPage blog_posts/>} />
                        <Route path="?tab" view=move || view! { <AdminPage blog_posts/>} />
                        <Route path="/posts/:slug" view=move || view! { <EditBlogPostPage blog_posts/> }/>
                        </Route>
                </Route>
            </Routes>
        </Router>
    }
}
