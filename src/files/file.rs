use axum::{
    Extension, Json,
    extract::{Multipart, Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};

use chrono::Utc;
use serde::Deserialize;
use std::{fs::File as FsFile, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::models::{Directory, File, User};

use super::{DIRECTORY, PRIVATE, get_full_path};

#[derive(Deserialize, Debug)]
pub struct DirectoryQuery {
    directory_id: Option<i32>,
}

#[tracing::instrument(skip(user, multipart))]
pub async fn upload(
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
        return (StatusCode::BAD_REQUEST, Json(["Directory does not exist."])).into_response();
    }

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    let dir_path = PathBuf::from(DIRECTORY);

    // Ensure the directory exists
    if let Err(e) = std::fs::create_dir_all(&dir_path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json([format!("Failed to create directory: {e}")]),
        )
            .into_response();
    }

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let id = Uuid::new_v4();

        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or(format!("file_{id}"));
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

        let file_path = dir_path.join(id.to_string());

        let full_path = format!(
            "{}/{}",
            get_full_path(directory.directory_id)
                .await
                .iter()
                .fold("".to_string(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
            file_name,
        );

        if sqlx::query_as::<_, File>("SELECT * FROM files WHERE file_path = $1")
            .bind(&full_path)
            .fetch_optional(crate::database::db())
            .await
            .unwrap_or(None)
            .is_some()
        {
            errors.push(format!("File with name exists: {file_name}"));
            continue;
        }

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
        .bind(&full_path)
        .bind(mime_type)
        .bind(Utc::now())
        .fetch_one(crate::database::db())
        .await
        {
            Ok(res) => uploaded_files.push(res),
            Err(err) => {
                errors.push(err.to_string());
                continue;
            }
        };

        // Store file on disk
        match FsFile::create(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&data) {
                    errors.push(format!("Write error: {e}"));
                }
            }
            Err(err) => {
                errors.push(format!("Create error: {err}"));
            }
        };
    }

    if uploaded_files.is_empty() {
        (StatusCode::BAD_REQUEST, Json(errors)).into_response()
    } else {
        (StatusCode::OK, Json(uploaded_files)).into_response()
    }
}

#[tracing::instrument(skip(user))]
pub async fn delete_by_id(
    Path(id): Path<Uuid>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let success = sqlx::query("DELETE FROM files WHERE id = $1")
        .bind(id)
        .execute(crate::database::db())
        .await
        .map_or(0, |r| r.rows_affected());

    if success == 1 {
        let file_path: PathBuf = [DIRECTORY, &id.to_string()].iter().collect();
        if let Err(e) = tokio::fs::remove_file(file_path).await {
            (StatusCode::INTERNAL_SERVER_ERROR, Json([e.to_string()])).into_response()
        } else {
            StatusCode::OK.into_response()
        }
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

#[tracing::instrument(skip(user))]
pub async fn get_by_id(Path(id): Path<Uuid>, user: Option<Extension<User>>) -> impl IntoResponse {
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

#[tracing::instrument(skip(user))]
pub async fn traverse(
    Path(file_path): Path<String>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    // if it is in any folder containing .private, user must be admin
    if file_path.contains(PRIVATE) && !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

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
