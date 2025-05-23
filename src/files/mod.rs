use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};

mod directory;
mod file;

use crate::{
    models::{Directory, User},
    trace::TraceExt,
};

pub static DIRECTORY: &str = "files";
pub static ROOT: &str = "~";
pub static PRIVATE: &str = ".private";

pub fn router() -> Router {
    Router::new()
        .route("/browse", get(file_browser_html))
        .route(
            "/upload/f",
            // 1 Gigabyte file size limit
            post(file::upload).layer(DefaultBodyLimit::max(1e+9 as usize)),
        )
        .route("/upload/d", post(directory::create))
        .route(
            "/f_id/{id}",
            get(file::get_by_id).delete(file::delete_by_id),
        )
        .route(
            "/d_id/{id}",
            get(directory::get_by_id).delete(directory::delete_by_id),
        )
        .route("/d/{*dir_path}", get(directory::traverse))
        .route("/f/{*file_path}", get(file::traverse))
        .with_tracing()
        .layer(axum::middleware::from_fn(crate::auth::auth_guard))
}

#[tracing::instrument(skip(user))]
async fn file_browser_html(user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    Html(include_str!("file_browser.html")).into_response()
}

async fn get_full_path(mut directory_id: Option<i32>) -> Vec<Directory> {
    let mut path = Vec::new();

    while let Some(id) = directory_id {
        if let Some(dir) = sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE id = $1")
            .bind(id)
            .fetch_optional(crate::database::db())
            .await
            .unwrap_or(None)
        {
            directory_id = dir.parent_id;
            path.push(dir);
        } else {
            break;
        }
    }

    path.push(Directory {
        id: 0,
        parent_id: None,
        dir_name: ROOT.to_string(),
        dir_path: ROOT.to_string(),
    });

    path.reverse();
    tracing::info!("{path:?}");
    path
}
