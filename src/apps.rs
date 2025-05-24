use leptos::prelude::{AnyView, IntoAny};
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "ssr")]
macro_rules! define_leptos_router {
    (
        $static_name:ident,
        $with_auth:expr
    ) => {{
        use crate::trace::TraceExt;
        use axum::Router;
        use leptos::config::get_configuration;
        use leptos_axum::{LeptosRoutes, generate_route_list};

        static $static_name: OnceCell<Router> = OnceCell::const_new();

        $static_name
            .get_or_init(|| async {
                let conf = get_configuration(None).unwrap();
                let leptos_options = conf.leptos_options.clone();
                let routes = generate_route_list(App);

                let mut router = Router::new()
                    .leptos_routes(&leptos_options, routes, {
                        let leptos_options = leptos_options.clone();
                        move || shell(leptos_options.clone())
                    })
                    .with_tracing()
                    .fallback(leptos_axum::file_and_error_handler(shell));

                if $with_auth {
                    router = router.layer(axum::middleware::from_fn(crate::auth::auth_guard));
                }

                router.with_state(leptos_options)
            })
            .await
            .clone()
    }};
}

#[cfg(feature = "ssr")]
macro_rules! define_router {
    (
    $static_name:ident,
    $router:expr
    ) => {{
        static $static_name: OnceCell<axum::Router> = OnceCell::const_new();

        $static_name.get_or_init(|| async { $router }).await.clone()
    }};
}

#[derive(EnumIter, Clone, PartialEq, Eq)]
pub enum Apps {
    Blog,
    Www,
    Auth,
    Files,
    SandBox,
}

impl Apps {
    fn prefix(&self) -> &'static str {
        match self {
            Apps::Blog => "blog.",
            Apps::Www => "www.",
            Apps::Auth => "auth.",
            Apps::Files => "files.",
            Apps::SandBox => "sandbox.",
        }
    }

    pub fn starts_with<S: AsRef<str>>(&self, string: S) -> bool {
        string.as_ref().starts_with(self.prefix())
    }

    pub fn is_leptos(&self) -> bool {
        *self != Apps::Files
    }

    pub fn url(&self) -> String {
        format!("{}://{}{}", crate::PROTOCOL, self.prefix(), crate::DOMAIN)
    }

    pub fn app(&self) -> Box<dyn FnOnce() -> AnyView> {
        match self {
            Apps::Blog => Box::new(|| crate::blog::app::App().into_any()),
            Apps::Www => Box::new(|| crate::www::app::App().into_any()),
            Apps::Auth => Box::new(|| crate::auth::app::App().into_any()),
            Apps::Files | Apps::SandBox => unreachable!(),
        }
    }

    #[cfg(feature = "hydrate")]
    pub fn hydrate() {
        let hostname = leptos::prelude::window()
            .location()
            .hostname()
            .unwrap_or_default();
        for app in Apps::iter().filter(|a| a.is_leptos()) {
            if app.starts_with(&hostname) {
                leptos::mount::hydrate_body(app.app());
            }
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn router(&self) -> axum::Router {
        use tokio::sync::OnceCell;

        match self {
            Apps::Blog => {
                use crate::blog::app::*;
                define_leptos_router!(BLOG_ROUTER, true)
            }
            Apps::Www => {
                use crate::www::app::*;
                define_leptos_router!(WWW_ROUTER, false)
            }
            Apps::Auth => {
                use crate::auth::app::*;
                define_leptos_router!(AUTH_ROUTER, true)
            }
            Apps::Files => define_router!(FILES_ROUTER, crate::files::router()),
            Apps::SandBox => define_router!(SANDBOX_ROUTER, crate::sandbox::router()),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn routers() -> Vec<(Self, axum::Router)> {
        let mut routers = Vec::with_capacity(Self::iter().count());
        for app in Self::iter() {
            routers.push((app.clone(), app.router().await));
        }
        routers
    }

    #[cfg(feature = "ssr")]
    pub async fn fallback_service(
        req: axum::extract::Request<axum::body::Body>,
    ) -> Result<impl axum::response::IntoResponse, std::convert::Infallible> {
        use axum::{http::StatusCode, response::IntoResponse};
        use tower::ServiceExt;

        let host = req
            .headers()
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        let apps = Apps::routers().await;

        for (app, router) in apps {
            if app.starts_with(host) {
                return router.oneshot(req).await;
            }
        }

        Ok(StatusCode::NOT_FOUND.into_response())
    }
}
