use std::path::PathBuf;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

use serde::Deserialize;

use common::{
    api::{ApiError, ApiResult},
    models::{Directory, User},
};

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
) -> ApiResult<Json<Directory>> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    if let Some(id) = directory.parent_id
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
            "Parent directory does not exist.".to_string(),
        ));
    }

    let full_path = format!(
        "{}/{}",
        get_full_path(directory.parent_id)
            .await
            .iter()
            .fold(String::new(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
        directory.name,
    );

    common::db_query_as!(
        Directory,
        fetch_one,
        "INSERT INTO directories (parent_id, dir_name, dir_path) VALUES ($1, $2, $3) RETURNING *",
        directory.parent_id,
        &directory.name,
        &full_path
    )
    .map(Json)
    .map_err(Into::into)
}

#[tracing::instrument(skip(user))]
pub async fn delete_by_id(Path(id): Path<i32>, user: Option<Extension<User>>) -> ApiResult<()> {
    if !user.as_ref().is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let mut errors = Vec::new();

    let contents = get_directory_contents(Some(id)).await?;

    for dir in contents.directories {
        Box::pin(delete_by_id(Path(dir.id), user.clone())).await?;
    }

    let success = common::db_query!(execute, "DELETE FROM directories WHERE id = $1", "", id)
        .map_or(0, |r| r.rows_affected());

    if success == 1 {
        for file in contents.files {
            let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
            if let Err(e) = tokio::fs::remove_file(file_path).await {
                errors.push(e.to_string());
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ApiError::MultipleMessages(
            StatusCode::INTERNAL_SERVER_ERROR,
            errors,
        ))
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
) -> ApiResult<impl IntoResponse> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let contents = q.contents.is_some_and(|c| c);

    if id == 0 {
        if contents {
            return get_directory_contents(None)
                .await
                .map(|r| Json(r).into_response());
        } else {
            return Ok((StatusCode::OK, Json(Directory::root())).into_response());
        }
    }

    let dir = common::db_query_as!(
        Directory,
        fetch_optional,
        "SELECT * FROM directories WHERE id = $1",
        id
    )
    .unwrap_or(None);

    if let Some(dir) = dir {
        if contents {
            return get_directory_contents(Some(dir.id))
                .await
                .map(|r| Json(r).into_response());
        } else {
            return Ok((StatusCode::OK, Json(dir)).into_response());
        }
    }

    Err(ApiError::not_found())
}

#[tracing::instrument]
pub async fn traverse(
    Path(dir_path): Path<String>,
    user: Option<Extension<User>>,
    Query(q): Query<GetContentsQuery>,
) -> ApiResult<impl IntoResponse> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let contents = q.contents.is_some_and(|c| c);

    if dir_path == ROOT {
        if contents {
            return get_directory_contents(None)
                .await
                .map(|r| Json(r).into_response());
        } else {
            return Ok((StatusCode::OK, Json(Directory::root())).into_response());
        }
    }

    if let Some(dir) = common::db_query_as!(
        Directory,
        fetch_optional,
        "SELECT * FROM directories WHERE dir_path = $1",
        format!("/{dir_path}")
    )
    .unwrap_or(None)
    {
        if contents {
            return get_directory_contents(Some(dir.id))
                .await
                .map(|r| Json(r).into_response());
        } else {
            return Ok((StatusCode::OK, Json(dir)).into_response());
        }
    }

    Err(ApiError::not_found())
}
