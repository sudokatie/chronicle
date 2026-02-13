//! Sync commands for Tauri

use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::error::ChronicleError;
use crate::sync::{ConflictInfo, ConflictResolution, GitRepo, SyncStatus};
use crate::sync::conflict::{parse_conflict_markers, resolve_conflict};

/// Result type for sync operations
#[derive(serde::Serialize)]
pub struct SyncResult {
    pub success: bool,
    pub files_changed: Vec<String>,
    pub conflicts: Vec<String>,
    pub message: String,
}

/// Get current sync status
#[tauri::command]
pub async fn sync_status(state: State<'_, Mutex<AppState>>) -> Result<SyncStatus, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    
    if !GitRepo::is_repo(vault_path) {
        return Ok(SyncStatus::uninitialized());
    }
    
    let repo = GitRepo::open(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    repo.status().map_err(|e| ChronicleError::SyncError(e.to_string()))
}

/// Initialize git repository for sync
#[tauri::command]
pub async fn sync_init(
    state: State<'_, Mutex<AppState>>,
    remote_url: Option<String>,
) -> Result<SyncStatus, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    
    let repo = if GitRepo::is_repo(vault_path) {
        GitRepo::open(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?
    } else {
        GitRepo::init(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?
    };
    
    if let Some(url) = remote_url {
        repo.set_remote(&url).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    }
    
    repo.status().map_err(|e| ChronicleError::SyncError(e.to_string()))
}

/// Push local changes to remote
#[tauri::command]
pub async fn sync_push(state: State<'_, Mutex<AppState>>) -> Result<SyncResult, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    
    let repo = GitRepo::open(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    
    // Commit any pending changes
    let changed_files = repo.changed_files().map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    if !changed_files.is_empty() {
        let message = format!("Update {} notes", changed_files.len());
        repo.commit(&message).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    }
    
    // Push to remote
    repo.push().map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    
    Ok(SyncResult {
        success: true,
        files_changed: changed_files,
        conflicts: Vec::new(),
        message: "Push successful".to_string(),
    })
}

/// Pull remote changes
#[tauri::command]
pub async fn sync_pull(state: State<'_, Mutex<AppState>>) -> Result<SyncResult, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    
    let repo = GitRepo::open(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    
    // Commit any pending changes first
    if repo.is_dirty().map_err(|e| ChronicleError::SyncError(e.to_string()))? {
        repo.commit("Auto-commit before pull").map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    }
    
    // Pull from remote
    let conflicts = repo.pull().map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    
    if conflicts.is_empty() {
        Ok(SyncResult {
            success: true,
            files_changed: Vec::new(),
            conflicts: Vec::new(),
            message: "Pull successful".to_string(),
        })
    } else {
        Ok(SyncResult {
            success: false,
            files_changed: Vec::new(),
            conflicts,
            message: "Conflicts detected".to_string(),
        })
    }
}

/// Get conflict details for a file
#[tauri::command]
pub async fn sync_get_conflict(
    state: State<'_, Mutex<AppState>>,
    path: String,
) -> Result<ConflictInfo, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let file_path = vault_path.join(&path);
    
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ChronicleError::Io(e.to_string()))?;
    
    let (local, remote, base) = parse_conflict_markers(&content)
        .ok_or_else(|| ChronicleError::SyncError("No conflict markers found".to_string()))?;
    
    Ok(ConflictInfo {
        path,
        local_content: local,
        remote_content: remote,
        base_content: base,
    })
}

/// Resolve a conflict
#[tauri::command]
pub async fn sync_resolve_conflict(
    state: State<'_, Mutex<AppState>>,
    path: String,
    resolution: ConflictResolution,
) -> Result<SyncResult, ChronicleError> {
    let state = state.lock().map_err(|_| ChronicleError::LockFailed)?;
    
    let vault_path = state.vault_path.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let file_path = vault_path.join(&path);
    
    // Read the conflicted file
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ChronicleError::Io(e.to_string()))?;
    
    let (local, remote, _) = parse_conflict_markers(&content)
        .ok_or_else(|| ChronicleError::SyncError("No conflict markers found".to_string()))?;
    
    // Resolve the conflict
    let created_files = resolve_conflict(vault_path, &path, resolution, &local, &remote)
        .map_err(|e| ChronicleError::Io(e.to_string()))?;
    
    // Mark as resolved in git
    let repo = GitRepo::open(vault_path).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    for file in &created_files {
        repo.resolve_conflict(file).map_err(|e| ChronicleError::SyncError(e.to_string()))?;
    }
    
    Ok(SyncResult {
        success: true,
        files_changed: created_files,
        conflicts: Vec::new(),
        message: "Conflict resolved".to_string(),
    })
}
