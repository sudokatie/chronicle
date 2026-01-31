//! Full-text search operations

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

/// Search result with snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub snippet: String,
    pub rank: f64,
    pub match_count: i32,
}

/// Update FTS index for a note
pub fn update_fts(conn: &Connection, note_id: i64, title: &str, content: &str) -> Result<()> {
    // Delete existing entry
    conn.execute("DELETE FROM notes_fts WHERE rowid = ?1", params![note_id])?;

    // Insert new entry
    conn.execute(
        "INSERT INTO notes_fts (rowid, title, content) VALUES (?1, ?2, ?3)",
        params![note_id, title, content],
    )?;

    Ok(())
}

/// Delete FTS entry for a note
pub fn delete_fts(conn: &Connection, note_id: i64) -> Result<()> {
    conn.execute("DELETE FROM notes_fts WHERE rowid = ?1", params![note_id])?;
    Ok(())
}

/// Search notes using FTS5
pub fn search_notes(conn: &Connection, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
    // Escape FTS5 special characters
    let safe_query = escape_fts_query(query);

    if safe_query.is_empty() {
        return Ok(vec![]);
    }

    // Get raw query for match counting (without FTS escaping)
    let raw_query = query.trim().to_lowercase();

    let mut stmt = conn.prepare(
        r#"
        SELECT 
            n.id,
            n.path,
            n.title,
            snippet(notes_fts, 1, '<mark>', '</mark>', '...', 32) as snippet,
            bm25(notes_fts) as rank,
            notes_fts.content as content
        FROM notes_fts
        JOIN notes n ON notes_fts.rowid = n.id
        WHERE notes_fts MATCH ?1
        ORDER BY rank
        LIMIT ?2
        "#,
    )?;

    let mut results = Vec::new();
    let mut rows = stmt.query(params![safe_query, limit as i64])?;
    
    while let Some(row) = rows.next()? {
        let content: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let title: String = row.get(2)?;
        
        // Count occurrences in title and content
        let text = format!("{} {}", title, content).to_lowercase();
        let match_count = if !raw_query.is_empty() {
            text.matches(&raw_query).count() as i32
        } else {
            0
        };

        results.push(SearchResult {
            id: row.get(0)?,
            path: row.get(1)?,
            title,
            snippet: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            rank: row.get(4)?,
            match_count: match_count.max(1), // At least 1 if it matched
        });
    }

    Ok(results)
}

/// Escape special FTS5 characters in query
fn escape_fts_query(query: &str) -> String {
    // For simple queries, wrap terms in quotes
    // This handles most special characters
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // If query contains quotes, escape them
    let escaped = trimmed.replace('"', "\"\"");

    // Wrap in quotes for phrase search
    format!("\"{}\"", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{notes::upsert_note, schema::Database};

    #[test]
    fn test_update_and_search_fts() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let id = upsert_note(&conn, "test.md", "Hello World", None, None, "x", 10).unwrap();
        update_fts(
            &conn,
            id,
            "Hello World",
            "This is a test note about Rust programming.",
        )
        .unwrap();

        let results = search_notes(&conn, "rust", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Hello World");
    }

    #[test]
    fn test_empty_search() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let results = search_notes(&conn, "", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_escape_fts_query() {
        assert_eq!(escape_fts_query("hello"), "\"hello\"");
        assert_eq!(escape_fts_query("hello world"), "\"hello world\"");
        assert_eq!(escape_fts_query(""), "");
    }
}
