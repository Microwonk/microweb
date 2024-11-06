use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use bcrypt::{hash, DEFAULT_COST};

use crate::{
    admin_check, logs_handler::Log, ok, ApiError, ApiResult, IsAdminResponse, NewUser, ServerState,
    User, UserProfile, UserUpdate,
};

pub async fn is_admin(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    ok!(IsAdminResponse {
        admin: admin_check(&identity, &state).await.is_ok()
    })
}

// because admin, okay to return full info (User)
pub async fn get_all_users(
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
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
    admin_check(&identity, &state).await?;
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
        Ok(user) => {
            Log::info(
                format!(
                    "User Profile [{} | {}] with email {} changed by User.",
                    user.id, user.name, user.email
                ),
                &state,
            )
            .await?;
            ok!(user)
        }
        Err(e) => Err(ApiError::werr(
            "Could not update User.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => {
            Log::info(format!("User with id {} deleted.", id), &state).await?;
            ok!()
        }
        Err(e) => Err(ApiError::werr(
            "Could not delete User.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn update_user(
    Path(id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(user): Json<UserUpdate>,
) -> ApiResult<impl IntoResponse> {
    admin_check(&identity, &state).await?;
    match sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = $1, email = $2
        WHERE id = $3
        RETURNING id, name, email, admin, passwordhash, created_at
        "#,
    )
    .bind(user.name)
    .bind(user.email)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => {
            Log::info(
                format!(
                    "User [{} | {}] with email {} updated.",
                    user.id, user.name, user.email
                ),
                &state,
            )
            .await?;
            ok!(user)
        }
        Err(e) => Err(ApiError::werr(
            "Could not update User.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}
