// Chronicle - Personal Knowledge Graph

pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod sync;
pub mod vault;

use commands::vault::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::open_vault,
            commands::get_vault_info,
            commands::close_vault,
            commands::poll_vault_events,
            commands::list_notes,
            commands::get_note,
            commands::create_note,
            commands::save_note,
            commands::delete_note,
            commands::rename_note,
            commands::update_note_tags,
            commands::search_notes,
            commands::get_backlinks_cmd,
            commands::get_graph_data,
            commands::list_tags,
            commands::get_notes_by_tag,
            commands::get_config,
            commands::save_config,
            // Sync commands
            commands::sync_status,
            commands::sync_init,
            commands::sync_push,
            commands::sync_pull,
            commands::sync_get_conflict,
            commands::sync_resolve_conflict,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
