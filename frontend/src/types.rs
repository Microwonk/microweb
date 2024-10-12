use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IsAdminResponse {
    pub admin: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub markdown_content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub media_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadReturn {
    success: Vec<Media>,
    failure: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub markdown_content: String,
}
