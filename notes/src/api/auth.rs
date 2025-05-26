use crate::auth::AuthenticatedUser;
use crate::dto::{RegisterRequest, LoginRequest, AuthResponse, UserResponse, ErrorResponse};
use crate::services::UserService;
use rocket::{get, post, serde::json::Json, State, http::Status};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[post("/api/register", data = "<request>")]
pub async fn register(
    db: &State<DatabaseConnection>,
    request: Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (Status, Json<ErrorResponse>)> {
    // Validate request
    if let Err(validation_errors) = request.validate() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse::new(
                "validation_error",
                &format!("Validation failed: {:?}", validation_errors),
            )),
        ));
    }

    match UserService::register(db, request.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("already exists") {
                Err((
                    Status::Conflict,
                    Json(ErrorResponse::new("user_exists", &error_msg)),
                ))
            } else {
                Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new("registration_failed", &error_msg)),
                ))
            }
        }
    }
}

#[post("/api/login", data = "<request>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    request: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (Status, Json<ErrorResponse>)> {
    // Validate request
    if let Err(validation_errors) = request.validate() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse::new(
                "validation_error",
                &format!("Validation failed: {:?}", validation_errors),
            )),
        ));
    }

    match UserService::login(db, request.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Invalid credentials") {
                Err((
                    Status::Unauthorized,
                    Json(ErrorResponse::new("invalid_credentials", &error_msg)),
                ))
            } else {
                Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new("login_failed", &error_msg)),
                ))
            }
        }
    }
}

#[get("/api/me")]
pub async fn me(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<UserResponse>, (Status, Json<ErrorResponse>)> {
    match UserService::get_user_by_id(db, user.id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new("user_fetch_failed", &e.to_string())),
        )),
    }
}