//! Database schema and initialization

use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Database wrapper with connection pooling
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Open or create database at path
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Use WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode = WAL;")?;

        // Initialize schema
        init_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Open in-memory database (for testing)
    pub fn open_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        init_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get connection for operations
    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("Database mutex poisoned")
    }
}

/// Initialize database schema
pub fn init_db(conn: &Connection) -> Result<()> {
    init_schema(conn)
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

const SCHEMA: &str = r#"
-- Notes metadata (synced from filesystem)
CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    created_at TEXT,
    modified_at TEXT,
    content_hash TEXT,
    word_count INTEGER DEFAULT 0
);

-- Full-text search index (external content table)
CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
    title, 
    content,
    tokenize = 'porter unicode61'
);

-- Links between notes
CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY,
    source_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    target_path TEXT NOT NULL,
    target_id INTEGER REFERENCES notes(id) ON DELETE SET NULL,
    display_text TEXT,
    line_number INTEGER,
    UNIQUE(source_id, target_path, line_number)
);

-- Tags
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL COLLATE NOCASE
);

-- Note-tag relationships
CREATE TABLE IF NOT EXISTS note_tags (
    note_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_id);
CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_id);
CREATE INDEX IF NOT EXISTS idx_links_target_path ON links(target_path);
CREATE INDEX IF NOT EXISTS idx_notes_modified ON notes(modified_at);
CREATE INDEX IF NOT EXISTS idx_notes_path ON notes(path);
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let db = Database::open_memory().expect("Failed to create database");
        let conn = db.conn();

        // Verify tables exist
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='notes'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query");

        assert_eq!(count, 1);
    }

    #[test]
    fn test_foreign_keys_enabled() {
        let db = Database::open_memory().expect("Failed to create database");
        let conn = db.conn();

        let fk_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("Failed to query");

        assert_eq!(fk_enabled, 1);
    }
}
