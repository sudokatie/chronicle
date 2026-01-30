//! Search commands

use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::db::{links::get_backlinks, search::search_notes as db_search, Backlink, SearchResult};
use crate::error::ChronicleError;

/// Search notes
#[tauri::command]
pub async fn search_notes(
    query: String,
    limit: Option<usize>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<SearchResult>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();

    let results = db_search(&conn, &query, limit.unwrap_or(20))?;
    Ok(results)
}

/// Get backlinks to a note
#[tauri::command]
pub async fn get_backlinks_cmd(
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Backlink>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();

    let backlinks = get_backlinks(&conn, &path)?;
    Ok(backlinks)
}
