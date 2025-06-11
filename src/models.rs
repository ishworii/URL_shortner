use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i64,
    pub original_url: String,
    pub short_code: String,
    pub created_at: DateTime<Utc>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLinkRequest {
    #[validate(url(message = "must be a valid URL"))]
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub short_url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "must be a valid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}
