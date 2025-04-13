pub mod app;
#[cfg(feature = "ssr")]
pub mod auth;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod pages;

pub const THEME_STR: &str = include_str!("peel-light.tmTheme");

#[cfg(feature = "ssr")]
pub mod router {
    use axum::{async_trait, middleware, Router};

    #[async_trait]
    pub trait BlogRouter {
        async fn blog_router() -> Self;
    }

    #[async_trait]
    impl BlogRouter for Router {
        async fn blog_router() -> Self {
            use leptos::prelude::*;
            use leptos_axum::{generate_route_list, LeptosRoutes};
            use {crate::blog::app::*, crate::blog::auth};

            // Init the pool into static
            crate::blog::database::init_db()
                .await
                .expect("problem during initialization of the database");

            let conf = get_configuration(None).unwrap();
            // let addr = conf.leptos_options.site_addr;
            let leptos_options = conf.leptos_options;
            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);

            Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell))
                .layer(middleware::from_fn(auth::auth_guard))
                .layer(
                    tower_http::trace::TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                        )
                        .on_request(
                            tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO),
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                        )
                        .on_failure(
                            tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR),
                        ),
                )
                .with_state(leptos_options)
        }
    }
}
