use crate::models::Claims;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::de::Error;
use tokio::task;

pub async fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await
    .unwrap()
}

pub async fn verify_password(
    password: String,
    hash: String,
) -> Result<bool, argon2::password_hash::Error> {
    task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    })
    .await
    .unwrap()
}

pub fn generate_jwt(user_id: i64, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    //TODO load at startup
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("Failed to create a valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expiration as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}
