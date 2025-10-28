use std::{fs::File as FsFile, io::Write, path::PathBuf};

use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

pub mod directory;
pub mod file;

use common::models::{Directory, DirectoryContents, File, User};
use common::{
    api::{ApiError, ApiResult},
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
        .layer(axum::middleware::from_fn(common::auth::auth_guard))
        .layer(CorsLayer::new().allow_origin(Any))
}

#[tracing::instrument(skip(user))]
async fn file_browser_html(user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    Html(include_str!("file_browser.html")).into_response()
}

pub async fn save_to_disk(file_path: PathBuf, data: &[u8]) -> Result<(), String> {
    match FsFile::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(data) {
                Err(format!("Write error: {e}"))
            } else {
                Ok(())
            }
        }
        Err(err) => Err(format!("Create error: {err}")),
    }
}

pub async fn get_full_path(mut directory_id: Option<i32>) -> Vec<Directory> {
    let mut path = Vec::new();

    while let Some(id) = directory_id {
        if let Some(dir) = common::db_query_as!(
            Directory,
            fetch_optional,
            "SELECT * FROM directories WHERE id = $1",
            id
        )
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

pub async fn get_directory_contents(id: Option<i32>) -> ApiResult<DirectoryContents> {
    let files = common::db_query_as!(
        File,
        fetch_all,
        "SELECT * FROM files WHERE directory_id IS NOT DISTINCT FROM $1",
        id
    )
    .map_err(|err| ApiError::Message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let directories = common::db_query_as!(
        Directory,
        fetch_all,
        "SELECT * FROM directories WHERE parent_id IS NOT DISTINCT FROM $1",
        id
    )
    .map_err(|err| ApiError::Message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(DirectoryContents { files, directories })
}
