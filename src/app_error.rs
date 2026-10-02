use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    Conflict(String),
}

// 2. Convert AppError into an HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalServerError(msg) => {

                tracing::error!("Internal server error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                dbg!(&db_err);
                if (&db_err).is_unique_violation(){
                  return  AppError::Conflict("Conflict detected".to_string())
                }
                if (&db_err).is_check_violation() {
                    return AppError::InternalServerError("Data didn't match the expected types".to_string())
                }
                AppError::InternalServerError("Internal server error".to_string())

            }
            _ => AppError::InternalServerError(err.to_string()),
        }
    }
}