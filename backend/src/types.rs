use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// macro_rules! generate_types {
//     ($($model_name:ident {
//         $($field_name:ident: $field_type:ty),+ $(,)?
//     } => $new_model_name:ident),+ $(,)?) => {
//         $(
//             #[derive(Debug, Serialize, Deserialize, FromRow)]
//             pub struct $model_name {
//                 pub id: i32,
//                 $(pub $field_name: $field_type),+
//             }

//             #[derive(Debug, Serialize, Deserialize)]
//             pub struct $new_model_name {
//                 $(pub $field_name: $field_type),+
//             }
//         )+
//     };
// }

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub markdown_content: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub markdown_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    // pub admin: bool,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub static_path: String,
    pub media_type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
