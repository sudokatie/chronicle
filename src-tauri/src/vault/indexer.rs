//! Vault indexer - scans and indexes notes

use crate::db::{
    self,
    schema::Database,
    notes::{upsert_note, delete_note as db_delete_note, get_note_by_path},
    links::replace_links,
    tags::set_note_tags,
    search::update_fts,
};
use crate::vault::parser::parse_note;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Vault path does not exist: {0}")]
    VaultNotFound(PathBuf),
}

/// Vault indexer
pub struct Indexer {
    vault_path: PathBuf,
}

impl Indexer {
    /// Create new indexer for vault path
    pub fn new(vault_path: PathBuf) -> Result<Self, IndexError> {
        if !vault_path.exists() {
            return Err(IndexError::VaultNotFound(vault_path));
        }
        Ok(Self { vault_path })
    }
    
    /// Full index of all notes in vault
    pub fn full_index(&self, db: &Database) -> Result<usize, IndexError> {
        let mut count = 0;
        
        for entry in walkdir(&self.vault_path)? {
            if self.is_markdown_file(&entry) {
                if let Err(e) = self.index_file(db, &entry) {
                    eprintln!("Error indexing {:?}: {}", entry, e);
                    continue;
                }
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// Index a single file
    pub fn index_file(&self, db: &Database, path: &Path) -> Result<(), IndexError> {
        let relative_path = path.strip_prefix(&self.vault_path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        let content = fs::read_to_string(path)?;
        let filename = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        let parsed = parse_note(&content, filename);
        let content_hash = hash_content(&content);
        
        let conn = db.conn();
        
        // Get timestamps from file metadata
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified().ok()
            .map(|t| chrono_from_systemtime(t));
        let created = metadata.created().ok()
            .map(|t| chrono_from_systemtime(t));
        
        // Upsert note
        let note_id = upsert_note(
            &conn,
            &relative_path,
            &parsed.title,
            created.as_deref(),
            modified.as_deref(),
            &content_hash,
            parsed.word_count as i32,
        )?;
        
        // Update FTS index
        update_fts(&conn, note_id, &parsed.title, &content)?;
        
        // Update links
        let links: Vec<(String, Option<String>, Option<i32>)> = parsed.links
            .into_iter()
            .map(|l| (l.target, l.display, Some(l.line_number as i32)))
            .collect();
        replace_links(&conn, note_id, &links)?;
        
        // Update tags from frontmatter
        if let Some(fm) = parsed.frontmatter {
            set_note_tags(&conn, note_id, &fm.tags)?;
        }
        
        Ok(())
    }
    
    /// Remove a file from the index
    pub fn remove_file(&self, db: &Database, path: &Path) -> Result<(), IndexError> {
        let relative_path = path.strip_prefix(&self.vault_path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        let conn = db.conn();
        
        // Get note ID for FTS cleanup
        if let Some(note) = get_note_by_path(&conn, &relative_path)? {
            db::search::delete_fts(&conn, note.id)?;
        }
        
        db_delete_note(&conn, &relative_path)?;
        Ok(())
    }
    
    /// Check if path is a markdown file
    fn is_markdown_file(&self, path: &Path) -> bool {
        path.is_file() && 
        path.extension().map(|e| e == "md").unwrap_or(false)
    }
}

/// Walk directory recursively, skipping hidden files/dirs
fn walkdir(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    walkdir_recursive(root, &mut files)?;
    Ok(files)
}

fn walkdir_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        
        // Skip hidden files and directories
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        
        if path.is_dir() {
            walkdir_recursive(&path, files)?;
        } else {
            files.push(path);
        }
    }
    
    Ok(())
}

/// Hash content for change detection
fn hash_content(content: &str) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Convert SystemTime to ISO 8601 string
fn chrono_from_systemtime(time: std::time::SystemTime) -> String {
    let duration = time.duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;
    
    // Simple ISO 8601 format without external deps
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    
    // Approximate date calculation (not accounting for leap years exactly)
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;
    
    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1;
    let mut day = remaining_days + 1;
    
    for days in month_days.iter() {
        let days_in_month = *days as i64;
        if day <= days_in_month {
            break;
        }
        day -= days_in_month;
        month += 1;
    }
    
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;
    
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    fn setup_test_vault() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db = Database::open_memory().unwrap();
        
        // Create test notes
        fs::write(temp.path().join("note1.md"), "# Note One\n\nContent.").unwrap();
        fs::write(temp.path().join("note2.md"), "---\ntitle: Note Two\ntags:\n  - test\n---\n\nLinks to [[note1]].").unwrap();
        
        // Create subdirectory
        fs::create_dir(temp.path().join("subdir")).unwrap();
        fs::write(temp.path().join("subdir/nested.md"), "# Nested\n\nDeep note.").unwrap();
        
        // Create hidden file (should be skipped)
        fs::write(temp.path().join(".hidden.md"), "Should be skipped").unwrap();
        
        (temp, db)
    }
    
    #[test]
    fn test_full_index() {
        let (temp, db) = setup_test_vault();
        let indexer = Indexer::new(temp.path().to_path_buf()).unwrap();
        
        let count = indexer.full_index(&db).unwrap();
        
        // Should find 3 markdown files (excluding hidden)
        assert_eq!(count, 3);
        
        // Verify notes in DB
        let conn = db.conn();
        let notes = db::notes::list_notes(&conn).unwrap();
        assert_eq!(notes.len(), 3);
    }
    
    #[test]
    fn test_index_extracts_links() {
        let (temp, db) = setup_test_vault();
        let indexer = Indexer::new(temp.path().to_path_buf()).unwrap();
        
        indexer.full_index(&db).unwrap();
        
        let conn = db.conn();
        
        // Get note2 which has a link
        let note2 = db::notes::get_note_by_path(&conn, "note2.md").unwrap().unwrap();
        let links = db::links::get_outlinks(&conn, note2.id).unwrap();
        
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target_path, "note1");
    }
    
    #[test]
    fn test_index_extracts_tags() {
        let (temp, db) = setup_test_vault();
        let indexer = Indexer::new(temp.path().to_path_buf()).unwrap();
        
        indexer.full_index(&db).unwrap();
        
        let conn = db.conn();
        
        // Get note2 which has tags
        let note2 = db::notes::get_note_by_path(&conn, "note2.md").unwrap().unwrap();
        let tags = db::tags::get_note_tags(&conn, note2.id).unwrap();
        
        assert_eq!(tags, vec!["test"]);
    }
    
    #[test]
    fn test_remove_file() {
        let (temp, db) = setup_test_vault();
        let indexer = Indexer::new(temp.path().to_path_buf()).unwrap();
        
        indexer.full_index(&db).unwrap();
        
        let conn = db.conn();
        assert!(db::notes::get_note_by_path(&conn, "note1.md").unwrap().is_some());
        
        drop(conn); // Release connection before remove
        
        indexer.remove_file(&db, &temp.path().join("note1.md")).unwrap();
        
        let conn = db.conn();
        assert!(db::notes::get_note_by_path(&conn, "note1.md").unwrap().is_none());
    }
}
