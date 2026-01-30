//! Vault management commands

use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

use crate::db::schema::Database;
use crate::error::ChronicleError;
use crate::models::VaultInfo;
use crate::vault::{Indexer, VaultWatcher};

/// Events emitted to frontend
#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum VaultEventPayload {
    #[serde(rename = "note_created")]
    NoteCreated { path: String },
    #[serde(rename = "note_modified")]
    NoteModified { path: String },
    #[serde(rename = "note_deleted")]
    NoteDeleted { path: String },
    #[serde(rename = "note_renamed")]
    NoteRenamed { old_path: String, new_path: String },
    #[serde(rename = "index_complete")]
    IndexComplete { note_count: usize },
}

/// Application state
#[derive(Default)]
pub struct AppState {
    pub db: Option<Database>,
    pub vault_path: Option<PathBuf>,
    pub watcher: Option<VaultWatcher>,
}


/// Open a vault directory
#[tauri::command]
pub async fn open_vault(
    path: String,
    state: State<'_, Mutex<AppState>>,
    app: AppHandle,
) -> Result<VaultInfo, ChronicleError> {
    let vault_path = PathBuf::from(&path);

    if !vault_path.exists() {
        return Err(ChronicleError::VaultNotFound(path));
    }

    // Database path in vault directory
    let db_path = vault_path.join(".chronicle").join("chronicle.db");
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Open database
    let db = Database::open(&db_path).map_err(|e| ChronicleError::Database(e.to_string()))?;

    // Index vault
    let indexer = Indexer::new(vault_path.clone())?;
    let note_count = indexer.full_index(&db)?;

    // Start file watcher
    let watcher =
        VaultWatcher::new(vault_path.clone()).map_err(|e| ChronicleError::Io(e.to_string()))?;

    // Update state
    {
        let mut app_state = state.lock().expect("Failed to lock state");
        app_state.db = Some(db);
        app_state.vault_path = Some(vault_path.clone());
        app_state.watcher = Some(watcher);
    }

    // Emit index complete event
    let _ = app.emit(
        "vault-event",
        VaultEventPayload::IndexComplete { note_count },
    );

    Ok(VaultInfo {
        path: vault_path.to_string_lossy().to_string(),
        note_count,
        is_open: true,
    })
}

/// Get current vault info
#[tauri::command]
pub async fn get_vault_info(
    state: State<'_, Mutex<AppState>>,
) -> Result<VaultInfo, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");

    match &app_state.vault_path {
        Some(path) => {
            let note_count = if let Some(db) = &app_state.db {
                let conn = db.conn();
                crate::db::notes::list_notes(&conn)
                    .map(|notes| notes.len())
                    .unwrap_or(0)
            } else {
                0
            };

            Ok(VaultInfo {
                path: path.to_string_lossy().to_string(),
                note_count,
                is_open: true,
            })
        }
        None => Ok(VaultInfo {
            path: String::new(),
            note_count: 0,
            is_open: false,
        }),
    }
}

/// Close the current vault
#[tauri::command]
pub async fn close_vault(state: State<'_, Mutex<AppState>>) -> Result<(), ChronicleError> {
    let mut app_state = state.lock().expect("Failed to lock state");

    app_state.db = None;
    app_state.vault_path = None;
    app_state.watcher = None;

    Ok(())
}
