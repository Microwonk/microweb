use serde::{Deserialize, Serialize};

pub static FILE_DIRECTORY: &str = "files";
pub static FILE_ROOT: &str = "~";
pub static FILE_PRIVATE: &str = ".private";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct SandboxPage {
    #[cfg(feature = "back")]
    pub id: uuid::Uuid,
    pub directory_id: i32,
    #[cfg(not(feature = "back"))]
    pub id: String,
    pub slug: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Directory {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub dir_name: String,
    pub dir_path: String,
}

#[cfg(feature = "back")]
impl Directory {
    pub fn root() -> Self {
        Self {
            id: 0,
            parent_id: None,
            dir_name: FILE_ROOT.to_string(),
            dir_path: FILE_ROOT.to_string(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct File {
    #[cfg(feature = "back")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "back"))]
    pub id: String,
    pub directory_id: Option<i32>,
    pub file_name: String,
    pub mime_type: String,
    pub uploaded_at: chrono::NaiveDateTime,
    pub file_path: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectoryContents {
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

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
    pub is_admin: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub author_name: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub markdown_content: String,
    pub released: bool,
    pub release_date: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[cfg(feature = "back")]
impl User {
    /// clones self and makes a UserProfile instance
    pub fn profile(&self) -> Profile {
        let cloned = self.clone();
        Profile {
            id: cloned.id,
            name: cloned.name,
            email: cloned.email,
            is_admin: cloned.admin,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub markdown_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Comment {
    pub id: i32,
    pub author_name: Option<String>,
    pub author_id: Option<i32>,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct NewComment {
    pub content: String,
}
