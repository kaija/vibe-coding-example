mod api;
mod auth;
mod db;
mod dto;
mod entities;
mod migration;
mod services;

use dotenv::dotenv;
use rocket::{launch, routes, Build, Rocket, Request, Data, fairing::{Fairing, Info, Kind}, http::Status};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use tracing::{info, error};
use tracing_subscriber;
use std::env;

#[launch]
async fn rocket() -> Rocket<Build> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    info!("Starting Note API backend (Rocket + SeaORM)");

    // Load environment variables
    dotenv().ok();
    info!("Loaded environment variables from .env");

    // Validate required environment variables
    let required_vars = ["DATABASE_URL"];
    let mut missing_vars = vec![];
    for var in &required_vars {
        if env::var(var).is_err() {
            missing_vars.push(*var);
        }
    }
    if !missing_vars.is_empty() {
        error!(?missing_vars, "Missing required environment variables");
        std::process::exit(1);
    }

    // Establish database connection
    let db = match db::establish_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            error!(error = %e, "Failed to connect to database");
            std::process::exit(1);
        }
    };

    // Run migrations
    if let Err(e) = db::run_migrations(&db).await {
        error!(error = %e, "Failed to run migrations");
        std::process::exit(1);
    }

    // Configure CORS
    let cors = match CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
        .allowed_headers(rocket_cors::AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
    {
        Ok(cors) => cors,
        Err(e) => {
            error!(error = ?e, "Failed to create CORS configuration");
            std::process::exit(1);
        }
    };

    // Log Rocket config
    let port = rocket::Config::figment().extract_inner::<u16>("port").unwrap_or(8000);
    info!(port, "Rocket will bind to port");

    // Build and configure Rocket
    rocket::build()
        .manage(db)
        .attach(cors)
        .attach(RequestLogger)
        .mount("/", api::get_routes())
        .mount("/", routes![health_check, status])
}


#[rocket::get("/health")]
async fn health_check(db: &State<DatabaseConnection>) -> (Status, &'static str) {
    use sea_orm::ConnectionTrait;
    match db.ping().await {
        Ok(_) => (Status::Ok, "OK"),
        Err(_) => (Status::ServiceUnavailable, "DB Unavailable"),
    }
}

#[rocket::get("/status")]
fn status() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app": "note-api",
        "version": env!("CARGO_PKG_VERSION"),
        "build": option_env!("BUILD_TIME").unwrap_or("unknown"),
        "status": "running"
    }))
}

// Rocket fairing for request/response logging
pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request/Response Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        info!(
            method = %request.method(),
            uri = %request.uri(),
            remote = ?request.client_ip(),
            "Incoming request"
        );
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut rocket::Response<'r>) {
        info!(
            method = %request.method(),
            uri = %request.uri(),
            status = %response.status(),
            "Response sent"
        );
    }
}
