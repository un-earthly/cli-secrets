//! Main entry point for the Axum REST API server.

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create routes
    let app = Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/vault/push", post(vault_push))
        .route("/api/vault/pull", get(vault_pull))
        .route("/api/vault/projects", get(list_projects))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register() -> &'static str {
    "Register endpoint - Scaffolding"
}

async fn login() -> &'static str {
    "Login endpoint - Scaffolding"
}

async fn vault_push() -> &'static str {
    "Vault push endpoint - Scaffolding"
}

async fn vault_pull() -> &'static str {
    "Vault pull endpoint - Scaffolding"
}

async fn list_projects() -> &'static str {
    "List projects endpoint - Scaffolding"
}
