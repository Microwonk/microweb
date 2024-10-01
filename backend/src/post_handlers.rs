use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{ApiError, NewPost, Post, ServerState};

pub async fn create_post(
    Path(author): Path<i32>,
    State(state): State<ServerState>,
    Json(post): Json<NewPost>,
) -> Result<impl IntoResponse, ApiError> {
    // Insert skills into the database and return the created row
    let result = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (
            author, title, markdown_content
        )
        VALUES (
            $1, $2, $3
        )
        RETURNING id, author, title, markdown_content, created_at, updated_at
        "#,
    )
    .bind(author)
    .bind(post.title)
    .bind(post.markdown_content)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(post) => Ok((StatusCode::CREATED, Json(post))),
        Err(e) => Err(ApiError::werr(
            "Error creating post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_posts(
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, ApiError> {
    match sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all posts.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
