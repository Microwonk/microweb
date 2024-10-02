use axum::{
    body::Body,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[macro_export]
macro_rules! ok {
    () => {
        Ok(StatusCode::OK)
    };
    ($val:expr) => {
        Ok((StatusCode::OK, Json($val)))
    };
}

#[macro_export]
macro_rules! created {
    () => {
        Ok(StatusCode::CREATED)
    };
    ($val:expr) => {
        Ok((StatusCode::CREATED, Json($val)))
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    message: String,
    error_info: Option<String>,
    status_code: u16,
}

impl ApiError {
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status_code: status_code.as_u16(),
            error_info: None,
        }
    }

    pub fn werr(message: impl Into<String>, status_code: StatusCode, error: impl Error) -> Self {
        Self {
            message: message.into(),
            status_code: status_code.as_u16(),
            error_info: Some(error.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Serialize the ApiError to a JSON string
        let body = serde_json::to_string(&self).unwrap();

        // Build the response using the serialized JSON
        Response::builder()
            .status(self.status_code)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}
