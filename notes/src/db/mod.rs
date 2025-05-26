use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    use tracing::{info, error};
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Redact password for logging
    let safe_url = redact_db_url(&database_url);
    info!(db_url = %safe_url, "Attempting to connect to database");

    match Database::connect(&database_url).await {
        Ok(conn) => {
            info!("Successfully connected to database");
            Ok(conn)
        }
        Err(e) => {
            error!(error = %e, "Failed to connect to database");
            Err(e)
        }
    }
}

// Redact password in DB URL for logging
fn redact_db_url(url: &str) -> String {
    if let Some(idx) = url.find("://") {
        let (scheme, rest) = url.split_at(idx + 3);
        if let Some(at_idx) = rest.find('@') {
            let (userpass, host) = rest.split_at(at_idx);
            if let Some(colon_idx) = userpass.find(':') {
                let user = &userpass[..colon_idx];
                return format!("{}{}:***{}", scheme, user, host);
            }
        }
    }
    url.to_string()
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm_migration::MigratorTrait;
    use tracing::{info, error};

    info!("Running database migrations");
    match crate::migration::Migrator::up(db, None).await {
        Ok(_) => {
            info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            error!(error = %e, "Database migration failed");
            Err(e)
        }
    }
}