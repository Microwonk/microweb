use crate::{ApiError, Media, ServerState};
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

pub async fn upload(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    // Ensure the directory exists
    create_dir_all("public/files").await.map_err(|_| {
        ApiError::new(
            "public/files could not be created.",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

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

        // Create a path for the soon-to-be file
        let file_path = format!("public/files/{}", file_name);

        if field.name().unwrap() == "file_upload" {
            // Unwrap the incoming bytes
            let data = match field.bytes().await {
                Ok(data) => data,
                Err(_) => {
                    failures.push(format!("Could not read bytes for file: {}", file_name));
                    continue;
                }
            };

            // Open a handle to the file (in binary write mode)
            let mut file = match File::create(&file_path).await {
                Ok(file) => file,
                Err(_) => {
                    failures.push(format!("Could not open handle for file: {}", file_name));
                    continue;
                }
            };

            // Write the incoming data to the handle
            if (file.write_all(&data).await).is_err() {
                failures.push(format!("Could not write data for file: {}", file_name));
                continue;
            }

            let media_type = match content_type.as_str() {
                "image/jpeg" | "image/png" => "image",
                "video/mp4" | "video/mpeg" => "video",
                "text/plain" | "application/pdf" => "document",
                _ => "unknown",
            };

            // Try to insert media into the database
            match sqlx::query_as::<_, Media>(
                r#"
                INSERT INTO media (post_id, name, static_path, media_type)
                VALUES ($1, $2, $3, $4)
                RETURNING id, post_id, name, static_path, media_type, created_at
                "#,
            )
            .bind(post_id)
            .bind(file_name.clone())
            .bind(file_path)
            .bind(media_type)
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
    Ok((StatusCode::OK, Json(response)))
}
