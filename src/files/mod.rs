use axum::{Router, response::Html, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(|| async { Html("Hello, Files!") }))
}
