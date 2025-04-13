#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body,
        http::Request,
        response::{IntoResponse, Redirect},
        Router,
    };
    use microblog::{blog::router::BlogRouter, www::router::WWWRouter};
    use tower::{service_fn, ServiceExt};

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let blog_app = Router::blog_router().await;
    let www_app = Router::www_router().await;

    let domain = std::env::var("DOMAIN").expect("Env var DOMAIN must be set.");
    let port = std::env::var("PORT").expect("Env var PORT must be set.");
    let addr = format!("{}:{}", domain.clone(), port);

    let app = Router::new().fallback_service(service_fn(move |req: Request<Body>| {
        let blog_app = blog_app.clone();
        let www_app = www_app.clone();
        let domain = domain.clone();
        async move {
            let host = req
                .headers()
                .get("host")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");

            if host.starts_with("blog.") {
                blog_app.clone().oneshot(req).await
            } else if host.starts_with("www.") {
                www_app.clone().oneshot(req).await
            } else {
                // TODO
                Ok(Redirect::permanent(&format!("http://www.{}", domain.clone())).into_response())
            }
        }
    }));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

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
