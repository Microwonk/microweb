pub mod api_error;
pub mod auth_handler;
pub mod get_handlers;
pub mod post_handlers;
pub mod types;
pub mod upload_handler;

pub use {api_error::*, types::*};

#[derive(Clone)]
pub struct ServerState {
    pub pool: sqlx::PgPool,
    pub secret_key: String,
}
