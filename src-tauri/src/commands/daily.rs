//! Daily notes commands

use chrono::{Datelike, Duration, Local, NaiveDate};
use std::fs;
use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::db::notes as db_notes;
use crate::error::ChronicleError;
use crate::models::{AppConfig, DailyNotesConfig};
use crate::vault::Indexer;

/// Get or create today's daily note
#[tauri::command]
pub async fn get_or_create_today(
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let today = Local::now().date_naive();
    get_or_create_daily_note_for_date(today, state).await
}

/// Get or create a daily note for a specific date (YYYY-MM-DD)
#[tauri::command]
pub async fn get_or_create_daily_note(
    date: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ChronicleError::InvalidDate(date.clone()))?;
    get_or_create_daily_note_for_date(parsed_date, state).await
}

/// Navigate to previous/next daily note
#[tauri::command]
pub async fn navigate_daily_note(
    current_date: String,
    direction: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let parsed_date = NaiveDate::parse_from_str(&current_date, "%Y-%m-%d")
        .map_err(|_| ChronicleError::InvalidDate(current_date.clone()))?;
    
    let target_date = match direction.as_str() {
        "prev" | "previous" => parsed_date - Duration::days(1),
        "next" => parsed_date + Duration::days(1),
        _ => return Err(ChronicleError::InvalidDirection(direction)),
    };
    
    get_or_create_daily_note_for_date(target_date, state).await
}

/// List all daily notes
#[tauri::command]
pub async fn list_daily_notes(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<DailyNoteInfo>, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();
    
    let config = AppConfig::load();
    let daily_folder = &config.daily_notes.folder;
    
    // Get all notes in the daily folder
    let all_notes = db_notes::list_notes(&conn)?;
    let daily_notes: Vec<DailyNoteInfo> = all_notes
        .into_iter()
        .filter(|n| n.path.starts_with(daily_folder))
        .filter_map(|n| {
            // Try to extract date from filename
            let filename = n.path.rsplit('/').next()?.trim_end_matches(".md");
            let date = NaiveDate::parse_from_str(filename, "%Y-%m-%d").ok()?;
            Some(DailyNoteInfo {
                path: n.path,
                title: n.title,
                date: date.format("%Y-%m-%d").to_string(),
                word_count: n.word_count,
            })
        })
        .collect();
    
    Ok(daily_notes)
}

/// Get the daily note path for a date
#[tauri::command]
pub async fn get_daily_note_path(date: String) -> Result<String, ChronicleError> {
    let config = AppConfig::load();
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ChronicleError::InvalidDate(date))?;
    Ok(format_daily_note_path(&parsed_date, &config.daily_notes))
}

/// Check if a daily note exists for a date
#[tauri::command]
pub async fn daily_note_exists(
    date: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    
    let config = AppConfig::load();
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ChronicleError::InvalidDate(date))?;
    
    let path = format_daily_note_path(&parsed_date, &config.daily_notes);
    let full_path = vault_path.join(&path);
    
    Ok(full_path.exists())
}

/// Daily note summary info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DailyNoteInfo {
    pub path: String,
    pub title: String,
    pub date: String,
    pub word_count: i32,
}

// Internal helper functions

async fn get_or_create_daily_note_for_date(
    date: NaiveDate,
    state: State<'_, Mutex<AppState>>,
) -> Result<db_notes::NoteMeta, ChronicleError> {
    let config = AppConfig::load();
    let daily_config = &config.daily_notes;
    
    let path = format_daily_note_path(&date, daily_config);
    
    let app_state = state.lock().expect("Failed to lock state");
    let vault_path = app_state
        .vault_path
        .as_ref()
        .ok_or(ChronicleError::NoVaultOpen)?;
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    
    let full_path = vault_path.join(&path);
    
    // Check if note exists
    if !full_path.exists() {
        // Create the daily notes folder if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate content from template
        let content = render_daily_template(&date, daily_config);
        fs::write(&full_path, &content)?;
        
        // Index the new note
        let indexer = Indexer::new(vault_path.clone())?;
        indexer.index_file(db, &full_path)?;
    }
    
    let conn = db.conn();
    let meta = db_notes::get_note_by_path(&conn, &path)?
        .ok_or_else(|| ChronicleError::NoteNotFound(path))?;
    
    Ok(meta)
}

