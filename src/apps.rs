use leptos::prelude::{AnyView, IntoAny};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone)]
pub enum Apps {
    Blog,
    Www,
}

impl Apps {
    pub fn starts_with<S: AsRef<str>>(&self, string: S) -> bool {
        let host = string.as_ref();
        match self {
            Apps::Blog => host.starts_with("blog."),
            Apps::Www => host.starts_with("www."),
        }
    }

    pub fn app(&self) -> Box<dyn FnOnce() -> AnyView> {
        match self {
            Apps::Blog => Box::new(|| crate::blog::app::App().into_any()),
            Apps::Www => Box::new(|| crate::www::app::App().into_any()),
        }
    }

    #[cfg(feature = "hydrate")]
    pub fn hydrate() {
        let hostname = leptos::prelude::window()
            .location()
            .hostname()
            .unwrap_or_default();
        for app in Apps::iter() {
            if app.starts_with(&hostname) {
                leptos::mount::hydrate_body(app.app());
            }
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn router(&self) -> axum::Router {
        use leptos::config::get_configuration;
        use tokio::sync::OnceCell;
        match self {
            Apps::Blog => {
                use leptos_axum::{generate_route_list, LeptosRoutes};
                use {crate::blog::app::*, crate::blog::auth};

                static BLOG_ROUTER: OnceCell<axum::Router> = OnceCell::const_new();

                BLOG_ROUTER
                    .get_or_init(|| async {
                        // One-time DB init
                        crate::blog::database::init_db()
                            .await
                            .expect("problem during initialization of the database");

                        let conf = get_configuration(None).unwrap();
                        let leptos_options = conf.leptos_options;
                        let routes = generate_route_list(App);

                        axum::Router::new()
                            .leptos_routes(&leptos_options, routes, {
                                let leptos_options = leptos_options.clone();
                                move || shell(leptos_options.clone())
                            })
                            .fallback(leptos_axum::file_and_error_handler(shell))
                            .layer(axum::middleware::from_fn(auth::auth_guard))
                            .layer(
                                tower_http::trace::TraceLayer::new_for_http()
                                    .make_span_with(
                                        tower_http::trace::DefaultMakeSpan::new()
                                            .level(tracing::Level::INFO),
                                    )
                                    .on_request(
                                        tower_http::trace::DefaultOnRequest::new()
                                            .level(tracing::Level::INFO),
                                    )
                                    .on_response(
                                        tower_http::trace::DefaultOnResponse::new()
                                            .level(tracing::Level::INFO),
                                    )
                                    .on_failure(
                                        tower_http::trace::DefaultOnFailure::new()
                                            .level(tracing::Level::ERROR),
                                    ),
                            )
                            .with_state(leptos_options)
                    })
                    .await
                    .clone()
            }

            Apps::Www => {
                use crate::www::app::*;
                use leptos_axum::{generate_route_list, LeptosRoutes};

                static WWW_ROUTER: OnceCell<axum::Router> = OnceCell::const_new();

                WWW_ROUTER
                    .get_or_init(|| async {
                        let conf = get_configuration(None).unwrap();
                        let leptos_options = conf.leptos_options;
                        let routes = generate_route_list(App);

                        axum::Router::new()
                            .leptos_routes(&leptos_options, routes, {
                                let leptos_options = leptos_options.clone();
                                move || shell(leptos_options.clone())
                            })
                            .fallback(leptos_axum::file_and_error_handler(shell))
                            .with_state(leptos_options)
                    })
                    .await
                    .clone()
            }
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn routers() -> Vec<(Self, axum::Router)> {
        let mut routers = vec![];

        for app in Self::iter() {
            routers.push((app.clone(), app.router().await));
        }

        routers
    }

    #[cfg(feature = "ssr")]
    pub async fn fallback_service(
        req: axum::extract::Request<axum::body::Body>,
    ) -> Result<impl axum::response::IntoResponse, std::convert::Infallible> {
        use axum::response::IntoResponse;
        use tower::ServiceExt;

        let host = req
            .headers()
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        let apps = Apps::routers().await;

        for (app, router) in apps {
            if app.starts_with(host) {
                return router.oneshot(req).await;
            }
        }

        Ok(
            axum::response::Redirect::permanent(&format!("http://www.{}", *crate::DOMAIN))
                .into_response(),
        )
    }
}
