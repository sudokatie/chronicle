//! Search commands

use std::fs;
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

/// Get backlinks to a note with surrounding context
#[tauri::command]
pub async fn get_backlinks_cmd(
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Backlink>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();

    let mut backlinks = get_backlinks(&conn, &path)?;
    
    // Add context by reading source files
    for backlink in &mut backlinks {
        if let Some(line_num) = backlink.line_number {
            let source_path = vault_path.join(&backlink.source_path);
            if let Ok(content) = fs::read_to_string(&source_path) {
                let lines: Vec<&str> = content.lines().collect();
                let idx = (line_num - 1) as usize;
                if idx < lines.len() {
                    // Get the line containing the link, trimmed
                    let line = lines[idx].trim();
                    // Truncate if too long
                    let context = if line.len() > 120 {
                        format!("{}...", &line[..117])
                    } else {
                        line.to_string()
                    };
                    backlink.context = Some(context);
                }
            }
        }
    }
    
    Ok(backlinks)
}
