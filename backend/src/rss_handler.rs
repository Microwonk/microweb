use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{ApiError, ApiResult, ProcessedPost, RssEntry, ServerState};

pub fn generate_rss(title: &str, description: &str, link: &str, posts: &[ProcessedPost]) -> String {
    // Let's generate all those XML tags for Posts and collect them into a single string
    let rss_entries = posts
        .iter()
        .cloned()
        .map(|p| p.into())
        .map(|r: RssEntry| r.to_item())
        .collect::<String>();

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{title}</title>
        <description>{description}</description>
        <link>{link}</link>
        <language>en-us</language>
        <ttl>60</ttl>
        <atom:link href="https://blog.nicolas-frey.com/rss" rel="self" type="application/rss+xml" />
        {}
    </channel>
</rss>   
     "#,
        rss_entries
    )
}

pub async fn rss(State(state): State<ServerState>) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, ProcessedPost>(
        r#"SELECT 
            posts.id,
            users.name AS author_name,
            posts.author AS author,
            posts.description,
            posts.title,
            posts.slug,
            posts.markdown_content,
            posts.released,
            posts.created_at,
            posts.updated_at
        FROM posts
        JOIN users ON posts.author = users.id
        WHERE released = true ORDER BY created_at DESC"#,
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(response) => Ok((
            StatusCode::OK,
            generate_rss(
                "Microwonk's Blog",
                "Ramblings of an Aspiring (Game) Developer",
                "https://blog.nicolas-frey.com",
                &response,
            ),
        )),
        Err(e) => Err(ApiError::werr(
            "Error retrieving/generating RSS feed.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
