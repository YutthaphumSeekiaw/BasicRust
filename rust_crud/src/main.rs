mod config;
mod database;
mod models;
mod repository;
mod service;
mod handlers;
mod status_reporter;
mod errors;

use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::AppConfig;
use database::{create_pool, run_migrations};
use repository::OrderRepository;
use service::OrderService;
use status_reporter::StatusReporter;
use handlers::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "order_crud_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env()?;
    
    tracing::info!("Starting Order CRUD API server on port {}", config.server_port);

    // Initialize database connection pool
    let pool = create_pool(&config).await?;
    
    // Run database migrations
    run_migrations(&pool).await?;

    // Initialize services
    let repository = Arc::new(OrderRepository::new(pool));
    let status_reporter = Arc::new(StatusReporter::new(
        config.status_endpoint.clone(),
        config.request_timeout,
    ));
    let service = Arc::new(OrderService::new(repository, status_reporter));

    // Setup routes
    let app = Router::new()
        .route("/api/orders", post(create_order))
        .route("/api/orders", get(get_orders))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/:id", put(update_order))
        .route("/api/orders/:id", delete(delete_order))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(service);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port)).await?;
    
    tracing::info!("Server listening on http://0.0.0.0:{}", config.server_port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}