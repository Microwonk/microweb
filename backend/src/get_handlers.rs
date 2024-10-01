use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{ApiError, Media, ServerState};

pub async fn get_all_media(
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, ApiError> {
    match sqlx::query_as::<_, Media>("SELECT * FROM media")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_media_by_post(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, ApiError> {
    match sqlx::query_as::<_, Media>("SELECT * FROM media WHERE post_id = $1")
        .bind(post_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_media(
    Path(media_id): Path<i32>,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, ApiError> {
    match sqlx::query_as::<_, Media>("SELECT * FROM media WHERE id = $1")
        .bind(media_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
