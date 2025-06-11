use crate::{errors::AppError, models::Claims};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use sqlx::SqlitePool;

pub async fn auth(
    State(_pool): State<SqlitePool>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let claims = decode_jwt(auth_header.token())?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn decode_jwt(token: &str) -> Result<Claims, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::default();
    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_err| AppError::Unauthorized)
}
