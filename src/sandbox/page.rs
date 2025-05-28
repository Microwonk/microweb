use std::{collections::HashMap, io::Cursor, path::PathBuf};

use axum::{
    Extension, Json,
    extract::{Multipart, Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use zip::ZipArchive;

use crate::{
    apps::Apps,
    files::{DIRECTORY, get_directory_contents, get_full_path},
    models::{Directory, File, SandboxPage, User},
};

pub static SANDBOX_DIR: &str = "/~/sandbox";

#[derive(Deserialize, Debug)]
pub struct UploadPageQuery {
    slug: String,
}

#[tracing::instrument(skip(user, multipart))]
pub async fn create(
    Query(q): Query<UploadPageQuery>,
    user: Option<Extension<User>>,
    multipart: Multipart,
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
        Err(err) => return (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response(),
    };

    let _files = match upload_zip(dir.id, full_path, &q.slug, multipart).await {
        Ok(files) => files,
        Err(errs) => return (StatusCode::BAD_REQUEST, Json(errs)).into_response(),
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

pub async fn upload_zip(
    parent_id: i32,
    root_path: String,
    slug: &str,
    mut multipart: Multipart,
) -> Result<Vec<File>, Vec<String>> {
    let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM directories WHERE id = $1")
        .bind(parent_id)
        .fetch_one(crate::database::db())
        .await
        .unwrap_or(0);
    if exists == 0 {
        return Err(vec!["Directory does not exist.".to_string()]);
    }

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    let Some(field) = multipart.next_field().await.unwrap_or(None) else {
        return Err(vec!["No file provided.".to_string()]);
    };

    let data = match field.bytes().await {
        Ok(d) => d,
        Err(e) => return Err(vec![e.to_string()]),
    };

    let mut archive = match ZipArchive::new(Cursor::new(data)) {
        Ok(a) => a,
        Err(e) => return Err(vec![e.to_string()]),
    };

    let db = crate::database::db();
    let mut dir_cache: HashMap<String, i32> = HashMap::new();
    dir_cache.insert("".into(), parent_id); // root

    // Before the loop
    let mut found_index_html = false;

    // Inside the for loop
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        if entry.is_dir() {
            continue;
        }

        let path = PathBuf::from(entry.name());
        let is_root_index = path == PathBuf::from("index.html");
        if is_root_index {
            found_index_html = true;
        }

        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("file_{}", Uuid::new_v4()));

        let mut current_path = root_path.clone();
        let mut current_parent = parent_id;

        for component in path.parent().unwrap_or(PathBuf::new().as_path()) {
            let comp_str = component.to_string_lossy().to_string();
            current_path = format!("{current_path}/{comp_str}");
            if dir_cache.contains_key(&current_path) {
                current_parent = dir_cache[&current_path];
                continue;
            }

            let existing =
                sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE dir_path = $1")
                    .bind(&current_path)
                    .fetch_optional(db)
                    .await
                    .unwrap_or(None);

            let dir_id = if let Some(d) = existing {
                d.id
            } else {
                let d = sqlx::query_as::<_, Directory>(
                    r#"
                INSERT INTO directories (parent_id, dir_name, dir_path)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
                )
                .bind(current_parent)
                .bind(&comp_str)
                .bind(&current_path)
                .fetch_one(db)
                .await
                .unwrap();
                d.id
            };

            dir_cache.insert(current_path.clone(), dir_id);
            current_parent = dir_id;
        }

        let id = Uuid::new_v4();
        let mime_type = mime_guess::from_path(&file_name)
            .first_or_octet_stream()
            .to_string();
        let dir_id = current_parent;
        let full_path = format!(
            "{}/{}",
            get_full_path(Some(dir_id))
                .await
                .iter()
                .fold("".into(), |acc, d| format!("{}/{}", acc, d.dir_name)),
            file_name
        );

        if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM files WHERE file_path = $1")
            .bind(&full_path)
            .fetch_one(db)
            .await
            .unwrap_or(0)
            > 0
        {
            errors.push(format!("Duplicate file: {file_name}"));
            continue;
        }

        let file = sqlx::query_as::<_, File>(
            r#"
        INSERT INTO files (id, directory_id, file_name, file_path, mime_type, uploaded_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
    "#,
        )
        .bind(id)
        .bind(dir_id)
        .bind(&file_name)
        .bind(&full_path)
        .bind(&mime_type)
        .bind(Utc::now())
        .fetch_one(db)
        .await;

        match file {
            Ok(res) => {
                let disk_path = PathBuf::from(DIRECTORY).join(id.to_string());
                let mut buf = Vec::new();
                if let Err(e) = std::io::copy(&mut entry, &mut buf).map(|_| ()) {
                    errors.push(e.to_string());
                    continue;
                }

                // If it's index.html in root, modify the HTML
                if is_root_index && let Ok(mut html) = String::from_utf8(buf.clone()) {
                    let base_tag = format!(r#"<base href="{}/{slug}/"/>"#, Apps::SandBox.url());
                    if let Some(idx) = html.find("<head>") {
                        html.insert_str(idx + 6, &base_tag);
                        buf = html.into_bytes();
                    }
                }

                if let Err(e) = tokio::fs::write(disk_path, buf).await {
                    errors.push(e.to_string());
                    continue;
                }
                uploaded_files.push(res);
            }
            Err(e) => errors.push(e.to_string()),
        }
    }

    // After the loop
    if !found_index_html {
        return Err(vec!["Missing index.html in root of zip.".to_string()]);
    }

    if uploaded_files.is_empty() {
        Err(errors)
    } else {
        Ok(uploaded_files)
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

    match sqlx::query(
        "DELETE FROM directories d USING sandbox s WHERE s.directory_id = d.id AND s.id = $1",
    )
    .bind(id)
    .execute(crate::database::db())
    .await
    {
        Ok(_) => (),
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json([err.to_string()])).into_response();
        }
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

#[tracing::instrument]
pub async fn view_static(Path((slug, file_path)): Path<(String, String)>) -> impl IntoResponse {
    let file = match sqlx::query_as::<_, File>("SELECT * FROM files WHERE file_path = $1")
        .bind(format!("{SANDBOX_DIR}/{slug}/{file_path}"))
        .fetch_one(crate::database::db())
        .await
    {
        Ok(file) => file,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();
    match tokio::fs::read(file_path).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, &file.mime_type)], bytes).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
