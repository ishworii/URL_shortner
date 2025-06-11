use crate::models::{Link, User};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

pub async fn create_link(
    pool: &SqlitePool,
    original_url: &str,
    short_code: &str,
    user_id: i64,
) -> Result<Link, sqlx::Error> {
    let link = sqlx::query_as!(
        Link,
        r#"
        INSERT INTO links(original_url,short_code,user_id)
        VALUES ($1,$2,$3)
        RETURNING id as "id! : i64",original_url,short_code,user_id as "user_id:i64",created_at as "created_at:DateTime<Utc>"
        "#,
        original_url,
        short_code,
        user_id
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
            user_id as "user_id:i64",
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

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users(username,email,password_hash)
        VALUES ($1,$2,$3)
        RETURNING
            id as "id!: i64",
            username,
            email,
            password_hash,
            created_at as "created_at: DateTime<Utc>"
        "#,
        username,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}
