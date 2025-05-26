use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Authentication DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Note DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct NotesListResponse {
    pub notes: Vec<NoteResponse>,
    pub total: usize,
}

// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}