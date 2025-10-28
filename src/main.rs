#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        Router,
        http::{HeaderValue, header::*},
    };
    use common::apps::*;
    use microweb::apps::SsrApps;
    use tower::service_fn;
    use tower_http::cors::CorsLayer;

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    common::db::init_db()
        .await
        .expect("problem during initialization of the database");

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(
                    Apps::iter()
                        .filter_map(|app| HeaderValue::from_str(&app.url()).ok())
                        .collect::<Vec<_>>(),
                )
                .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT, COOKIE]),
        )
        .fallback_service(service_fn(Apps::fallback_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
