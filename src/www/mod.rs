pub mod app;
pub mod components;
pub mod pages;
pub mod sections;

#[cfg(feature = "ssr")]
pub mod router {
    use axum::{async_trait, Router};

    #[async_trait]
    pub trait WWWRouter {
        async fn www_router() -> Self;
    }

    #[async_trait]
    impl WWWRouter for Router {
        async fn www_router() -> Self {
            use crate::www::app::*;
            use leptos::prelude::*;
            use leptos_axum::{generate_route_list, LeptosRoutes};

            let conf = get_configuration(None).unwrap();
            let leptos_options = conf.leptos_options;
            let routes = generate_route_list(App);

            Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell))
                .with_state(leptos_options)
        }
    }
}

pub mod utils {
    pub fn map_y_to_value(y: f64, y_visible_coord: f64) -> f64 {
        let start_y = y_visible_coord;
        let end_y = y_visible_coord + 300.0;
        let start_value = 100.0;
        let end_value = 0.0;

        if y < start_y {
            return start_value;
        }
        if y > end_y {
            return end_value;
        }

        let scale = (y - start_y) / (end_y - start_y);
        start_value + scale * (end_value - start_value)
    }
}
