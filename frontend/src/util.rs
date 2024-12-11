use std::{error::Error, sync::Arc};

use tokio::sync::RwLock;

use codee::string::JsonSerdeCodec;
use gloo_net::http::Request;
use lazy_static::lazy_static;
use leptos::SignalGetUntracked;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};
use serde::{Deserialize, Serialize};
use web_sys::{File, FormData};

use crate::types::{
    Comment, IsAdminResponse, LogEntry, LoginRequest, LoginResponse, Media, NewComment, NewPost,
    Post, Profile, RegisterRequest, UploadReturn, User, UserUpdate,
};

// fn generate_json_ld(post: &Post) -> String {
//     format!(
//         r#"{{
//             "@context": "https://schema.org",
//             "@type": "BlogPosting",
//             "headline": "{}",
//             "description": "{}",
//             "datePublished": "{}",
//             "author": {{
//                 "@type": "Person",
//                 "name": "{}"
//             }}
//         }}"#,
//         post.title, post.description, post.created_at, post.author
//     )
// }

// fn inject_json_ld(schema_data: &str) {
//     web_sys::window()
//         .unwrap()
//         .document()
//         .unwrap()
//         .get_elements_by_tag_name("head")
//         .item(0)
//         .unwrap()
//         .insert_adjacent_html(
//             "beforeend",
//             &format!(
//                 "<script type=\"application/ld+json\">{}</script>",
//                 schema_data
//             ),
//         )
//         .expect("Failed to inject JSON-LD");
// }

pub const API_PATH: &str = "https://blogapi.nicolas-frey.com";

lazy_static! {
    pub static ref TOKEN: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
}

#[derive(Clone, Debug)]
pub struct Api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub error_info: Option<String>,
    pub status_code: u16,
}

impl ApiError {
    pub fn json(error: impl Error) -> Self {
        Self {
            message: "JSON parse error".into(),
            status_code: 418, // I'm a teapot
            error_info: Some(error.to_string()),
        }
    }
}

