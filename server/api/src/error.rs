use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum RestError {
    NotFound(String),
    Invalid(String),
    Internal(anyhow::Error),
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        tracing::error!("rest error: {:?}", self);

        let (status, error_message) = match self {
            RestError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            RestError::Invalid(msg) => (StatusCode::BAD_REQUEST, msg),
            RestError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for RestError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.into())
    }
}

impl From<serde_json::Error> for RestError {
    fn from(err: serde_json::Error) -> Self {
        Self::Invalid(err.to_string())
    }
}

impl From<sea_orm::DbErr> for RestError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Internal(err.into())
    }
}
