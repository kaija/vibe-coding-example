use crate::auth::{hash_password, verify_password, generate_jwt};
use crate::dto::{RegisterRequest, LoginRequest, AuthResponse, UserResponse};
use crate::entities::{users, Users};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use anyhow::Result;

pub struct UserService;

impl UserService {
    pub async fn register(
        db: &DatabaseConnection,
        request: RegisterRequest,
    ) -> Result<AuthResponse> {
        // Check if user already exists
        let existing_user = Users::find()
            .filter(users::Column::Username.eq(&request.username))
            .one(db)
            .await?;

        if existing_user.is_some() {
            return Err(anyhow::anyhow!("Username already exists"));
        }

        // Hash password
        let hashed_password = hash_password(&request.password)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        // Create user
        let user_id = Uuid::new_v4();
        let new_user = users::ActiveModel {
            id: Set(user_id),
            username: Set(request.username.clone()),
            password: Set(hashed_password),
            created_at: Set(chrono::Utc::now()),
        };

        let user = new_user.insert(db).await?;

        // Generate JWT token
        let token = generate_jwt(user.id, &user.username)
            .map_err(|e| anyhow::anyhow!("Failed to generate token: {}", e))?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
            },
        })
    }

    pub async fn login(
        db: &DatabaseConnection,
        request: LoginRequest,
    ) -> Result<AuthResponse> {
        // Find user by username
        let user = Users::find()
            .filter(users::Column::Username.eq(&request.username))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        // Verify password
        let is_valid = verify_password(&request.password, &user.password)
            .map_err(|e| anyhow::anyhow!("Failed to verify password: {}", e))?;

        if !is_valid {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Generate JWT token
        let token = generate_jwt(user.id, &user.username)
            .map_err(|e| anyhow::anyhow!("Failed to generate token: {}", e))?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
            },
        })
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<UserResponse> {
        let user = Users::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        })
    }
}