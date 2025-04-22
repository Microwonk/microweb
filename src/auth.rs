pub mod app;
mod login;
mod register;

use leptos::Params;
use leptos_router::params::Params;
#[cfg(feature = "ssr")]
pub use ssr::*;

#[derive(Params, PartialEq)]
pub struct ReturnUrlQuery {
    return_url: String,
}

#[cfg(feature = "ssr")]
mod ssr {
    use axum::{
        extract::Request,
        http::{header, HeaderValue, StatusCode},
        middleware::Next,
        response::Response,
    };
    use axum_extra::headers::{Cookie, HeaderMapExt};
    use bcrypt::verify;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
    use serde::{Deserialize, Serialize};

    use crate::models::User;

    pub const EXPIRATION_DAYS: i64 = 30;

    #[tracing::instrument]
    pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, StatusCode> {
        let Some(token) = req.headers().typed_get::<Cookie>() else {
            return Ok(next.run(req).await);
        };

        let Some(token) = token.get("auth_token") else {
            return Ok(next.run(req).await);
        };

        let token = token.to_owned();

        let Ok(claim) = decode_jwt(token) else {
            let mut response = next.run(req).await;

            response.headers_mut().append(
                header::SET_COOKIE,
                HeaderValue::from_str(
                    &format!("auth_token=deleted; Domain={}; Path=/; SameSite=Lax; Secure; Expires=Thu, 01 Jan 1970 00:00:00 GMT;", *crate::DOMAIN),
                )
                .unwrap(),
            );

            return Ok(response);
        };

        let claim = claim.claims;

        let identity = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(claim.email)
            .fetch_one(crate::database::db())
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        req.extensions_mut().insert(identity);

        Ok(next.run(req).await)
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Claims {
        pub exp: usize,
        pub iat: usize,
        pub email: String,
    }

    pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
        let now = Utc::now();
        let expire = Duration::days(EXPIRATION_DAYS);

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
}
