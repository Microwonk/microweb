use std::path::PathBuf;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    files::{DIRECTORY, get_directory_contents, get_full_path},
    models::{Directory, File, SandboxPage, User},
};

pub static SANDBOX_DIR: &str = "/~/sandbox";

#[derive(Deserialize, Debug)]
pub struct UploadPageQuery {
    slug: String,
}

#[tracing::instrument(skip(user))]
pub async fn create(
    Query(q): Query<UploadPageQuery>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let Some(sbx_dir) =
        sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE dir_path = $1")
            .bind(SANDBOX_DIR)
            .fetch_optional(crate::database::db())
            .await
            .unwrap_or(None)
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(["Sandbox directory does not exist."]),
        )
            .into_response();
    };

    let full_path = format!(
        "{}/{}",
        get_full_path(Some(sbx_dir.id))
            .await
            .iter()
            .fold("".to_string(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
        &q.slug,
    );

    if sqlx::query_as::<_, SandboxPage>("SELECT * FROM sandbox WHERE slug = $1")
        .bind(&q.slug)
        .fetch_optional(crate::database::db())
        .await
        .unwrap_or(None)
        .is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(["Page with same slug exists."]),
        )
            .into_response();
    }

    let dir = match sqlx::query_as::<_, Directory>(
        r#"
        INSERT INTO directories (parent_id, dir_name, dir_path)
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
    )
    .bind(sbx_dir.id)
    .bind(&q.slug)
    .bind(&full_path)
    .fetch_one(crate::database::db())
    .await
    {
        Ok(dir) => dir,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response();
        }
    };

    match sqlx::query_as::<_, SandboxPage>(
        r#"
        INSERT INTO sandbox (directory_id, slug)
        VALUES ($1, $2)
        RETURNING *
    "#,
    )
    .bind(dir.id)
    .bind(&q.slug)
    .fetch_one(crate::database::db())
    .await
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response(),
    }
}

#[tracing::instrument(skip(user))]
pub async fn list(user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    match sqlx::query_as::<_, SandboxPage>("SELECT * FROM sandbox")
        .fetch_all(crate::database::db())
        .await
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response(),
    }
}

#[tracing::instrument(skip(user))]
pub async fn get_by_id(Path(id): Path<Uuid>, user: Option<Extension<User>>) -> impl IntoResponse {
    if !user.is_some_and(|u| u.admin) {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    match sqlx::query_as::<_, SandboxPage>("SELECT * FROM sandbox WHERE id = $1")
        .bind(id)
        .fetch_all(crate::database::db())
        .await
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response(),
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

    match sqlx::query("DELETE FROM sandbox WHERE id = $1")
        .bind(id)
        .execute(crate::database::db())
        .await
    {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response(),
    }
}

pub async fn view(Path(slug): Path<String>) -> impl IntoResponse {
    let dir = match sqlx::query_as::<_, Directory>(
        "SELECT d.* FROM directories d JOIN sandbox s ON d.id = s.directory_id WHERE s.slug = $1",
    )
    .bind(slug)
    .fetch_one(crate::database::db())
    .await
    {
        Ok(dir) => dir,
        Err(_) => {
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    let contents = match get_directory_contents(Some(dir.id)).await {
        Ok(contents) => contents,
        Err(err) => return err.into_response(),
    };

    let Some(index) = contents.files.iter().find(|f| f.file_name == "index.html") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(["File index.html does not exist."]),
        )
            .into_response();
    };

    let file_path: PathBuf = [DIRECTORY, &index.id.to_string()].iter().collect();
    if let Ok(bytes) = tokio::fs::read(file_path).await {
        return ([(header::CONTENT_TYPE, &index.mime_type)], bytes).into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}

pub async fn view_static(Path((slug, file_path)): Path<(String, String)>) -> impl IntoResponse {
    let dir = match sqlx::query_as::<_, Directory>(
        "SELECT d.* FROM directories d JOIN sandbox s ON d.id = s.directory_id WHERE s.slug = $1",
    )
    .bind(&slug)
    .fetch_one(crate::database::db())
    .await
    {
        Ok(dir) => dir,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let contents = match get_directory_contents(Some(dir.id)).await {
        Ok(c) => c,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let file = match contents
        .files
        .into_iter()
        .find(|f| f.file_name == file_path)
    {
        Some(file) => file,
        None => {
            let mut file = File::default();

            for dir in contents.directories {
                if let Some(f) =
                    sqlx::query_as::<_, File>("SELECT * FROM files WHERE file_path = $1")
                        .bind(format!("{}/{}", dir.dir_path, file_path))
                        .fetch_optional(crate::database::db())
                        .await
                        .unwrap_or(None)
                {
                    file = f;
                } else {
                    return StatusCode::NOT_FOUND.into_response();
                }
            }

            file
        }
    };

    let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
    match tokio::fs::read(file_path).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, &file.mime_type)], bytes).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
