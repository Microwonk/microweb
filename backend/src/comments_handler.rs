use std::collections::HashMap;

use axum::*;
use extract::{Path, State};
use http::StatusCode;
use response::IntoResponse;

use crate::{
    created, ok, ApiError, ApiResult, Comment, CommentTreeNode, NewComment, ServerState, User,
};

pub async fn create_comment(
    Path(post_id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(comment): Json<NewComment>,
) -> ApiResult<impl IntoResponse> {
    let result = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (
            author, post, content, replying_to
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING id, author, post, content, replying_to, created_at
        "#,
    )
    .bind(identity.id)
    .bind(post_id)
    .bind(comment.content)
    .bind(comment.replying_to)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(comment) => created!(comment),
        Err(e) => Err(ApiError::werr(
            "Error creating comment.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_comments_of_post(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE post = $1")
        .bind(post_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all comments on post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_comments_of_post_tree(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, CommentTreeNode>(
        r#"
        SELECT 
            comments.id,
            users.name AS author_name,
            comments.content,
            comments.replying_to,
            comments.created_at
        FROM comments
        JOIN posts ON comments.post = posts.id
        LEFT JOIN users ON comments.author = users.id
        WHERE comments.post = $1
        ORDER BY comments.created_at
        "#,
    )
    .bind(post_id)
    .fetch_all(&state.pool)
    .await
    {
        Ok(response) => {
            ok!(build_comment_tree(response))
        }
        Err(e) => Err(ApiError::werr(
            "Error retrieving all comments on post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

fn build_comment_tree(comments: Vec<CommentTreeNode>) -> Vec<CommentTreeNode> {
    let mut comments_by_id: HashMap<i32, CommentTreeNode> = comments
        .into_iter()
        .map(|mut comment| {
            comment.children = vec![]; // Initialize children
            (comment.id, comment)
        })
        .collect();

    // Temporary vector to store root nodes.
    let mut roots = Vec::new();

    let comments = comments_by_id.clone();
    let comments = comments.values();
    // Insert each child into its parent's children vector.
    for comment in comments {
        if let Some(parent_id) = comment.replying_to {
            {
                if let Some(parent) = comments_by_id.get_mut(&parent_id) {
                    parent.children.push(comment.clone()); // Attach child to parent
                }
            }
        } else {
            roots.push(comment.clone()); // Collect root comments
        }
    }

    roots
}
