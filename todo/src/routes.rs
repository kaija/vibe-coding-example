use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    database::Database,
    handlers::{
        auth::{login, register},
        todo::{create_todo, delete_todo, get_todo, get_todos, update_todo},
    },
};

pub fn create_routes(db: Database) -> Router {
    Router::new()
        // Authentication routes
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        
        // Todo routes
        .route("/api/todos", post(create_todo))
        .route("/api/todos", get(get_todos))
        .route("/api/todos/:id", get(get_todo))
        .route("/api/todos/:id", put(update_todo))
        .route("/api/todos/:id", delete(delete_todo))
        
        .with_state(db)
}