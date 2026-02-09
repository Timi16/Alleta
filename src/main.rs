mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

use anyhow::Result;
use axum::http::{header, Method};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use config::Config;
use db::PrismaClient;

#[derive(Clone)]
pub struct AppState {
    pub db: PrismaClient,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Aletta Backend...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded");
    info!("Server will run on port: {}", config.port);

    // Create database client
    info!("Connecting to database...");
    let db = db::create_client(&config.database_url).await?;
    info!("Database connected successfully");

    // Create app state
    let state = AppState {
        db,
        config: Arc::new(config.clone()),
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Create router
    let app = routes::create_routes(state).layer(cors);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("🚀 Aletta Backend is running on http://{}", addr);
    info!("📊 Health check: http://{}/health", addr);
    info!("🔍 Analyze endpoint: POST http://{}/api/analyze", addr);
    info!("📄 Report endpoint: GET http://{}/api/report/:id", addr);

    axum::serve(listener, app).await?;

    Ok(())
}