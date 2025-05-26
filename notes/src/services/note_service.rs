use crate::dto::{CreateNoteRequest, UpdateNoteRequest, NoteResponse, NotesListResponse};
use crate::entities::{notes, Notes};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use anyhow::Result;

pub struct NoteService;

impl NoteService {
    pub async fn create_note(
        db: &DatabaseConnection,
        user_id: Uuid,
        request: CreateNoteRequest,
    ) -> Result<NoteResponse> {
        let note_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let new_note = notes::ActiveModel {
            id: Set(note_id),
            user_id: Set(user_id),
            title: Set(request.title),
            content: Set(request.content),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let note = new_note.insert(db).await?;

        Ok(NoteResponse {
            id: note.id,
            user_id: note.user_id,
            title: note.title,
            content: note.content,
            created_at: note.created_at,
            updated_at: note.updated_at,
        })
    }

    pub async fn get_user_notes(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<NotesListResponse> {
        let notes = Notes::find()
            .filter(notes::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        let note_responses: Vec<NoteResponse> = notes
            .into_iter()
            .map(|note| NoteResponse {
                id: note.id,
                user_id: note.user_id,
                title: note.title,
                content: note.content,
                created_at: note.created_at,
                updated_at: note.updated_at,
            })
            .collect();

        let total = note_responses.len();

        Ok(NotesListResponse {
            notes: note_responses,
            total,
        })
    }

    pub async fn get_note_by_id(
        db: &DatabaseConnection,
        user_id: Uuid,
        note_id: Uuid,
    ) -> Result<NoteResponse> {
        let note = Notes::find_by_id(note_id)
            .filter(notes::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Note not found"))?;

        Ok(NoteResponse {
            id: note.id,
            user_id: note.user_id,
            title: note.title,
            content: note.content,
            created_at: note.created_at,
            updated_at: note.updated_at,
        })
    }

    pub async fn update_note(
        db: &DatabaseConnection,
        user_id: Uuid,
        note_id: Uuid,
        request: UpdateNoteRequest,
    ) -> Result<NoteResponse> {
        let note = Notes::find_by_id(note_id)
            .filter(notes::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Note not found"))?;

        let mut active_note: notes::ActiveModel = note.into();

        if let Some(title) = request.title {
            active_note.title = Set(title);
        }

        if let Some(content) = request.content {
            active_note.content = Set(content);
        }

        active_note.updated_at = Set(chrono::Utc::now());

        let updated_note = active_note.update(db).await?;

        Ok(NoteResponse {
            id: updated_note.id,
            user_id: updated_note.user_id,
            title: updated_note.title,
            content: updated_note.content,
            created_at: updated_note.created_at,
            updated_at: updated_note.updated_at,
        })
    }

    pub async fn delete_note(
        db: &DatabaseConnection,
        user_id: Uuid,
        note_id: Uuid,
    ) -> Result<()> {
        let note = Notes::find_by_id(note_id)
            .filter(notes::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Note not found"))?;

        let active_note: notes::ActiveModel = note.into();
        active_note.delete(db).await?;

        Ok(())
    }
}