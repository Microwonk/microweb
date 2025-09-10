use rss::{Channel, Item};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct SandboxPage {
    #[cfg(feature = "ssr")]
    pub id: uuid::Uuid,
    pub directory_id: i32,
    #[cfg(not(feature = "ssr"))]
    pub id: String,
    pub slug: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Directory {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub dir_name: String,
    pub dir_path: String,
}

#[cfg(feature = "ssr")]
impl Directory {
    pub fn root() -> Self {
        Self {
            id: 0,
            parent_id: None,
            dir_name: crate::files::ROOT.to_string(),
            dir_path: crate::files::ROOT.to_string(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct File {
    #[cfg(feature = "ssr")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "ssr"))]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[cfg(feature = "ssr")]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[cfg(feature = "ssr")]
impl From<Post> for RssEntry {
    fn from(post: Post) -> Self {
        use chrono::{TimeZone, Utc};

        let full_url = format!("https://blog.nicolas-frey.com/posts/{}", post.slug);
        let perm_url = format!("https://blog.nicolas-frey.com/posts/{}", post.slug);
        Self {
            title: post.title,
            link: full_url,
            description: post.description,
            pub_date: Utc
                .from_utc_datetime(
                    &post
                        .release_date
                        .unwrap_or(post.updated_at.unwrap_or(post.created_at)),
                )
                .to_rfc2822(),
            author: post.author_name,
            guid: perm_url,
        }
    }
}

#[cfg(feature = "ssr")]
impl RssEntry {
    // Converts an RSSEntry to a String containing the rss item tags
    pub fn to_item(&self) -> String {
        format!(
            r#"
        <item>
            <title><![CDATA[{}]]></title>
            <description><![CDATA[{}]]></description>
            <pubDate>{}</pubDate>
            <link>{}</link>
            <guid isPermaLink="true">{}</guid>
            <author>{}</author>
        </item>
      "#,
            self.title, self.description, self.pub_date, self.link, self.guid, self.author
        )
    }
}
