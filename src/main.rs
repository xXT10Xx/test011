mod auth;
mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod utils;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    database::Database,
    handlers::{auth as auth_handlers, health, users as user_handlers},
    middleware::auth::auth_middleware,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_advanced_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let database = Database::new(&config.database_url).await?;

    let app = create_app(database).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app(database: Database) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/auth/register", post(auth_handlers::register))
        .route("/api/auth/login", post(auth_handlers::login))
        .route("/api/users", get(user_handlers::list_users))
        .route("/api/users/:id", get(user_handlers::get_user))
        .route("/api/users/:id", post(user_handlers::update_user))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(axum::middleware::from_fn_with_state(
                    database.clone(),
                    auth_middleware,
                )),
        )
        .with_state(database)
}