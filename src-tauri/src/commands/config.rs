//! Configuration commands

use crate::error::ChronicleError;
use crate::models::AppConfig;

/// Get current config
#[tauri::command]
pub async fn get_config() -> Result<AppConfig, ChronicleError> {
    Ok(AppConfig::load())
}

/// Save config
#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), ChronicleError> {
    config.save().map_err(|e| ChronicleError::Io(e.to_string()))
}
