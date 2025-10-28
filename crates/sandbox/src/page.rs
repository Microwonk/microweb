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

use common::{
    api::{ApiError, ApiResult},
    models::{Directory, File, SandboxPage, User},
};

use files::{DIRECTORY, directory};

use common::Apps;

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
) -> ApiResult<Json<SandboxPage>> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let Some(sbx_dir) = common::db_query_as!(
        Directory,
        fetch_optional,
        "SELECT * FROM directories WHERE dir_path = $1",
        SANDBOX_DIR
    )
    .unwrap_or(None) else {
        return Err(ApiError::Message(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Sandbox directory does not exist.".to_string(),
        ));
    };

    let full_path = format!(
        "{}/{}",
        files::get_full_path(Some(sbx_dir.id))
            .await
            .iter()
            .fold(String::new(), |acc, dir| format!("{acc}/{}", dir.dir_name)),
        &q.slug,
    );

    let dir = common::db_query_as!(
        Directory,
        fetch_one,
        r#"
        INSERT INTO directories (parent_id, dir_name, dir_path)
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
        sbx_dir.id,
        &q.slug,
        &full_path,
    )?;

    let _files = upload_zip(dir.id, full_path, &q.slug, multipart).await?;

    common::db_query_as!(
        SandboxPage,
        fetch_one,
        r#"
        INSERT INTO sandbox (directory_id, slug)
        VALUES ($1, $2)
        RETURNING *
    "#,
        dir.id,
        &q.slug,
    )
    .map(Json)
    .map_err(Into::into)
}

pub async fn upload_zip(
    parent_id: i32,
    root_path: String,
    slug: &str,
    mut multipart: Multipart,
) -> ApiResult<Vec<File>> {
    let exists = common::db_query_scalar!(
        i64,
        fetch_one,
        "SELECT COUNT(*) FROM directories WHERE id = $1",
        parent_id
    )
    .unwrap_or(0);
    if exists == 0 {
        return Err(ApiError::Message(
            StatusCode::BAD_REQUEST,
            "Directory does not exist.".to_string(),
        ));
    }

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    let field = multipart.next_field().await?.ok_or(ApiError::Message(
        StatusCode::BAD_REQUEST,
        "At least one file should be in multipart.".to_string(),
    ))?;

    let data = field.bytes().await?;

    let mut archive = ZipArchive::new(Cursor::new(data)).map_err(ApiError::any)?;

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
        let is_root_index = path.to_str().is_some_and(|p| p == "index.html");
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

            let existing = common::db_query_as!(
                Directory,
                fetch_optional,
                "SELECT * FROM directories WHERE dir_path = $1",
                &current_path
            )
            .unwrap_or(None);

            let dir_id = if let Some(d) = existing {
                d.id
            } else {
                let d = common::db_query_as!(
                    Directory,
                    fetch_one,
                    r#"
                    INSERT INTO directories (parent_id, dir_name, dir_path)
                    VALUES ($1, $2, $3)
                    RETURNING *
                    "#,
                    current_parent,
                    &comp_str,
                    &current_path,
                )?;
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
            files::get_full_path(Some(dir_id))
                .await
                .iter()
                .fold("".into(), |acc, d| format!("{}/{}", acc, d.dir_name)),
            file_name
        );

        if common::db_query_scalar!(
            i64,
            fetch_one,
            "SELECT COUNT(*) FROM files WHERE file_path = $1",
            &full_path
        )
        .unwrap_or(0)
            > 0
        {
            errors.push(format!("Duplicate file: {file_name}"));
            continue;
        }

        let file = common::db_query_as!(
            File,
            fetch_one,
            r#"
            INSERT INTO files (id, directory_id, file_name, file_path, mime_type, uploaded_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            id,
            dir_id,
            &file_name,
            &full_path,
            &mime_type,
            Utc::now(),
        );

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

    if !found_index_html {
        return Err(ApiError::Message(
            StatusCode::BAD_REQUEST,
            "Missing index.html in root of zip.".to_string(),
        ));
    }

    if uploaded_files.is_empty() {
        Err(ApiError::MultipleMessages(StatusCode::BAD_REQUEST, errors))
    } else {
        Ok(uploaded_files)
    }
}

#[tracing::instrument(skip(user))]
pub async fn list(user: Option<Extension<User>>) -> ApiResult<Json<Vec<SandboxPage>>> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    common::db_query_as!(SandboxPage, fetch_all, "SELECT * FROM sandbox")
        .map(Json)
        .map_err(Into::into)
}

#[tracing::instrument(skip(user))]
pub async fn get_by_id(
    Path(id): Path<Uuid>,
    user: Option<Extension<User>>,
) -> ApiResult<Json<SandboxPage>> {
    if !user.is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    common::db_query_as!(
        SandboxPage,
        fetch_one,
        "SELECT * FROM sandbox WHERE id = $1",
        id
    )
    .map(Json)
    .map_err(Into::into)
}

#[tracing::instrument(skip(user))]
pub async fn delete_by_id(Path(id): Path<Uuid>, user: Option<Extension<User>>) -> ApiResult<()> {
    if !user.as_ref().is_some_and(|u| u.admin) {
        return Err(ApiError::unauthorized());
    };

    let dir_id = common::db_query_scalar!(
        i32,
        fetch_one,
        "SELECT d.id FROM directories d JOIN sandbox s ON s.directory_id = d.id WHERE s.id = $1",
        id
    )?;

    println!("{dir_id}");

    directory::delete_by_id(Path(dir_id), user).await
}

pub async fn view(Path(slug): Path<String>) -> ApiResult<impl IntoResponse> {
    let dir = common::db_query_as!(
        Directory,
        fetch_one,
        "SELECT d.* FROM directories d JOIN sandbox s ON d.id = s.directory_id WHERE s.slug = $1",
        slug
    )
    .map_err(|_| ApiError::not_found())?;

    let contents = files::get_directory_contents(Some(dir.id)).await?;

    let Some(index) = contents.files.iter().find(|f| f.file_name == "index.html") else {
        return Err(ApiError::Message(
            StatusCode::INTERNAL_SERVER_ERROR,
            "index.html does not exist.".to_string(),
        ));
    };

    let file_path: PathBuf = [DIRECTORY, &index.id.to_string()].iter().collect();
    if let Ok(bytes) = tokio::fs::read(file_path).await {
        return Ok(([(header::CONTENT_TYPE, &index.mime_type)], bytes).into_response());
    }

    Err(ApiError::not_found())
}

#[tracing::instrument]
pub async fn view_static(
    Path((slug, file_path)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let file = common::db_query_as!(
        File,
        fetch_one,
        "SELECT * FROM files WHERE file_path = $1",
        format!("{SANDBOX_DIR}/{slug}/{file_path}")
    )
    .map_err(|_| ApiError::not_found())?;

    let file_path: PathBuf = [DIRECTORY, &file.id.to_string()].iter().collect();

    tokio::fs::read(file_path)
        .await
        .map(|bytes| ([(header::CONTENT_TYPE, file.mime_type)], bytes))
        .map_err(|_| ApiError::not_found())
}
