pub mod api_response;
pub mod auth_handler;
pub mod comments_handler;
pub mod logs_handler;
pub mod media_handler;
pub mod post_handler;
pub mod rss_handler;
pub mod types;
pub mod user_handler;

use axum::http::StatusCode;
use logs_handler::Log;
pub use {api_response::*, types::*};

#[derive(Clone)]
pub struct ServerState {
    pub pool: sqlx::PgPool,
    pub secret_key: String,
}

pub async fn admin_check(identity: &User, state: &ServerState) -> ApiResult<()> {
    if !identity.admin {
        Log::notice(
            format!(
                "User [{} | {}] with email {} tried an authenticated route as non-admin.",
                identity.id, identity.name, identity.email
            ),
            state,
        )
        .await?;
        Err(ApiError::new(
            "User is not Admin.",
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(())
    }
}
