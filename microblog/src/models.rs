use rss::{Channel, Item};
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
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Media {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub media_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UploadReturn {
    success: Vec<Media>,
    failure: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub markdown_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Comment {
    pub id: i32,
    pub author_name: Option<String>,
    pub author_id: Option<i32>,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct NewComment {
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserUpdate {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct LogEntry {
    pub id: i32,
    pub message: String,
    pub context: String,
    pub log_time: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Default)]
pub struct RssFeed {
    pub title: String,
    pub description: String,
    pub link: String,
    pub language: String,
    pub items: Vec<RssEntry>,
}

impl From<Channel> for RssFeed {
    fn from(c: Channel) -> Self {
        let items: Vec<RssEntry> = c.items().iter().cloned().map(|i| i.into()).collect();
        Self {
            title: c.title,
            description: c.description,
            link: c.link,
            language: c.language.unwrap_or_default(),
            items,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RssEntry {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub author: String,
    pub guid: String,
}

impl From<Item> for RssEntry {
    fn from(i: Item) -> Self {
        Self {
            title: i.title.unwrap_or_default(),
            link: i.link.unwrap_or_default(),
            description: i.description.unwrap_or_default(),
            pub_date: i.pub_date.unwrap_or_default(),
            author: i.author.unwrap_or_default(),
            guid: i.guid.unwrap_or_default().value,
        }
    }
}
