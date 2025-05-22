use axum::{
    Extension, Json, Router,
    extract::{Multipart, Path, Query},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use chrono::Utc;
use serde::Deserialize;
use std::{fs::File as FsFile, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::{
    models::{Directory, DirectoryContents, File, User},
    trace::TraceExt,
};

static DIRECTORY: &str = "files";

pub fn router() -> Router {
    let protected = Router::new()
        .route("/upload/f", post(upload_file))
        .route("/upload/d", post(create_directory))
        .route("/f_id/{id}", get(get_file_by_id))
        .route("/d_id/{id}", get(get_dir_by_id))
        .route("/browse", get(file_browser_html))
        .layer(axum::middleware::from_fn(crate::auth::auth_guard));

    Router::new()
        .merge(protected)
        .route("/f/{*file_path}", get(traverse_files))
        .route("/d/{*dir_path}", get(traverse_directories))
        .with_tracing()
}

async fn file_browser_html(user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    Html(include_str!("file_browser.html")).into_response()
}

#[derive(Deserialize, Debug)]
struct DirectoryCreateQuery {
    parent_id: Option<i32>,
    name: String,
}

#[tracing::instrument]
pub async fn create_directory(
    Query(directory): Query<DirectoryCreateQuery>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if let Some(id) = directory.parent_id
        && sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE id = $1")
            .bind(id)
            .fetch_optional(crate::database::db())
            .await
            .unwrap_or(None)
            .is_none()
    {
        return (StatusCode::BAD_REQUEST, "Directory does not exist.").into_response();
    }

    match sqlx::query_as::<_, Directory>(
        r#"
        INSERT INTO directories (parent_id, dir_name, dir_path)
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
    )
    .bind(directory.parent_id)
    .bind(&directory.name)
    .bind(format!(
        "{}/{}",
        get_full_path(directory.parent_id)
            .await
            .iter()
            .fold("".to_string(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
        directory.name,
    ))
    .fetch_one(crate::database::db())
    .await
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[derive(Deserialize, Debug)]
struct DirectoryQuery {
    directory_id: Option<i32>,
}

#[tracing::instrument]
pub async fn upload_file(
    Query(directory): Query<DirectoryQuery>,
    user: Option<Extension<User>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if let Some(id) = directory.directory_id
        && sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE id = $1")
            .bind(id)
            .fetch_optional(crate::database::db())
            .await
            .unwrap_or(None)
            .is_none()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(vec!["Directory does not exist."]),
        )
            .into_response();
    }

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    let dir_path = PathBuf::from(DIRECTORY);

    // Ensure the directory exists
    if let Err(e) = std::fs::create_dir_all(&dir_path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(vec![format!("Failed to create directory: {}", e)]),
        )
            .into_response();
    }

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or("file".into());
        let mime_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or("application/octet-stream".into());
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(err) => {
                errors.push(err.to_string());
                continue;
            }
        };

        let id = Uuid::new_v4();
        let file_path = dir_path.join(id.to_string());

        // Store file on disk
        match FsFile::create(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&data) {
                    errors.push(format!("Write error: {e}"));
                    continue;
                }
            }
            Err(err) => {
                errors.push(format!("Create error: {err}"));
                continue;
            }
        };

        let dirs = get_full_path(directory.directory_id).await;

        match sqlx::query_as::<_, File>(
            r#"
            INSERT INTO files (id, directory_id, file_name, file_path, mime_type, uploaded_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(directory.directory_id)
        .bind(&file_name)
        .bind(format!(
            "{}/{}",
            dirs.iter()
                .fold("".to_string(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
            file_name,
        ))
        .bind(mime_type)
        .bind(Utc::now())
        .fetch_one(crate::database::db())
        .await
        {
            Ok(res) => uploaded_files.push(res),
            Err(err) => errors.push(err.to_string()),
        };
    }

    if uploaded_files.is_empty() {
        (StatusCode::BAD_REQUEST, Json(errors)).into_response()
    } else {
        (StatusCode::OK, Json(uploaded_files)).into_response()
    }
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
        dir_name: "~".to_string(),
        dir_path: "~".to_string(),
    });

    path.reverse();
    tracing::info!("{path:?}");
    path
}

