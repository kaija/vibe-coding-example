use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{error::AppError, models::{User, Todo, CreateTodo, UpdateTodo}};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<(), AppError> {
        // In a real application, you'd use sqlx-cli for migrations
        // For now, we'll assume the init.sql handles the schema
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at, updated_at
            "#
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.try_get("id").unwrap(),
            username: r.try_get("username").unwrap(),
            email: r.try_get("email").unwrap(),
            password_hash: r.try_get("password_hash").unwrap(),
            created_at: r.try_get("created_at").unwrap(),
            updated_at: r.try_get("updated_at").unwrap(),
        }))
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.try_get("id").unwrap(),
            username: r.try_get("username").unwrap(),
            email: r.try_get("email").unwrap(),
            password_hash: r.try_get("password_hash").unwrap(),
            created_at: r.try_get("created_at").unwrap(),
            updated_at: r.try_get("updated_at").unwrap(),
        }))
    }

    // Todo operations
    pub async fn create_todo(&self, user_id: Uuid, todo: CreateTodo) -> Result<Todo, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO todos (user_id, title, description, completed, scheduled_for)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, title, description, completed, scheduled_for, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed.unwrap_or(false))
        .bind(&todo.scheduled_for)
        .fetch_one(&self.pool)
        .await?;

        Ok(Todo {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            completed: row.try_get("completed")?,
            scheduled_for: row.try_get("scheduled_for")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub async fn get_todos_by_user(&self, user_id: Uuid) -> Result<Vec<Todo>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, title, description, completed, scheduled_for, created_at, updated_at
            FROM todos
            WHERE user_id = $1
            ORDER BY
                CASE WHEN scheduled_for IS NULL THEN 1 ELSE 0 END,
                scheduled_for ASC,
                created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Todo {
            id: r.try_get("id").unwrap(),
            user_id: r.try_get("user_id").unwrap(),
            title: r.try_get("title").unwrap(),
            description: r.try_get("description").unwrap(),
            completed: r.try_get("completed").unwrap(),
            scheduled_for: r.try_get("scheduled_for").unwrap(),
            created_at: r.try_get("created_at").unwrap(),
            updated_at: r.try_get("updated_at").unwrap(),
        }).collect())
    }

    pub async fn get_todo_by_id(&self, todo_id: Uuid, user_id: Uuid) -> Result<Option<Todo>, AppError> {
        let row = sqlx::query(
            "SELECT id, user_id, title, description, completed, scheduled_for, created_at, updated_at FROM todos WHERE id = $1 AND user_id = $2"
        )
        .bind(todo_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Todo {
            id: r.try_get("id").unwrap(),
            user_id: r.try_get("user_id").unwrap(),
            title: r.try_get("title").unwrap(),
            description: r.try_get("description").unwrap(),
            completed: r.try_get("completed").unwrap(),
            scheduled_for: r.try_get("scheduled_for").unwrap(),
            created_at: r.try_get("created_at").unwrap(),
            updated_at: r.try_get("updated_at").unwrap(),
        }))
    }

    pub async fn update_todo(&self, todo_id: Uuid, user_id: Uuid, update: UpdateTodo) -> Result<Option<Todo>, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE todos
            SET title = COALESCE($3, title),
                description = COALESCE($4, description),
                completed = COALESCE($5, completed),
                scheduled_for = COALESCE($6, scheduled_for),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, title, description, completed, scheduled_for, created_at, updated_at
            "#,
        )
        .bind(todo_id)
        .bind(user_id)
        .bind(&update.title)
        .bind(&update.description)
        .bind(update.completed)
        .bind(&update.scheduled_for)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Todo {
            id: r.try_get("id").unwrap(),
            user_id: r.try_get("user_id").unwrap(),
            title: r.try_get("title").unwrap(),
            description: r.try_get("description").unwrap(),
            completed: r.try_get("completed").unwrap(),
            scheduled_for: r.try_get("scheduled_for").unwrap(),
            created_at: r.try_get("created_at").unwrap(),
            updated_at: r.try_get("updated_at").unwrap(),
        }))
    }

    pub async fn delete_todo(&self, todo_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM todos WHERE id = $1 AND user_id = $2"
        )
        .bind(todo_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}