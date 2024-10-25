use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use bcrypt::{hash, DEFAULT_COST};

use crate::{
    admin_check, ok, ApiError, ApiResult, IsAdminResponse, NewUser, ServerState, User, UserProfile,
};

pub async fn is_admin(Extension(identity): Extension<User>) -> ApiResult<impl IntoResponse> {
    ok!(IsAdminResponse {
        admin: admin_check(&identity).is_ok()
    })
}

// because admin, okay to return full info (User)
pub async fn get_all_users(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
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

// because admin, okay to return full info (User)
pub async fn get_all_admin_users(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity)?;
    match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE admin = true ORDER BY created_at DESC",
    )
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

pub async fn get_profile(
    Extension(identity): Extension<User>,
    // State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    ok!(identity.profile())
}

pub async fn change_profile(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(user): Json<NewUser>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, UserProfile>(
        r#"
        UPDATE users
        SET email = $1, name = $2, passwordhash = $3
        WHERE id = $4
        RETURNING id, email, name
        "#,
    )
    .bind(user.email)
    .bind(user.name)
    // rehash new password
    .bind(hash(user.password.as_str(), DEFAULT_COST).map_err(|e| {
        ApiError::werr(
            "Error hashing password.",
            StatusCode::INTERNAL_SERVER_ERROR,
            e,
        )
    })?)
    .bind(identity.id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Could not update User.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
