use std::path::PathBuf;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

use serde::Deserialize;

use crate::models::{Directory, File, User};

use super::{DIRECTORY, ROOT, get_directory_contents, get_full_path};

#[derive(Deserialize, Debug)]
pub struct DirectoryCreateQuery {
    parent_id: Option<i32>,
    name: String,
}

#[tracing::instrument(skip(user))]
pub async fn create(
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
        return (
            StatusCode::BAD_REQUEST,
            Json(["Parent directory does not exist."]),
        )
            .into_response();
    }

    let full_path = format!(
        "{}/{}",
        get_full_path(directory.parent_id)
            .await
            .iter()
            .fold("".to_string(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
        directory.name,
    );

    if sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE dir_path = $1")
        .bind(&full_path)
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None)
        .is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(["Directory with same name exists."]),
        )
            .into_response();
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
    .bind(&full_path)
    .fetch_one(crate::database::db())
    .await
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response(),
    }
}

#[tracing::instrument(skip(user))]
pub async fn delete_by_id(Path(id): Path<i32>, user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let mut errors = Vec::new();

    let files = match sqlx::query_as::<_, File>(
        "SELECT * FROM files WHERE directory_id IS NOT DISTINCT FROM $1",
    )
    .bind(id)
    .fetch_all(crate::database::db())
    .await
    {
        Ok(files) => files,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response();
        }
    };

    let success = sqlx::query("DELETE FROM directories WHERE id = $1")
        .bind(id)
        .execute(crate::database::db())
        .await
        .map_or(0, |r| r.rows_affected());

    if success == 1 {
        for file in files {
            let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
            if let Err(e) = tokio::fs::remove_file(file_path).await {
                errors.push(e.to_string());
            }
        }
    }

    if errors.is_empty() {
        StatusCode::OK.into_response()
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(errors)).into_response();
    }
}

#[derive(Deserialize, Debug)]
pub struct GetContentsQuery {
    contents: Option<bool>,
}

#[tracing::instrument(skip(user))]
pub async fn get_by_id(
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
                    dir_name: ROOT.to_string(),
                    dir_path: ROOT.to_string(),
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
pub async fn traverse(
    Path(dir_path): Path<String>,
    user: Option<Extension<User>>,
    Query(q): Query<GetContentsQuery>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if dir_path == ROOT {
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
                    dir_name: ROOT.to_string(),
                    dir_path: ROOT.to_string(),
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
