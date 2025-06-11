use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i64,
    pub original_url: String,
    pub short_code: String,
    pub created_at: DateTime<Utc>,
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
