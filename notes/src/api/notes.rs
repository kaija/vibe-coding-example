use crate::auth::AuthenticatedUser;
use crate::dto::{CreateNoteRequest, UpdateNoteRequest, NoteResponse, NotesListResponse, ErrorResponse};
use crate::services::NoteService;
use rocket::{get, post, put, delete, serde::json::Json, State, http::Status};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use validator::Validate;

#[post("/api/notes", data = "<request>")]
pub async fn create_note(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    request: Json<CreateNoteRequest>,
) -> Result<Json<NoteResponse>, (Status, Json<ErrorResponse>)> {
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

    match NoteService::create_note(db, user.id, request.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new("note_creation_failed", &e.to_string())),
        )),
    }
}

#[get("/api/notes")]
pub async fn get_notes(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<NotesListResponse>, (Status, Json<ErrorResponse>)> {
    match NoteService::get_user_notes(db, user.id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new("notes_fetch_failed", &e.to_string())),
        )),
    }
}

#[get("/api/notes/<note_id>")]
pub async fn get_note(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    note_id: String,
) -> Result<Json<NoteResponse>, (Status, Json<ErrorResponse>)> {
    let note_uuid = match Uuid::parse_str(&note_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse::new("invalid_uuid", "Invalid note ID format")),
            ));
        }
    };

    match NoteService::get_note_by_id(db, user.id, note_uuid).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not found") {
                Err((
                    Status::NotFound,
                    Json(ErrorResponse::new("note_not_found", &error_msg)),
                ))
            } else {
                Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new("note_fetch_failed", &error_msg)),
                ))
            }
        }
    }
}

#[put("/api/notes/<note_id>", data = "<request>")]
pub async fn update_note(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    note_id: String,
    request: Json<UpdateNoteRequest>,
) -> Result<Json<NoteResponse>, (Status, Json<ErrorResponse>)> {
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

    let note_uuid = match Uuid::parse_str(&note_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse::new("invalid_uuid", "Invalid note ID format")),
            ));
        }
    };

    match NoteService::update_note(db, user.id, note_uuid, request.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not found") {
                Err((
                    Status::NotFound,
                    Json(ErrorResponse::new("note_not_found", &error_msg)),
                ))
            } else {
                Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new("note_update_failed", &error_msg)),
                ))
            }
        }
    }
}

#[delete("/api/notes/<note_id>")]
pub async fn delete_note(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    note_id: String,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    let note_uuid = match Uuid::parse_str(&note_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse::new("invalid_uuid", "Invalid note ID format")),
            ));
        }
    };

    match NoteService::delete_note(db, user.id, note_uuid).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not found") {
                Err((
                    Status::NotFound,
                    Json(ErrorResponse::new("note_not_found", &error_msg)),
                ))
            } else {
                Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new("note_deletion_failed", &error_msg)),
                ))
            }
        }
    }
}