fn format_daily_note_path(date: &NaiveDate, config: &DailyNotesConfig) -> String {
    let date_str = date.format(&config.date_format).to_string();
    format!("{}/{}.md", config.folder, date_str)
}

fn render_daily_template(date: &NaiveDate, config: &DailyNotesConfig) -> String {
    let date_str = date.format(&config.date_format).to_string();
    let prev_date = (*date - Duration::days(1)).format(&config.date_format).to_string();
    let next_date = (*date + Duration::days(1)).format(&config.date_format).to_string();
    
    let mut content = config.template.clone();
    
    // Replace template variables
    content = content.replace("{{date}}", &date_str);
    content = content.replace("{{year}}", &date.year().to_string());
    content = content.replace("{{month}}", &format!("{:02}", date.month()));
    content = content.replace("{{day}}", &format!("{:02}", date.day()));
    content = content.replace("{{weekday}}", &date.weekday().to_string());
    content = content.replace("{{previous_date}}", &prev_date);
    content = content.replace("{{next_date}}", &next_date);
    
    // Handle navigation links based on config
    if !config.link_previous_day {
        // Remove previous day link line
        content = content
            .lines()
            .filter(|line| !line.contains("{{previous_date}}") && !line.contains(&prev_date))
            .collect::<Vec<_>>()
            .join("\n");
    }
    
    if !config.link_next_day {
        // Remove next day link line
        content = content
            .lines()
            .filter(|line| !line.contains("{{next_date}}") && !line.contains(&next_date))
            .collect::<Vec<_>>()
            .join("\n");
    }
    
    content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_daily_note_path() {
        let config = DailyNotesConfig::default();
        let date = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        let path = format_daily_note_path(&date, &config);
        assert_eq!(path, "daily/2026-03-09.md");
    }
    
    #[test]
    fn test_format_daily_note_path_custom_folder() {
        let config = DailyNotesConfig {
            folder: "journal".to_string(),
            ..Default::default()
        };
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let path = format_daily_note_path(&date, &config);
        assert_eq!(path, "journal/2026-01-15.md");
    }
    
    #[test]
    fn test_render_daily_template() {
        let config = DailyNotesConfig::default();
        let date = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        let content = render_daily_template(&date, &config);
        
        assert!(content.contains("# 2026-03-09"));
        assert!(content.contains("2026-03-08")); // Previous day link
        assert!(content.contains("2026-03-10")); // Next day link
        assert!(content.contains("## Tasks"));
        assert!(content.contains("## Notes"));
    }
    
    #[test]
    fn test_render_daily_template_custom() {
        let config = DailyNotesConfig {
            template: "# {{weekday}}, {{date}}\n\n## Today\n\n".to_string(),
            ..Default::default()
        };
        let date = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        let content = render_daily_template(&date, &config);
        
        // chrono weekday format is "Sun" 
        assert!(content.contains("2026-03-09"));
        assert!(content.contains("## Today"));
        // Check that weekday was replaced (should contain the weekday name)
        assert!(!content.contains("{{weekday}}"));
    }
    
    #[test]
    fn test_render_template_variables() {
        let config = DailyNotesConfig {
            template: "Year: {{year}}, Month: {{month}}, Day: {{day}}".to_string(),
            ..Default::default()
        };
        let date = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        let content = render_daily_template(&date, &config);
        
        assert!(content.contains("Year: 2026"));
        assert!(content.contains("Month: 03"));
        assert!(content.contains("Day: 09"));
    }
    
    #[test]
    fn test_default_daily_config() {
        let config = DailyNotesConfig::default();
        assert_eq!(config.folder, "daily");
        assert_eq!(config.date_format, "%Y-%m-%d");
        assert!(config.link_previous_day);
        assert!(config.link_next_day);
    }
}
