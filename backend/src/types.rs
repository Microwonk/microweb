use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// TODO: add archived bool to posts, so they can easily be hidden.archived
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub markdown_content: String,
    pub released: bool,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub markdown_content: String,
}

// TODO: profile picture

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

impl User {
    /// clones self and makes a UserProfile instance
    pub fn profile(&self) -> UserProfile {
        let cloned = self.clone();
        UserProfile {
            id: cloned.id,
            name: cloned.name,
            email: cloned.email,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    // pub admin: bool,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub data: Vec<u8>,
    pub media_type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct MediaNoData {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub media_type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub author: Option<i32>,
    pub post: i32,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Default)]
pub struct CommentTreeNode {
    pub id: i32,
    pub author_name: Option<String>,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    #[sqlx(skip)]
    pub children: Vec<CommentTreeNode>,
}

impl From<Vec<CommentTreeNode>> for CommentTreeNode {
    fn from(comments: Vec<CommentTreeNode>) -> Self {
        let mut comments_by_id: HashMap<i32, CommentTreeNode> = comments
            .into_iter()
            .map(|mut comment| {
                comment.children = vec![]; // Initialize children
                (comment.id, comment)
            })
            .collect();

        // Temporary vector to store root nodes.
        let mut roots = Vec::new();

        // Build the tree
        for comment in comments_by_id.values() {
            if let Some(parent_id) = comment.replying_to {
                if let Some(parent) = comments_by_id.get_mut(&parent_id) {
                    parent.children.push(comment.clone()); // Attach child to parent
                }
            } else {
                roots.push(comment.clone()); // Collect root comments
            }
        }

        // Assuming you want to return a single root node,
        // we can return the first root. This logic can change
        // based on your use case.
        roots.into_iter().next().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewComment {
    pub content: String,
    pub replying_to: Option<i32>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IsAdminResponse {
    pub admin: bool,
}