#[tracing::instrument]
pub async fn get_file_by_id(
    Path(id): Path<Uuid>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE id = $1")
        .bind(id)
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None);

    if let Some(file) = file {
        let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
        if let Ok(bytes) = tokio::fs::read(file_path).await {
            return ([(header::CONTENT_TYPE, file.mime_type)], bytes).into_response();
        }
    }

    StatusCode::NOT_FOUND.into_response()
}

#[tracing::instrument]
pub async fn traverse_files(Path(file_path): Path<String>) -> impl IntoResponse {
    let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE file_path = $1")
        .bind(format!("/{file_path}"))
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None);

    if let Some(file) = file {
        let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
        if let Ok(bytes) = tokio::fs::read(file_path).await {
            return ([(header::CONTENT_TYPE, file.mime_type)], bytes).into_response();
        }
    }

    StatusCode::NOT_FOUND.into_response()
}

#[derive(Deserialize, Debug)]
struct GetContentsQuery {
    contents: Option<bool>,
}

#[tracing::instrument]
pub async fn get_dir_by_id(
    Path(id): Path<i32>,
    user: Option<Extension<User>>,
    Query(q): Query<GetContentsQuery>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let contents = q.contents.is_some_and(|c| c);

    if id == 0 {
        if contents {
            return match get_directory_contents(None).await {
                Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
                Err(err) => err.into_response(),
            };
        } else {
            return (
                StatusCode::OK,
                Json(Directory {
                    id: 0,
                    parent_id: None,
                    dir_name: "~".to_string(),
                    dir_path: "~".to_string(),
                }),
            )
                .into_response();
        }
    }

    let dir = sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE id = $1")
        .bind(id)
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None);

    if let Some(dir) = dir {
        if contents {
            return match get_directory_contents(Some(dir.id)).await {
                Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
                Err(err) => err.into_response(),
            };
        } else {
            return (StatusCode::OK, Json(dir)).into_response();
        }
    }

    StatusCode::NOT_FOUND.into_response()
}

#[tracing::instrument]
pub async fn traverse_directories(
    Path(dir_path): Path<String>,
    Query(q): Query<GetContentsQuery>,
) -> impl IntoResponse {
    if dir_path == "~" {
        if q.contents.is_some_and(|c| c) {
            return match get_directory_contents(None).await {
                Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
                Err(err) => err.into_response(),
            };
        } else {
            return (
                StatusCode::OK,
                Json(Directory {
                    id: 0,
                    parent_id: None,
                    dir_name: "~".to_string(),
                    dir_path: "~".to_string(),
                }),
            )
                .into_response();
        }
    }

    let dir = sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE dir_path = $1")
        .bind(format!("/{dir_path}"))
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None);

    if let Some(dir) = dir {
        if q.contents.is_some_and(|c| c) {
            return match get_directory_contents(Some(dir.id)).await {
                Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
                Err(err) => err.into_response(),
            };
        } else {
            return (StatusCode::OK, Json(dir)).into_response();
        }
    }

    StatusCode::NOT_FOUND.into_response()
}

async fn get_directory_contents(
    id: Option<i32>,
) -> Result<DirectoryContents, (StatusCode, Json<Vec<String>>)> {
    let files =
        sqlx::query_as::<_, File>("SELECT * FROM files WHERE directory_id IS NOT DISTINCT FROM $1")
            .bind(id)
            .fetch_all(crate::database::db())
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(vec![err.to_string()]),
                )
            })?;

    let directories = sqlx::query_as::<_, Directory>(
        "SELECT * FROM directories WHERE parent_id IS NOT DISTINCT FROM $1",
    )
    .bind(id)
    .fetch_all(crate::database::db())
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(vec![err.to_string()]),
        )
    })?;

    Ok(DirectoryContents { files, directories })
}
