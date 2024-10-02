use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{ok, ApiError, ApiResult, LoginRequest, LoginResponse, NewUser, ServerState, User};

pub async fn register(
    State(state): State<ServerState>,
    Json(new_user): Json<NewUser>,
) -> ApiResult<impl IntoResponse> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(new_user.email.clone())
        .fetch_one(&state.pool)
        .await;

    if user.is_ok() {
        return Err(ApiError::new(
            "User with this email already exists.",
            StatusCode::CONFLICT,
        ));
    }

    match sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, admin, passwordhash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, admin, passwordhash, created_at
        "#,
    )
    .bind(new_user.name)
    .bind(new_user.email)
    .bind(false)
    .bind(hash(new_user.password.as_str(), DEFAULT_COST).map_err(|e| {
        ApiError::werr(
            "Error hashing password.",
            StatusCode::INTERNAL_SERVER_ERROR,
            e,
        )
    })?)
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => {
            // directly also login when registering.
            login(
                State(state.clone()),
                Json(LoginRequest {
                    email: user.email,
                    password: new_user.password,
                }),
            )
            .await
        }
        Err(e) => Err(ApiError::werr(
            "Error creating/registering User.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn login(
    State(state): State<ServerState>,
    Json(login): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(login.email)
        .fetch_one(&state.pool)
        .await;

    let matches = if let Ok(ref u) = user {
        verify_password(&login.password, &u.passwordhash)
    } else {
        false
    };

    if !matches {
        return Err(ApiError::new(
            "Wrong Credentials.",
            StatusCode::UNAUTHORIZED,
        ));
    }

    let token = encode_jwt(user.unwrap().email, state.secret_key)
        .map_err(|c| ApiError::new("Error encoding jwt.", c))?;

    ok!(LoginResponse { token })
}

fn verify_password(plain: &str, hashed: &str) -> bool {
    verify(plain, hashed).unwrap_or(false)
}

pub async fn auth_guard(
    State(state): State<ServerState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(ApiError::new(
            "No Auth token found.",
            StatusCode::BAD_REQUEST,
        ))?
        .token()
        .to_owned();

    let claim = decode_jwt(token, state.secret_key)
        .map_err(|_| ApiError::new("Token expired or invalid.", StatusCode::UNAUTHORIZED))?
        .claims;

    let identity = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(claim.email)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiError::werr("Could not find User.", StatusCode::UNAUTHORIZED, e))?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

fn encode_jwt(email: String, secret: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        email,
    };

    return encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}

fn decode_jwt(jwt: String, secret: String) -> Result<TokenData<Claims>, StatusCode> {
    let res: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    res
}
