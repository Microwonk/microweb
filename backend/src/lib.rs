pub mod api_response;
pub mod auth_handler;
pub mod comments_handler;
pub mod media_handler;
pub mod post_handler;
pub mod types;
pub mod user_handler;

use axum::http::StatusCode;
pub use {api_response::*, types::*};

#[derive(Clone)]
pub struct ServerState {
    pub pool: sqlx::PgPool,
    pub secret_key: String,
}

pub fn admin_check(identity: &User) -> Result<(), ApiError> {
    if !identity.admin {
        Err(ApiError::new(
            "User is not Admin.",
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(())
    }
}
