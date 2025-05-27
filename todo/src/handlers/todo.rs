use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    database::Database,
    error::{AppError, Result},
    models::{CreateTodo, TodoResponse, UpdateTodo},
};

pub async fn create_todo(
    State(db): State<Database>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<TodoResponse>)> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    // Validate scheduled_for is not in the past (optional validation)
    if let Some(scheduled_for) = payload.scheduled_for {
        let now = Utc::now();
        if scheduled_for < now {
            return Err(AppError::Validation(
                "Scheduled date cannot be in the past".to_string()
            ));
        }
    }

    // Create todo
    let todo = db.create_todo(user.user.id, payload).await?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

pub async fn get_todos(
    State(db): State<Database>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<TodoResponse>>> {
    let todos = db.get_todos_by_user(user.user.id).await?;
    let todo_responses: Vec<TodoResponse> = todos.into_iter().map(TodoResponse::from).collect();
    
    Ok(Json(todo_responses))
}

pub async fn get_todo(
    State(db): State<Database>,
    user: AuthenticatedUser,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoResponse>> {
    let todo = db
        .get_todo_by_id(todo_id, user.user.id)
        .await?
        .ok_or(AppError::NotFound("Todo not found".to_string()))?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn update_todo(
    State(db): State<Database>,
    user: AuthenticatedUser,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<TodoResponse>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    // Validate scheduled_for is not in the past (optional validation)
    if let Some(scheduled_for) = payload.scheduled_for {
        let now = Utc::now();
        if scheduled_for < now {
            return Err(AppError::Validation(
                "Scheduled date cannot be in the past".to_string()
            ));
        }
    }

    // Update todo
    let todo = db
        .update_todo(todo_id, user.user.id, payload)
        .await?
        .ok_or(AppError::NotFound("Todo not found".to_string()))?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn delete_todo(
    State(db): State<Database>,
    user: AuthenticatedUser,
    Path(todo_id): Path<Uuid>,
) -> Result<StatusCode> {
    let deleted = db.delete_todo(todo_id, user.user.id).await?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Todo not found".to_string()))
    }
}