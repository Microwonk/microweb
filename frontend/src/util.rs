use std::{error::Error, sync::Arc};

use tokio::sync::RwLock;

use codee::string::JsonSerdeCodec;
use gloo_net::http::Request;
use lazy_static::lazy_static;
use leptos::SignalGetUntracked;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};
use serde::{Deserialize, Serialize};

use crate::types::{
    IsAdminResponse, LoginRequest, LoginResponse, Post, Profile, RegisterRequest, User,
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
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
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
            Err(e) => Err(serde_json::from_str(e.to_string().as_str()).map_err(ApiError::json)?),
        }
    }

    pub async fn logout() {
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
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

    pub async fn all_users() -> Result<Vec<User>, ApiError> {
        Self::simple_get(format!("{}/users", API_PATH), true).await
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

    pub async fn register(
        email: impl Into<String>,
        password: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<(), ApiError> {
        let (_, set_token) = use_cookie_with_options::<LoginResponse, JsonSerdeCodec>(
            "token",
            UseCookieOptions::default()
                .max_age(3_600_000 * 24) // one day
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
            Err(e) => Err(serde_json::from_str(e.to_string().as_str()).map_err(ApiError::json)?),
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
