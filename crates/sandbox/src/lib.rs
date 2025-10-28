use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use common::models::User;

mod page;

use common::trace::TraceExt;

pub fn router() -> Router {
    Router::new()
        .route("/manage", get(manage_html))
        .route(
            "/create",
            post(page::create).layer(DefaultBodyLimit::max(1e+9 as usize)),
        )
        .route(
            "/page/{id}",
            get(page::get_by_id).delete(page::delete_by_id),
        )
        .route("/pages", get(page::list))
        .route("/{slug}", get(page::view))
        .route("/{slug}/{*file_path}", get(page::view_static))
        .with_tracing()
        .layer(axum::middleware::from_fn(common::auth::auth_guard))
}

#[tracing::instrument(skip(user))]
async fn manage_html(user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    Html(include_str!("manage.html")).into_response()
}
