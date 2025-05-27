mod auth;
mod config;
mod database;
mod error;
mod handlers;
mod models;
mod routes;

use axum::{
    http::{header, Method},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber;

use crate::{
    config::Config,
    database::Database,
    routes::create_routes,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database
    let db = Database::new(&config.database_url).await?;
    
    // Run migrations
    db.migrate().await?;

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build our application with routes
    let app = Router::new()
        .merge(create_routes(db.clone()))
        .nest_service("/", ServeDir::new("static"))
        .layer(cors);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    println!("🚀 Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}