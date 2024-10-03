use std::{io::BufWriter, num::NonZeroU32};

use crate::{admin_check, ok, ApiError, ApiResult, Media, MediaNoData, ServerState, User};
use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use fast_image_resize as fr;
use image::{
    codecs::{jpeg::JpegEncoder, png::PngEncoder},
    ColorType, ImageEncoder, ImageReader,
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

            let compressed_data = match content_type.as_str() {
                "image/jpeg" | "image/png" => match compress_image(&data, content_type.as_str()) {
                    Ok(compressed_img) => compressed_img,
                    Err(err) => {
                        failures.push(format!(
                            "Image compression failed for file: {} with error: {}",
                            file_name, err
                        ));
                        continue;
                    }
                },
                "image/gif" => {
                    // TODO: implement gif compression
                    data
                }
                "video/mp4" => {
                    // TODO: implement video compression
                    data
                }
                _ => {
                    failures.push(format!("Unsupported content type for file: {}", file_name));
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
            .bind(compressed_data) // Store the compressed data
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

fn compress_image(data: &[u8], content_type: &str) -> Result<Vec<u8>, String> {
    // Convert DynamicImage to an RGBA buffer
    let img = ImageReader::new(std::io::Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    // Multiple RGB channels of source image by alpha channel
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(480).unwrap();
    let dst_height = NonZeroU32::new(360).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Write destination image as PNG/JPEG-file
    let mut result_buf = BufWriter::new(Vec::new());

    match content_type {
        "image/jpeg" => JpegEncoder::new(&mut result_buf)
            .write_image(
                dst_image.buffer(),
                dst_width.get(),
                dst_height.get(),
                ColorType::Rgba8.into(),
            )
            .unwrap(),
        "image/png" => PngEncoder::new(&mut result_buf)
            .write_image(
                dst_image.buffer(),
                dst_width.get(),
                dst_height.get(),
                ColorType::Rgba8.into(),
            )
            .unwrap(),
        _ => return Err("Unsupported image format".to_string()),
    }

    let image_bytes = result_buf.into_inner().unwrap();

    Ok(image_bytes)
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
            // Cache Control for 7 days
            ("Cache-Control", "public, max-age=604800".to_owned()),
            // unique identifier for caching
            ("ETag", media.name),
            // last modified with datetime value of HTTP date format RFC 7231
            (
                "Last-Modified",
                media
                    .created_at
                    .format("%a, %d %b %Y %H:%M:%S GMT")
                    .to_string(),
            ),
            ("Content-Type", media.media_type),
        ],
        Bytes::from(media.data),
    ))
}

pub async fn delete_media(
    Extension(identity): Extension<User>,
    Path(id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query("DELETE FROM media WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => ok!(),
        Err(e) => Err(ApiError::werr(
            "Could not delete Media.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
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
