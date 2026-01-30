//! Note CRUD commands

use std::fs;
use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::db::{notes as db_notes, tags::get_note_tags};
use crate::error::ChronicleError;
use crate::models::Note;
use crate::vault::Indexer;

/// List all notes
#[tauri::command]
pub async fn list_notes(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<db_notes::NoteMeta>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();

    let notes = db_notes::list_notes(&conn)?;
    Ok(notes)
}

/// Get a single note with content
#[tauri::command]
pub async fn get_note(
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Note, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;

    let full_path = vault_path.join(&path);
    let content = fs::read_to_string(&full_path)?;

    let conn = db.conn();
    let meta = db_notes::get_note_by_path(&conn, &path)?
        .ok_or_else(|| ChronicleError::NoteNotFound(path.clone()))?;

    let tags = get_note_tags(&conn, meta.id)?;

    Ok(Note {
        path: meta.path,
        title: meta.title,
        content,
        word_count: meta.word_count,
        created_at: meta.created_at,
        modified_at: meta.modified_at,
        tags,
    })
}

/// Create a new note
#[tauri::command]
pub async fn create_note(
    title: String,
    content: Option<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;

    // Generate filename from title
    let filename = sanitize_filename(&title) + ".md";
    let full_path = vault_path.join(&filename);

    if full_path.exists() {
        return Err(ChronicleError::NoteExists(filename));
    }

    // Create content with title heading
    let note_content = content.unwrap_or_else(|| format!("# {}\n\n", title));
    fs::write(&full_path, &note_content)?;

    // Index the new note
    let indexer = Indexer::new(vault_path.clone())?;
    indexer.index_file(db, &full_path)?;

    let conn = db.conn();
    let meta = db_notes::get_note_by_path(&conn, &filename)?
        .ok_or(ChronicleError::NoteNotFound(filename))?;

    Ok(meta)
}

/// Save note content
#[tauri::command]
pub async fn save_note(
    path: String,
    content: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;

    let full_path = vault_path.join(&path);
    if !full_path.exists() {
        return Err(ChronicleError::NoteNotFound(path));
    }

    fs::write(&full_path, &content)?;

    // Re-index the note
    let indexer = Indexer::new(vault_path.clone())?;
    indexer.index_file(db, &full_path)?;

    let conn = db.conn();
    let meta = db_notes::get_note_by_path(&conn, &path)?
        .ok_or(ChronicleError::NoteNotFound(path))?;

    Ok(meta)
}

/// Delete a note
#[tauri::command]
pub async fn delete_note(
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;

    let full_path = vault_path.join(&path);

    // Remove from index first
    let indexer = Indexer::new(vault_path.clone())?;
    indexer.remove_file(db, &full_path)?;

    // Delete file
    if full_path.exists() {
        fs::remove_file(&full_path)?;
    }

    Ok(())
}

/// Rename a note
#[tauri::command]
pub async fn rename_note(
    old_path: String,
    new_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;

    let old_full = vault_path.join(&old_path);
    let new_full = vault_path.join(&new_path);

    if !old_full.exists() {
        return Err(ChronicleError::NoteNotFound(old_path));
    }

    if new_full.exists() {
        return Err(ChronicleError::NoteExists(new_path));
    }

    // Rename file
    fs::rename(&old_full, &new_full)?;

    // Update index
    let conn = db.conn();
    db_notes::rename_note(&conn, &old_path, &new_path)?;

    let meta = db_notes::get_note_by_path(&conn, &new_path)?
        .ok_or(ChronicleError::NoteNotFound(new_path))?;

    Ok(meta)
}

/// Sanitize a string for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim()
        .replace(' ', "-")
        .to_lowercase()
}
