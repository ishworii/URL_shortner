use axum::{
    Extension, Json,
    extract::{Path, State},
    response::Redirect,
};
use sqlx::SqlitePool;
use validator::Validate;

use crate::{
    db,
    errors::AppError,
    models::{
        AuthResponse, Claims, CreateLinkRequest, LinkResponse, LoginRequest, RegisterRequest,
        UserLinkResponse, UserResponse,
    },
    utils,
};

pub async fn create_short_link(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<Json<LinkResponse>, AppError> {
    payload.validate()?;
    let short_code = nanoid::nanoid!(8);
    let user_id = claims.sub;
    let new_link = db::create_link(&pool, &payload.url, &short_code, user_id).await?;
    //TODO : do not hardcode
    let response_url = format!("http://localhost:8000/{}", new_link.short_code);
    let response = LinkResponse {
        short_url: response_url,
    };
    Ok(Json(response))
}

pub async fn redirect_to_original(
    State(pool): State<SqlitePool>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    let link_record = db::find_link_by_short_code(&pool, &short_code).await?;
    if let Some(link) = link_record {
        Ok(Redirect::permanent(&link.original_url))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;
    let hashed_password = utils::hash_password(payload.password).await?;
    let new_user =
        db::create_user(&pool, &payload.username, &payload.email, &hashed_password).await?;
    let response = UserResponse {
        id: new_user.id,
        username: new_user.username,
        email: new_user.email,
    };
    Ok(Json(response))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate()?;
    let user = db::find_user_by_email(&pool, &payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let password_verified = utils::verify_password(payload.password, user.password_hash).await?;
    if !password_verified {
        return Err(AppError::Unauthorized);
    }
    let token = utils::generate_jwt(user.id, &user.username)?;
    let response = AuthResponse { token };
    Ok(Json(response))
}

pub async fn get_user_links(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserLinkResponse>>, AppError> {
    let user_id = claims.sub;
    let db_links = db::find_links_by_user_id(&pool, user_id).await?;
    let response_links: Vec<UserLinkResponse> = db_links
        .into_iter()
        .map(|db_link| UserLinkResponse {
            original_url: db_link.original_url,
            short_code: db_link.short_code,
            created_at: db_link.created_at,
        })
        .collect();
    Ok(Json(response_links))
}
