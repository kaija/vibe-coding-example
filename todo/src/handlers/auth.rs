use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::{
    auth::{hash_password, verify_password, JwtService},
    database::Database,
    error::{AppError, Result},
    models::{AuthResponse, CreateUser, LoginRequest, UserResponse},
};

pub async fn register(
    State(db): State<Database>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    // Check if user already exists
    if db.get_user_by_username(&payload.username).await?.is_some() {
        return Err(AppError::BadRequest("Username already exists".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = db
        .create_user(&payload.username, &payload.email, &password_hash)
        .await?;

    // Generate JWT token
    let jwt_service = JwtService::new();
    let token = jwt_service.generate_token(&user)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(db): State<Database>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    // Get user by username
    let user = db
        .get_user_by_username(&payload.username)
        .await?
        .ok_or(AppError::Auth("Invalid credentials".to_string()))?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Auth("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let jwt_service = JwtService::new();
    let token = jwt_service.generate_token(&user)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(Json(response))
}