impl Api {
    pub async fn login(
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<(), ApiError> {
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
                .path("/")
                .same_site(SameSite::Strict),
        );
        match Request::post(format!("{}/login", API_PATH).as_str())
            .json(&LoginRequest {
                email: email.into(),
                password: password.into(),
            })
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => {
                let token = response
                    .json::<LoginResponse>()
                    .await
                    .map_err(ApiError::json)?;
                set_token(Some(token.clone()));
                *TOKEN.write().await = token.token;
                Ok(())
            }
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn update_blog_post(post_id: i32, post: NewPost) -> Result<Post, ApiError> {
        match Request::put(format!("{}/user/post/{}", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .json(&post)
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn update_user(user_id: i32, user: UserUpdate) -> Result<User, ApiError> {
        match Request::put(format!("{}/user/{}", API_PATH, user_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .json(&user)
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn logout() {
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
                .path("/")
                .same_site(SameSite::Strict),
        );
        set_token(None);
        *TOKEN.write().await = "".to_string();
    }

    pub async fn get_profile() -> Result<Profile, ApiError> {
        Self::simple_get(format!("{}/profile", API_PATH), true).await
    }

    pub async fn get_post(slug: impl Into<String>) -> Result<Post, ApiError> {
        Self::simple_get(format!("{}/post/{}", API_PATH, slug.into()), false).await
    }

    pub async fn all_blog_posts() -> Result<Vec<Post>, ApiError> {
        Self::simple_get(format!("{}/posts", API_PATH), false).await
    }

    pub async fn admin_blog_posts() -> Result<Vec<Post>, ApiError> {
        Self::simple_get(format!("{}/user/posts", API_PATH), true).await
    }

    pub async fn all_users() -> Result<Vec<User>, ApiError> {
        Self::simple_get(format!("{}/users", API_PATH), true).await
    }

    pub async fn all_media() -> Result<Vec<Media>, ApiError> {
        Self::simple_get(format!("{}/media", API_PATH), true).await
    }

    pub async fn get_comments(post_id: i32) -> Result<Vec<Comment>, ApiError> {
        Self::simple_get(format!("{}/post/{}/comments", API_PATH, post_id), false).await
    }

    pub async fn get_logs() -> Result<Vec<LogEntry>, ApiError> {
        Self::simple_get(format!("{}/admin/logs", API_PATH), true).await
    }

    pub async fn get_rss() -> Result<String, ApiError> {
        match Request::get(format!("{}/rss", API_PATH).as_str())
            .send()
            .await
        {
            Ok(response) => Ok(response.text().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    async fn simple_get<T: for<'de> Deserialize<'de>>(
        path: impl Into<String>,
        authenticated: bool,
    ) -> Result<T, ApiError> {
        let mut r = Request::get(path.into().as_str());
        if authenticated {
            r = r.header("Authorization", &format!("Bearer {}", TOKEN.read().await));
        }
        match r.send().await {
            Ok(response) => Ok(response.json::<T>().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn post_unrelease(post_id: i32) -> Result<Post, ApiError> {
        match Request::post(format!("{}/user/post/{}/unrelease", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn post_release(post_id: i32) -> Result<Post, ApiError> {
        match Request::post(format!("{}/user/post/{}/release", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn create_post() -> Result<Post, ApiError> {
        match Request::post(format!("{}/user/post", API_PATH).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .json(&NewPost {
                title: "Your New Blog Post!".into(),
                description: "A Very Cool Blog Post.".into(),
                markdown_content: r#"
# Heading
```rs
pub fn main() {
    println("Hello World!");
}
"#
                .into(),
            })
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn create_comment(post_id: i32, comment: NewComment) -> Result<(), ApiError> {
        match Request::post(format!("{}/post/{}/comment", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .json(&comment)
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn upload(post_id: i32, files: Vec<File>) -> Result<UploadReturn, ApiError> {
        let form_data = FormData::new().map_err(|e| ApiError {
            message: "Could not create Form.".into(),
            error_info: e.as_string(),
            status_code: 418,
        })?;

        for file in files {
            form_data
                .append_with_blob("file_upload", &file)
                .map_err(|e| ApiError {
                    message: "Could not create Form.".into(),
                    error_info: e.as_string(),
                    status_code: 418,
                })?;
        }

        match Request::post(format!("{}/post/{}/upload", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .body(form_data)
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => Ok(response.json().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn post_delete(post_id: i32) -> Result<(), ApiError> {
        match Request::delete(format!("{}/user/post/{}", API_PATH, post_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                message: "Failed to delete post.".into(),
                error_info: Some(e.to_string()),
                status_code: 418,
            }),
        }
    }

    pub async fn delete_media(upload_id: i32) -> Result<(), ApiError> {
        match Request::delete(format!("{}/upload/{}", API_PATH, upload_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                message: "Failed to delete upload/media.".into(),
                error_info: Some(e.to_string()),
                status_code: 418,
            }),
        }
    }

    pub async fn delete_user(user_id: i32) -> Result<(), ApiError> {
        match Request::delete(format!("{}/user/{}", API_PATH, user_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                message: "Failed to delete upload/media.".into(),
                error_info: Some(e.to_string()),
                status_code: 418,
            }),
        }
    }

    pub async fn delete_comment(comment_id: i32) -> Result<(), ApiError> {
        match Request::delete(format!("{}/comment/{}", API_PATH, comment_id).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                message: "Could not delete Comment.".into(),
                error_info: Some(e.to_string()),
                status_code: 418,
            }),
        }
    }

    pub async fn register(
        email: impl Into<String>,
        password: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<(), ApiError> {
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
                .path("/")
                .same_site(SameSite::Strict),
        );
        match Request::post(format!("{}/register", API_PATH).as_str())
            .json(&RegisterRequest {
                name: name.into(),
                email: email.into(),
                password: password.into(),
            })
            .map_err(ApiError::json)?
            .send()
            .await
        {
            Ok(response) => {
                let token = response
                    .json::<LoginResponse>()
                    .await
                    .map_err(ApiError::json)?;
                set_token(Some(token.clone()));
                *TOKEN.write().await = token.token;
                Ok(())
            }
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn is_admin() -> Result<IsAdminResponse, ApiError> {
        if !Self::is_logged_in().await {
            return Ok(IsAdminResponse { admin: false });
        }
        Self::simple_get(format!("{}/user/admin", API_PATH), true).await
    }

    pub async fn initialize() -> bool {
        let (token, _) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
                .path("/")
                .same_site(SameSite::Strict),
        );
        let token = token.get_untracked();
        if let Some(t) = token {
            *TOKEN.write().await = t.token;
            true
        } else {
            false
        }
    }

    pub async fn is_logged_in() -> bool {
        !TOKEN.read().await.is_empty()
    }
}
