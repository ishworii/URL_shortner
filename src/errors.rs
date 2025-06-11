use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Validation(validator::ValidationErrors),
    Database(sqlx::Error),
    NotFound,
    Conflict,
    Unauthorized,
    PasswordHashing(argon2::password_hash::Error),
    Jwt(jsonwebtoken::errors::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::Validation(errors) => {
                let message = format!("Input validation failed. {}", errors).replace("\n", ",");
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Conflict => (
                StatusCode::CONFLICT,
                "Username or email already exists".to_string(),
            ),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid credentails".to_string()),
            AppError::Jwt(err) => {
                tracing::error!("JWT Error:{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occured".to_string(),
                )
            }
            AppError::PasswordHashing(err) => {
                tracing::error!("Password hashing error {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occured".to_string(),
                )
            }
            AppError::Database(err) => {
                tracing::error!("Database error {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal database error occured".to_string(),
                )
            }
        };
        let body = Json(json!({"error" : error_message}));
        (status_code, body).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        if let Some(db_err) = err.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::Conflict;
            }
        }
        AppError::Database(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashing(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Jwt(err)
    }
}
