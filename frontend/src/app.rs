use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::*;
use logging::debug_warn;

use crate::{
    components::ReRouter,
    pages::{
        admin::AdminPage, home::HomePage, login::LoginPage, p404::Page404, profile::ProfilePage,
        register::RegisterPage,
    },
    types::IsAdminResponse,
    util::Api,
};

// TODO: either through backend or on this, it generates a JSON-LD for SEO

#[component]
pub fn App() -> impl IntoView {
    let (is_admin, set_is_admin) = create_signal(false);
    let (logged_in, set_logged_in) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);

    // Initialize state and check if logged in
    spawn_local(async move {
        Api::initialize().await;
        debug_warn!("{}", Api::is_logged_in().await);
        set_logged_in(Api::is_logged_in().await);
        set_is_admin(
            Api::is_admin()
                .await
                .unwrap_or(IsAdminResponse { admin: false })
                .admin,
        );
        set_loaded(true);
    });

    provide_meta_context();
    let UseColorModeReturn { mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Title text="Nicolas Frey Blog"/>
        <Html lang="en" class=move || format!("{} smooth-scroll", mode.get())/>

        <Router>
            <Routes>
                <Route path="/" view=move || view! {
                    <Show  when=move || loaded.get() fallback=|| view! { LOADING }>
                        <Outlet/>
                    </Show>
                }>
                    // Public routes
                    <Route path="/" view=HomePage/>
                    <Route path="/register" view=RegisterPage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/*any" view=Page404/>

                    <Route path="/profile" view=move || {
                        view! {
                            <Show when=move || logged_in.get() fallback=|| view! {
                                <ReRouter route="/login"/>
                            }>
                                <Outlet/>
                            </Show>
                        }
                    }>
                        <Route path="/" view=ProfilePage/>
                    </Route>

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
                        <Route path="/" view=AdminPage/>
                    </Route>
                </Route>
            </Routes>
        </Router>
    }
}
