use axum::{
    Extension, Json,
    extract::{Multipart, Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};

use chrono::Utc;
use serde::Deserialize;
use std::path::PathBuf;
use uuid::Uuid;

use common::models::{Directory, File, User};

use common::api::{ApiError, ApiResult};

use super::{DIRECTORY, PRIVATE, get_full_path, save_to_disk};

#[derive(Deserialize, Debug)]
pub struct DirectoryQuery {
    directory_id: Option<i32>,
}

#[tracing::instrument(skip(user, multipart))]
pub async fn upload(
    Query(directory): Query<DirectoryQuery>,
    user: Option<Extension<User>>,
    mut multipart: Multipart,
) -> ApiResult<Json<Vec<File>>> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::StatusCode(StatusCode::UNAUTHORIZED));
    };

    if let Some(id) = directory.directory_id
        && common::db_query_as!(
            Directory,
            fetch_optional,
            "SELECT * FROM directories WHERE id = $1",
            id
        )
        .unwrap_or(None)
        .is_none()
    {
        return Err(ApiError::Message(
            StatusCode::BAD_REQUEST,
            "Directory does not exist.".into(),
        ));
    }

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    let dir_path = PathBuf::from(DIRECTORY);

    // Ensure the directory exists
    tokio::fs::create_dir_all(&dir_path).await?;

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
                .fold(String::new(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
            file_name,
        );

        match common::db_query_as!(
            File,
            fetch_one,
            r#"
            INSERT INTO files (id, directory_id, file_name, file_path, mime_type, uploaded_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            id,
            directory.directory_id,
            &file_name,
            &full_path,
            mime_type,
            Utc::now()
        ) {
            Ok(res) => uploaded_files.push(res),
            Err(err) => {
                errors.push(err.to_string());
                continue;
            }
        };

        if let Err(e) = save_to_disk(file_path, &data).await {
            errors.push(e);
        }
    }

    if uploaded_files.is_empty() {
        Err(ApiError::MultipleMessages(StatusCode::BAD_REQUEST, errors))
    } else {
        Ok(Json(uploaded_files))
    }
}

#[tracing::instrument(skip(user))]
pub async fn delete_by_id(Path(id): Path<Uuid>, user: Option<Extension<User>>) -> ApiResult<()> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    if common::db_query!(execute, "DELETE FROM files WHERE id = $1", id)
        .map_or(0, |r| r.rows_affected())
        == 1
    {
        tokio::fs::remove_file([DIRECTORY, &id.to_string()].iter().collect::<PathBuf>())
            .await
            .map_err(Into::into)
    } else {
        Err(ApiError::bad_request())
    }
}

#[tracing::instrument(skip(user))]
pub async fn get_by_id(
    Path(id): Path<Uuid>,
    user: Option<Extension<User>>,
) -> ApiResult<impl IntoResponse> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let file = common::db_query_as!(File, fetch_one, "SELECT * FROM files WHERE id = $1", id)
        .map_err(|_| ApiError::not_found())?;

    let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
    tokio::fs::read(file_path)
        .await
        .map(|bytes| ([(header::CONTENT_TYPE, file.mime_type)], bytes))
        .map_err(|_| ApiError::not_found())
}

#[tracing::instrument(skip(user))]
pub async fn traverse(
    Path(file_path): Path<String>,
    user: Option<Extension<User>>,
) -> ApiResult<impl IntoResponse> {
    // if it is in any folder containing .private, user must be admin
    if file_path.contains(PRIVATE) && !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let file = common::db_query_as!(
        File,
        fetch_one,
        "SELECT * FROM files WHERE file_path = $1",
        format!("/{file_path}")
    )
    .map_err(|_| ApiError::not_found())?;

    let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
    tokio::fs::read(file_path)
        .await
        .map(|bytes| ([(header::CONTENT_TYPE, file.mime_type)], bytes))
        .map_err(|_| ApiError::not_found())
}
