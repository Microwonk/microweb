use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;

use crate::{admin_check, created, ok, ApiError, ApiResult, NewPost, Post, ServerState, User};

pub async fn create_post(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(post): Json<NewPost>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    let result = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (
            author, title, description, slug, markdown_content
        )
        VALUES (
            $1, $2, $3, $4, $5
        )
        RETURNING id, author, slug, title, description, markdown_content, created_at, updated_at
        "#,
    )
    .bind(identity.id)
    .bind(post.title.as_str())
    .bind(post.description)
    .bind(post.title.replace(" ", "_").to_ascii_lowercase())
    .bind(post.markdown_content)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(post) => created!(post),
        Err(e) => Err(ApiError::werr(
            "Error creating post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_posts(State(state): State<ServerState>) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all posts.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_posts_by_identity(
    State(state): State<ServerState>,
    Extension(identity): Extension<User>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE author = $1")
        .bind(identity.id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all posts.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_posts_by_user(
    Path(id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE author = $1")
        .bind(id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all posts.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_post(
    Path(slug): Path<String>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = $1")
        .bind(slug.as_str())
        .fetch_one(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            format!("Error retrieving post with slug {}.", slug),
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn delete_post(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => ok!(),
        Err(e) => Err(ApiError::werr(
            "Could not delete Post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn update_post(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(post): Json<NewPost>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET title = $1, slug = $2, description = $3, markdown_content = $4, updated_at = $5
        WHERE id = $6
        RETURNING id, author, title, slug, markdown_content, created_at, updated_at, description
        "#,
    )
    .bind(post.title.as_str())
    .bind(post.title.replace(" ", "_").to_ascii_lowercase())
    .bind(post.description)
    .bind(post.markdown_content)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Could not update Post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_post_by_id(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
