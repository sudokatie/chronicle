//! Tag database operations

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

/// Tag with note count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub id: i64,
    pub name: String,
    pub count: i32,
}

/// Get or create a tag, return its ID
pub fn get_or_create_tag(conn: &Connection, name: &str) -> Result<i64> {
    // Try to get existing tag
    let existing: Result<i64> = conn.query_row(
        "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
        params![name],
        |row| row.get(0),
    );

    match existing {
        Ok(id) => Ok(id),
        Err(_) => {
            // Create new tag
            conn.execute("INSERT INTO tags (name) VALUES (?1)", params![name])?;
            Ok(conn.last_insert_rowid())
        }
    }
}

/// Set tags for a note (replaces existing)
pub fn set_note_tags(conn: &Connection, note_id: i64, tags: &[String]) -> Result<()> {
    // Remove existing tags
    conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![note_id])?;

    // Add new tags
    for tag_name in tags {
        let tag_id = get_or_create_tag(conn, tag_name)?;
        conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )?;
    }

    Ok(())
}

/// Get tags for a note
pub fn get_note_tags(conn: &Connection, note_id: i64) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT t.name
        FROM tags t
        JOIN note_tags nt ON t.id = nt.tag_id
        WHERE nt.note_id = ?1
        ORDER BY t.name
        "#,
    )?;

    let rows = stmt.query_map(params![note_id], |row| row.get(0))?;
    rows.collect()
}

/// List all tags with counts
pub fn list_tags(conn: &Connection) -> Result<Vec<TagInfo>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT t.id, t.name, COUNT(nt.note_id) as count
        FROM tags t
        LEFT JOIN note_tags nt ON t.id = nt.tag_id
        GROUP BY t.id, t.name
        HAVING count > 0
        ORDER BY t.name
        "#,
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(TagInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            count: row.get(2)?,
        })
    })?;

    rows.collect()
}

/// Get notes with a specific tag
pub fn get_notes_by_tag(conn: &Connection, tag_name: &str) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT nt.note_id
        FROM note_tags nt
        JOIN tags t ON nt.tag_id = t.id
        WHERE t.name = ?1 COLLATE NOCASE
        "#,
    )?;

    let rows = stmt.query_map(params![tag_name], |row| row.get(0))?;
    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{notes::upsert_note, schema::Database};

    #[test]
    fn test_set_and_get_tags() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let note_id = upsert_note(&conn, "test.md", "Test", None, None, "x", 0).unwrap();

        set_note_tags(
            &conn,
            note_id,
            &["rust".to_string(), "programming".to_string()],
        )
        .unwrap();

        let tags = get_note_tags(&conn, note_id).unwrap();
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"programming".to_string()));
    }

    #[test]
    fn test_list_tags() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let note1 = upsert_note(&conn, "a.md", "A", None, None, "x", 0).unwrap();
        let note2 = upsert_note(&conn, "b.md", "B", None, None, "x", 0).unwrap();

        set_note_tags(&conn, note1, &["rust".to_string()]).unwrap();
        set_note_tags(&conn, note2, &["rust".to_string(), "go".to_string()]).unwrap();

        let tags = list_tags(&conn).unwrap();
        assert_eq!(tags.len(), 2);

        let rust_tag = tags.iter().find(|t| t.name == "rust").unwrap();
        assert_eq!(rust_tag.count, 2);
    }

    #[test]
    fn test_case_insensitive_tags() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let id1 = get_or_create_tag(&conn, "Rust").unwrap();
        let id2 = get_or_create_tag(&conn, "rust").unwrap();
        let id3 = get_or_create_tag(&conn, "RUST").unwrap();

        assert_eq!(id1, id2);
        assert_eq!(id2, id3);
    }
}
