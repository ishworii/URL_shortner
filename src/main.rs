mod auth;
mod db;
mod errors;
mod models;
mod routes;
mod utils;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use sqlx::SqlitePool;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "linklair=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Logger initialized");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE URL must be set");

    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection pool created successfully");

    let protected_routes = Router::new()
        .route(
            "/api/links",
            post(routes::create_short_link).get(routes::get_user_links),
        )
        .layer(middleware::from_fn_with_state(db_pool.clone(), auth::auth));

    let public_routes = Router::new()
        .route("/api/auth/register", post(routes::register))
        .route("/api/auth/login", post(routes::login))
        .route("/{short_code}", get(routes::redirect_to_original));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(db_pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
