use axum::{Json, extract::multipart::MultipartError, http::StatusCode, response::IntoResponse};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Message(StatusCode, String),
    #[error("{0}")]
    StatusCode(StatusCode),
    #[error("{0:?}")]
    MultipleMessages(StatusCode, Vec<String>),
    #[error("DB Error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Multipart Error: {0}")]
    Multipart(#[from] MultipartError),
    #[error("Error: {0}")]
    Any(#[from] Box<dyn std::error::Error>),
}

impl ApiError {
    pub fn bad_request() -> Self {
        Self::StatusCode(StatusCode::BAD_REQUEST)
    }

    pub fn internal_server_error() -> Self {
        Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn unauthorized() -> Self {
        Self::StatusCode(StatusCode::UNAUTHORIZED)
    }

    pub fn not_found() -> Self {
        Self::StatusCode(StatusCode::NOT_FOUND)
    }

    pub fn any<E>(error: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        Self::Any(Box::new(error))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Message(c, s) => (c, Json([s])).into_response(),
            ApiError::StatusCode(c) => c.into_response(),
            ApiError::MultipleMessages(c, items) => (c, Json(items)).into_response(),
            ApiError::Sqlx(error) => {
                (StatusCode::BAD_REQUEST, Json([error.to_string()])).into_response()
            }
            ApiError::Io(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json([error.to_string()])).into_response()
            }
            ApiError::Multipart(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json([error.to_string()])).into_response()
            }
            ApiError::Any(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json([error.to_string()])).into_response()
            }
        }
    }
}
