use leptos::prelude::{AnyView, IntoAny};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone)]
pub enum Apps {
    Blog,
    Www,
    Auth,
}

impl Apps {
    pub fn starts_with<S: AsRef<str>>(&self, string: S) -> bool {
        let host = string.as_ref();
        match self {
            Apps::Blog => host.starts_with("blog."),
            Apps::Www => host.starts_with("www."),
            Apps::Auth => host.starts_with("auth."),
        }
    }

    pub fn url(self) -> String {
        match self {
            Apps::Blog => format!("{}://blog.{}", crate::PROTOCOL, crate::DOMAIN),
            Apps::Www => format!("{}://www.{}", crate::PROTOCOL, crate::DOMAIN),
            Apps::Auth => format!("{}://auth.{}", crate::PROTOCOL, crate::DOMAIN),
        }
    }

    pub fn app(&self) -> Box<dyn FnOnce() -> AnyView> {
        match self {
            Apps::Blog => Box::new(|| crate::blog::app::App().into_any()),
            Apps::Www => Box::new(|| crate::www::app::App().into_any()),
            Apps::Auth => Box::new(|| crate::auth::app::App().into_any()),
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
                use crate::blog::app::*;
                use leptos_axum::{LeptosRoutes, generate_route_list};

                static BLOG_ROUTER: OnceCell<axum::Router> = OnceCell::const_new();

                BLOG_ROUTER
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
            Apps::Www => {
                use crate::www::app::*;
                use leptos_axum::{LeptosRoutes, generate_route_list};

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
            Apps::Auth => {
                use crate::auth::app::*;
                use leptos_axum::{LeptosRoutes, generate_route_list};

                static AUTH_ROUTER: OnceCell<axum::Router> = OnceCell::const_new();

                AUTH_ROUTER
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
            .unwrap_or_default();

        let apps = Apps::routers().await;

        for (app, router) in apps {
            if app.starts_with(host) {
                return router.oneshot(req).await;
            }
        }

        Ok(axum::response::Redirect::permanent(&Apps::Www.url()).into_response())
    }
}
