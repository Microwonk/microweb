use crate::{admin_check, ok, ApiError, ApiResult, Media, MediaNoData, ServerState, User};
use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

pub async fn upload(
    Extension(identity): Extension<User>,
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
    mut multipart: Multipart,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;

    let mut successes = vec![];
    let mut failures = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        // Grab the name of the file
        let file_name = match field.file_name().map(|f| {
            let orig_name: Vec<&str> = f.split('.').collect();
            format!(
                "{}_post{}_{}.{}",
                orig_name[0],
                post_id,
                Uuid::new_v4(),
                orig_name[1]
            )
        }) {
            Some(name) => name,
            None => {
                failures.push("Failed to read file name.".to_string());
                continue;
            }
        };

        let content_type = match field.content_type().map(String::from) {
            Some(content_type) => content_type,
            None => {
                failures.push(format!(
                    "Failed to read content type for file: {}",
                    file_name
                ));
                continue;
            }
        };

        if field.name().unwrap() == "file_upload" {
            // Unwrap the incoming bytes
            let data = match field.bytes().await {
                Ok(data) => data.to_vec(), // Convert Bytes to Vec<u8>
                Err(_) => {
                    failures.push(format!("Could not read bytes for file: {}", file_name));
                    continue;
                }
            };

            // Try to insert media into the database
            match sqlx::query_as::<_, Media>(
                r#"
                INSERT INTO media (post_id, name, data, media_type)
                VALUES ($1, $2, $3, $4)
                RETURNING id, post_id, name, data, media_type, created_at
                "#,
            )
            .bind(post_id)
            .bind(file_name.clone())
            .bind(data) // Store the binary data
            .bind(content_type) // directly store MIME
            .fetch_one(&state.pool)
            .await
            {
                Ok(media) => successes.push(media),
                Err(_) => failures.push(format!("Database insert failed for file: {}", file_name)),
            }
        }
    }

    // Prepare response with both successes and failures
    let response = serde_json::json!({
        "success": successes,
        "failure": failures
    });

    // Return the response
    ok!(response)
}

pub async fn get_upload(
    Path(id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    let media: Media = match sqlx::query_as::<_, Media>("SELECT * FROM media WHERE id = $1 ")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(media) => media,
        Err(e) => {
            return Err(ApiError::werr("Asset not Found.", StatusCode::NOT_FOUND, e));
        }
    };

    Ok((
        StatusCode::OK,
        [
            (
                "Content-Disposition",
                format!("inline; filename=\"{}\"", media.name),
            ),
            ("Content-Type", media.media_type),
        ],
        Bytes::from(media.data),
    ))
}

pub async fn get_all_media(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;

    match sqlx::query_as::<_, MediaNoData>(
        "SELECT id, post_id, name, media_type, created_at FROM media",
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_media_by_post(
    Extension(identity): Extension<User>,
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, MediaNoData>(
        "SELECT id, post_id, name, media_type, created_at FROM media WHERE post_id = $1",
    )
    .bind(post_id)
    .fetch_all(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
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
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, MediaNoData>(
        "SELECT id, post_id, name, media_type, created_at FROM media WHERE id = $1",
    )
    .bind(media_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
