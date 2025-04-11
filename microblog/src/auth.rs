use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::headers::{Cookie, HeaderMapExt};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{database, models::User};

pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let Some(token) = req.headers().typed_get::<Cookie>() else {
        return Ok(next.run(req).await);
    };

    let Some(token) = token.get("auth_token") else {
        return Ok(next.run(req).await);
    };

    let token = token.to_owned();

    let claim = decode_jwt(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims;

    let identity = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(claim.email)
        .fetch_one(database::db())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::days(1);

    let claim = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        email,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(
            std::env::var("SECRET_KEY")
                .expect("no secret key specified")
                .as_ref(),
        ),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(plain: &str, hashed: &str) -> bool {
    verify(plain, hashed).unwrap_or(false)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let res: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(
            std::env::var("SECRET_KEY")
                .expect("no secret key specified")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    res
}
