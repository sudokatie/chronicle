//! Error types for Chronicle

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChronicleError {
    #[error("Vault not found: {0}")]
    VaultNotFound(String),
    
    #[error("Note not found: {0}")]
    NoteNotFound(String),
    
    #[error("Note already exists: {0}")]
    NoteExists(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("No vault is open")]
    NoVaultOpen,
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("IO error: {0}")]
    Io(String),
}

// Make error serializable for Tauri
impl Serialize for ChronicleError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<rusqlite::Error> for ChronicleError {
    fn from(err: rusqlite::Error) -> Self {
        ChronicleError::Database(err.to_string())
    }
}

impl From<std::io::Error> for ChronicleError {
    fn from(err: std::io::Error) -> Self {
        ChronicleError::Io(err.to_string())
    }
}

impl From<crate::vault::IndexError> for ChronicleError {
    fn from(err: crate::vault::IndexError) -> Self {
        match err {
            crate::vault::IndexError::VaultNotFound(p) => {
                ChronicleError::VaultNotFound(p.to_string_lossy().to_string())
            }
            crate::vault::IndexError::Database(e) => ChronicleError::Database(e.to_string()),
            crate::vault::IndexError::Io(e) => ChronicleError::Io(e.to_string()),
        }
    }
}
