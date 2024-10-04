use std::{error::Error, sync::Arc};

use tokio::sync::RwLock;

use codee::string::JsonSerdeCodec;
use gloo_net::http::Request;
use lazy_static::lazy_static;
use leptos::SignalGetUntracked;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::types::{IsAdminResponse, LoginRequest, LoginResponse, Post, Profile, RegisterRequest};

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

pub const API_PATH: &str = "https://microblog.shuttleapp.rs";

lazy_static! {
    pub static ref TOKEN: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
}

#[derive(Clone, Debug)]
pub struct Api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    message: String,
    error_info: Option<String>,
    status_code: u16,
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
        let (_, set_token, _) = use_local_storage::<LoginResponse, JsonSerdeCodec>("token");
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
                set_token(token.clone());
                *TOKEN.write().await = token.token;
                Ok(())
            }
            Err(e) => Err(serde_json::from_str(e.to_string().as_str()).map_err(ApiError::json)?),
        }
    }

    pub async fn logout() {
        let (_, _, delete_token) = use_local_storage::<LoginResponse, JsonSerdeCodec>("token");
        delete_token();
        *TOKEN.write().await = "".to_string();
    }

    pub async fn get_profile() -> Result<Profile, ApiError> {
        match Request::get(format!("{}/profile", API_PATH).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(response) => Ok(response.json::<Profile>().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn register(
        email: impl Into<String>,
        password: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<(), ApiError> {
        let (_, set_token, _) = use_local_storage::<LoginResponse, JsonSerdeCodec>("token");
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
                set_token(token.clone());
                *TOKEN.write().await = token.token;
                Ok(())
            }
            Err(e) => Err(serde_json::from_str(e.to_string().as_str()).map_err(ApiError::json)?),
        }
    }

    pub async fn all_blog_posts() -> Result<Vec<Post>, ApiError> {
        match Request::get(format!("{}/posts", API_PATH).as_str())
            // .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(response) => Ok(response.json::<Vec<Post>>().await.map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn is_admin() -> Result<IsAdminResponse, ApiError> {
        if !Self::is_logged_in().await {
            return Ok(IsAdminResponse { admin: false });
        }
        match Request::get(format!("{}/user/admin", API_PATH).as_str())
            .header("Authorization", &format!("Bearer {}", TOKEN.read().await))
            .send()
            .await
        {
            Ok(response) => Ok(response
                .json::<IsAdminResponse>()
                .await
                .map_err(ApiError::json)?),
            Err(e) => Err(ApiError::json(e)),
        }
    }

    pub async fn initialize() -> bool {
        let (token, _, _) = use_local_storage::<LoginResponse, JsonSerdeCodec>("token");
        let token = token.get_untracked();
        if token.token.is_empty() {
            false
        } else {
            *TOKEN.write().await = token.token;
            true
        }
    }

    pub async fn is_logged_in() -> bool {
        !TOKEN.read().await.is_empty()
    }
}
