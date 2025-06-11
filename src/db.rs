use crate::models::Link;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

pub async fn create_link(
    pool: &SqlitePool,
    original_url: &str,
    short_code: &str,
) -> Result<Link, sqlx::Error> {
    let link = sqlx::query_as!(
        Link,
        r#"
        INSERT INTO links(original_url,short_code)
        VALUES ($1,$2)
        RETURNING id,original_url,short_code,created_at as "created_at:DateTime<Utc>"
        "#,
        original_url,
        short_code
    )
    .fetch_one(pool)
    .await?;
    Ok(link)
}

pub async fn find_link_by_short_code(
    pool: &SqlitePool,
    short_code: &str,
) -> Result<Option<Link>, sqlx::Error> {
    let link = sqlx::query_as!(
        Link,
        r#"
        SELECT
            id as "id!: i64",
            original_url,
            short_code,
            created_at as "created_at: DateTime<Utc>"
        FROM links
        WHERE short_code = $1
        "#,
        short_code
    )
    .fetch_optional(pool)
    .await?;

    Ok(link)
}
