// main.rs for the Tariro Core Services Application

mod authorship_module_core;
mod omega_news_content_generator_llm;
mod omega_news_core_service;
mod omega_news_api;

use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

use omega_news_api::ApiState;

async fn root() -> &'static str {
    "Tariro Core Services API is running!"
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize the core Omega News service
    let omega_news_service = match omega_news_core_service::OmegaNewsCoreService::new() {
        Ok(service) => service,
        Err(e) => {
            eprintln!("Failed to initialize OmegaNewsCoreService: {}", e);
            return;
        }
    };

    let api_state = Arc::new(omega_news_api::ApiState {
        omega_news_service,
    });

    // Build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/omega-news", omega_news_api::omega_news_api_router())
        .with_state(api_state);

    // Run our application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Tariro Core Services listening on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
