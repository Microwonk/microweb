use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{admin_check, ok, ApiError, ApiResult, ServerState, User};

pub async fn get_all_users(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all users.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_all_admin_users(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE admin = true")
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all users.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
