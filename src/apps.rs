#[cfg(feature = "ssr")]
pub use ssr::*;

#[cfg(feature = "hydrate")]
pub use hydrate::*;

#[cfg(feature = "ssr")]
pub(crate) mod ssr {
    use common::apps::*;

    macro_rules! define_leptos_router {
        (
        $static_name:ident,
        $with_auth:expr
    ) => {{
            use axum::Router;
            use common::trace::TraceExt;
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
                        router = router.layer(axum::middleware::from_fn(common::auth::auth_guard));
                    }

                    router.with_state(leptos_options)
                })
                .await
                .clone()
        }};
    }

    macro_rules! define_router {
        (
    $static_name:ident,
    $router:expr
    ) => {{
            static $static_name: OnceCell<axum::Router> = OnceCell::const_new();

            $static_name.get_or_init(|| async { $router }).await.clone()
        }};
    }

    pub trait SsrApps {
        fn router(&self) -> impl std::future::Future<Output = axum::Router> + Send;
        fn routers() -> impl std::future::Future<Output = Vec<(Self, axum::Router)>> + Send
        where
            Self: Sized;
        fn fallback_service(
            req: axum::extract::Request<axum::body::Body>,
        ) -> impl std::future::Future<
            Output = Result<impl axum::response::IntoResponse, std::convert::Infallible>,
        > + Send;
    }

    impl SsrApps for Apps {
        async fn router(&self) -> axum::Router {
            use tokio::sync::OnceCell;

            match self {
                Apps::Blog => {
                    use blog::app::*;
                    define_leptos_router!(BLOG_ROUTER, true)
                }
                Apps::Www => {
                    use www::app::*;
                    define_leptos_router!(WWW_ROUTER, false)
                }
                Apps::Auth => {
                    use auth::app::*;
                    define_leptos_router!(AUTH_ROUTER, true)
                }
                Apps::Files => define_router!(FILES_ROUTER, files::router()),
                Apps::SandBox => define_router!(SANDBOX_ROUTER, sandbox::router()),
            }
        }

        async fn routers() -> Vec<(Self, axum::Router)> {
            let mut routers = Vec::with_capacity(Self::iter().count());
            for app in Self::iter() {
                routers.push((app.clone(), app.router().await));
            }
            routers
        }

        async fn fallback_service(
            req: axum::extract::Request<axum::body::Body>,
        ) -> Result<impl axum::response::IntoResponse, std::convert::Infallible> {
            use axum::{http::StatusCode, response::IntoResponse};
            use tower::ServiceExt;

            for (app, router) in Apps::routers().await {
                if app.starts_with(
                    req.headers()
                        .get("host")
                        .and_then(|h| h.to_str().ok())
                        .unwrap_or_default(),
                ) {
                    return router.oneshot(req).await;
                }
            }

            Ok(StatusCode::NOT_FOUND.into_response())
        }
    }
}

#[cfg(feature = "hydrate")]
pub(crate) mod hydrate {
    use common::apps::*;
    use leptos::prelude::{AnyView, IntoAny};

    pub trait HydrateApps {
        fn app(&self) -> Box<dyn FnOnce() -> AnyView>;
        fn hydrate();
    }

    impl HydrateApps for Apps {
        fn hydrate() {
            Apps::iter()
                .filter(|a| {
                    a.is_leptos()
                        && a.starts_with(
                            leptos::prelude::window()
                                .location()
                                .hostname()
                                .unwrap_or(Apps::Www.prefix().into()),
                        )
                })
                .for_each(|app| leptos::mount::hydrate_body(app.app()));
        }

        fn app(&self) -> Box<dyn FnOnce() -> AnyView> {
            match self {
                Apps::Blog => Box::new(|| blog::app::App().into_any()),
                Apps::Www => Box::new(|| www::app::App().into_any()),
                Apps::Auth => Box::new(|| auth::app::App().into_any()),
                Apps::Files | Apps::SandBox => unreachable!(),
            }
        }
    }
}
