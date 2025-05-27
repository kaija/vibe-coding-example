use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    RequestPartsExt,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use uuid::Uuid;

use crate::{
    database::Database,
    error::AppError,
    models::{Claims, User},
};

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Auth(format!("Failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))
    }
}

pub struct AuthenticatedUser {
    pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Database: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // Check if it starts with "Bearer "
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        // Verify the token
        let jwt_service = JwtService::new();
        let claims = jwt_service.verify_token(token)?;

        // Parse user ID from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

        // Get database from state
        let db = Database::from_ref(state);

        // Fetch user from database
        let user = db
            .get_user_by_id(user_id)
            .await?
            .ok_or(AppError::Auth("User not found".to_string()))?;

        Ok(AuthenticatedUser { user })
    }
}

// Helper trait to extract Database from state
pub trait FromRef<T> {
    fn from_ref(input: &T) -> Self;
}

impl FromRef<Database> for Database {
    fn from_ref(input: &Database) -> Self {
        input.clone()
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Internal(format!("Failed to verify password: {}", e)))
}