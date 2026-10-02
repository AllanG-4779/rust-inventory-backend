use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsePayload<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    timestamp: String,
    #[serde(rename = "success")]
    status: bool,
    message: String,
    error_info: Option<T>,
    trace_id: Uuid,
}

impl<T> ResponsePayload<T> {
    pub fn new(
        data: Option<T>,
        status: bool,
        message: String,
        error_info: Option<T>,
        timestamp: String,
    ) -> ResponsePayload<T> {
        Self {
            timestamp,
            data,
            status,
            message,
            error_info,
            trace_id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub info: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    ValidationError(ErrorInfo),
    NotFound(String),
    InternalServerError(String),
    Conflict(String),
}

// 2. Convert AppError into an HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, info) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.message, msg.info),
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred".to_string(),
                    None,
                )
            }
        };

        let body = Json(ResponsePayload::<HashMap<String, String>>::new(
            None,
            false,
            error_message,
            info,
            chrono::Utc::now().to_string(),
        ));
        (status, body).into_response()
    }
}
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                dbg!(&db_err);
                if (&db_err).is_unique_violation() {
                    return AppError::Conflict("Conflict detected".to_string());
                }
                if (&db_err).is_check_violation() {
                    return AppError::InternalServerError(
                        "Data didn't match the expected types".to_string(),
                    );
                }
                AppError::InternalServerError("Internal server error".to_string())
            }
            _ => AppError::InternalServerError(err.to_string()),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        let fields = e
            .field_errors()
            .into_iter()
            .filter_map(|(field, errors)| {
                let err = errors.first()?;
                let message = err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| err.code.to_string());
                Some((field.to_string(), message))
            })
            .collect();

        AppError::ValidationError(ErrorInfo {
            message: "Validation failed".to_string(),
            info: Some(fields),
        })
    }
}
