//! Tag commands

use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::db::{notes::get_note_by_id, tags::{list_tags as db_list_tags, get_notes_by_tag as db_get_notes_by_tag, TagInfo}};
use crate::db::notes::NoteMeta;
use crate::error::ChronicleError;

/// List all tags
#[tauri::command]
pub async fn list_tags(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<TagInfo>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();
    
    let tags = db_list_tags(&conn)?;
    Ok(tags)
}

/// Get notes with a specific tag
#[tauri::command]
pub async fn get_notes_by_tag(
    tag: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<NoteMeta>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();
    
    let note_ids = db_get_notes_by_tag(&conn, &tag)?;
    let mut notes = Vec::new();
    
    for id in note_ids {
        if let Some(note) = get_note_by_id(&conn, id)? {
            notes.push(note);
        }
    }
    
    Ok(notes)
}
