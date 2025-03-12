use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;

use crate::{
    admin_check, created, logs_handler::Log, ok, ApiError, ApiResult, NewPost, Post, ProcessedPost,
    ServerState, User,
};

pub async fn create_post(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(post): Json<NewPost>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
    let result = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (
            author, title, description, slug, markdown_content
        )
        VALUES (
            $1, $2, $3, $4, $5
        )
        RETURNING *
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
        Ok(post) => {
            Log::info(
                format!("New Blog Post [{} | {}] created.", post.id, post.title),
                &state,
            )
            .await?;
            created!(post)
        }

        Err(e) => Err(ApiError::werr(
            "Error creating post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn release_post(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    // TODO: implement mailing on release? newsletter?
    admin_check(&identity, &state).await?;
    releaser(id, state, true).await
}

pub async fn unrelease_post(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
    releaser(id, state, false).await
}

async fn releaser(id: i32, state: ServerState, release: bool) -> ApiResult<impl IntoResponse> {
    match if release {
        sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET released = $1, release_date = $2
            WHERE id = $3
            RETURNING *
            "#,
        )
        .bind(release)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&state.pool)
    } else {
        sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET released = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(release)
        .bind(id)
        .fetch_one(&state.pool)
    }
    .await
    {
        Ok(post) => {
            Log::info(
                format!(
                    "Blog Post [{} | {}] {}.",
                    post.id,
                    post.title,
                    if release { "released" } else { "unreleased" }
                ),
                &state,
            )
            .await?;
            ok!(post)
        }
        Err(e) => Err(ApiError::werr(
            "Could not update Post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_posts(State(state): State<ServerState>) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.release_date,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE released = true ORDER BY release_date DESC"#,
    )
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
    admin_check(&identity, &state).await?;
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.release_date,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE author = $1 ORDER BY created_at DESC"#,
    )
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
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.created_at,
            posts.updated_at
        FROM posts
        WHERE author = $1 ORDER BY created_at DESC"#,
    )
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
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.release_date,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE posts.slug = $1"#,
    )
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

pub async fn get_post_by_id(
    Path(id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.release_date,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE posts.id = $1"#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            format!("Error retrieving post with id {}.", id),
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
    admin_check(&identity, &state).await?;
    match sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => {
            Log::info(format!("Blog Post with id {} deleted.", id), &state).await?;
            ok!()
        }
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
    admin_check(&identity, &state).await?;
    match sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET title = $1, slug = $2, description = $3, markdown_content = $4, updated_at = $5
        WHERE id = $6
        RETURNING *
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
        Ok(post) => {
            Log::info(
                format!("Blog Post [{} | {}] updated.", post.id, post.title),
                &state,
            )
            .await?;
            ok!(post)
        }
        Err(e) => Err(ApiError::werr(
            "Could not update Post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
