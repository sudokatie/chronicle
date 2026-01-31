//! Link database operations

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

/// Link between notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: i64,
    pub source_id: i64,
    pub target_path: String,
    pub target_id: Option<i64>,
    pub display_text: Option<String>,
    pub line_number: Option<i32>,
}

/// Backlink with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlink {
    pub source_path: String,
    pub source_title: String,
    pub line_number: Option<i32>,
    pub display_text: Option<String>,
    pub context: Option<String>, // Surrounding text from the source note
}

/// Replace all links for a note
pub fn replace_links(
    conn: &Connection,
    source_id: i64,
    links: &[(String, Option<String>, Option<i32>)], // (target_path, display_text, line_number)
) -> Result<()> {
    // Delete existing links
    conn.execute("DELETE FROM links WHERE source_id = ?1", params![source_id])?;

    // Insert new links, ignoring duplicates (same target on same line)
    let mut stmt = conn.prepare(
        "INSERT OR IGNORE INTO links (source_id, target_path, display_text, line_number) VALUES (?1, ?2, ?3, ?4)"
    )?;

    for (target_path, display_text, line_number) in links {
        stmt.execute(params![source_id, target_path, display_text, line_number])?;
    }

    // Resolve links to existing notes
    conn.execute(
        r#"
        UPDATE links SET target_id = (
            SELECT id FROM notes WHERE LOWER(notes.path) = LOWER(links.target_path || '.md')
            OR LOWER(notes.path) = LOWER(links.target_path)
        )
        WHERE source_id = ?1
        "#,
        params![source_id],
    )?;

    Ok(())
}

/// Get backlinks to a note (without context - context added at command level)
pub fn get_backlinks(conn: &Connection, path: &str) -> Result<Vec<Backlink>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT n.path, n.title, l.line_number, l.display_text
        FROM links l
        JOIN notes n ON l.source_id = n.id
        WHERE LOWER(l.target_path) = LOWER(?1)
           OR LOWER(l.target_path || '.md') = LOWER(?1)
        ORDER BY n.modified_at DESC
        "#,
    )?;

    let rows = stmt.query_map(params![path], |row| {
        Ok(Backlink {
            source_path: row.get(0)?,
            source_title: row.get(1)?,
            line_number: row.get(2)?,
            display_text: row.get(3)?,
            context: None, // Populated at command level with file access
        })
    })?;

    rows.collect()
}

/// Get outgoing links from a note
pub fn get_outlinks(conn: &Connection, source_id: i64) -> Result<Vec<Link>> {
    let mut stmt = conn.prepare(
        "SELECT id, source_id, target_path, target_id, display_text, line_number FROM links WHERE source_id = ?1"
    )?;

    let rows = stmt.query_map(params![source_id], |row| {
        Ok(Link {
            id: row.get(0)?,
            source_id: row.get(1)?,
            target_path: row.get(2)?,
            target_id: row.get(3)?,
            display_text: row.get(4)?,
            line_number: row.get(5)?,
        })
    })?;

    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{notes::upsert_note, schema::Database};

    #[test]
    fn test_replace_links() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let id = upsert_note(&conn, "source.md", "Source", None, None, "x", 0).unwrap();

        let links = vec![
            ("target1".to_string(), None, Some(5)),
            ("target2".to_string(), Some("display".to_string()), Some(10)),
        ];

        replace_links(&conn, id, &links).unwrap();

        let outlinks = get_outlinks(&conn, id).unwrap();
        assert_eq!(outlinks.len(), 2);
    }
}
