use axum::*;
use extract::State;
use http::StatusCode;
use response::IntoResponse;

use crate::{admin_check, ok, ApiError, ApiResult, LogEntry, NewLogEntry, ServerState, User};

pub enum Log {
    Info(String),
    Error(String),
    Warn(String),
    Notice(String),
    Debug(String),
}

impl Log {
    pub fn to_new_entry(&self) -> NewLogEntry {
        let (message, level) = match self {
            Log::Info(msg) => (msg, "info"),
            Log::Error(msg) => (msg, "error"),
            Log::Warn(msg) => (msg, "warn"),
            Log::Notice(msg) => (msg, "notice"),
            Log::Debug(msg) => (msg, "debug"),
        };
        NewLogEntry::new(message.clone(), level.into())
    }

    pub async fn info(message: String, state: &ServerState) -> ApiResult<()> {
        log(Self::Info(message), state).await
    }

    pub async fn error(message: String, state: &ServerState) -> ApiResult<()> {
        log(Self::Error(message), state).await
    }

    pub async fn warn(message: String, state: &ServerState) -> ApiResult<()> {
        log(Self::Warn(message), state).await
    }

    pub async fn notice(message: String, state: &ServerState) -> ApiResult<()> {
        log(Self::Notice(message), state).await
    }

    pub async fn debug(message: String, state: &ServerState) -> ApiResult<()> {
        log(Self::Debug(message), state).await
    }
}

pub async fn log(log: Log, state: &ServerState) -> ApiResult<()> {
    let entry = log.to_new_entry();
    match sqlx::query(
        r#"
        INSERT INTO logs (
            message, context
        )
        VALUES (
            $1, $2
        )
        "#,
    )
    .bind(entry.message)
    .bind(entry.context)
    .execute(&state.pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::werr(
            "Error creating post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_logs(
    State(state): State<ServerState>,
    Extension(identity): Extension<User>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
    match sqlx::query_as::<_, LogEntry>("SELECT * FROM logs ORDER BY log_time DESC")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all logs.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
