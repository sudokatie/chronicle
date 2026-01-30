//! Note database operations

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

/// Note metadata stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMeta {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub word_count: i32,
}

/// Insert or update a note in the database
pub fn upsert_note(
    conn: &Connection,
    path: &str,
    title: &str,
    created_at: Option<&str>,
    modified_at: Option<&str>,
    content_hash: &str,
    word_count: i32,
) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO notes (path, title, created_at, modified_at, content_hash, word_count)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(path) DO UPDATE SET
            title = excluded.title,
            modified_at = excluded.modified_at,
            content_hash = excluded.content_hash,
            word_count = excluded.word_count
        "#,
        params![
            path,
            title,
            created_at,
            modified_at,
            content_hash,
            word_count
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Get note by path
pub fn get_note_by_path(conn: &Connection, path: &str) -> Result<Option<NoteMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, title, created_at, modified_at, word_count FROM notes WHERE path = ?1",
    )?;

    let mut rows = stmt.query(params![path])?;

    if let Some(row) = rows.next()? {
        Ok(Some(NoteMeta {
            id: row.get(0)?,
            path: row.get(1)?,
            title: row.get(2)?,
            created_at: row.get(3)?,
            modified_at: row.get(4)?,
            word_count: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

/// Get note by ID
pub fn get_note_by_id(conn: &Connection, id: i64) -> Result<Option<NoteMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, title, created_at, modified_at, word_count FROM notes WHERE id = ?1",
    )?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(NoteMeta {
            id: row.get(0)?,
            path: row.get(1)?,
            title: row.get(2)?,
            created_at: row.get(3)?,
            modified_at: row.get(4)?,
            word_count: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

/// List all notes
pub fn list_notes(conn: &Connection) -> Result<Vec<NoteMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, title, created_at, modified_at, word_count FROM notes ORDER BY modified_at DESC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(NoteMeta {
            id: row.get(0)?,
            path: row.get(1)?,
            title: row.get(2)?,
            created_at: row.get(3)?,
            modified_at: row.get(4)?,
            word_count: row.get(5)?,
        })
    })?;

    rows.collect()
}

/// Delete note by path
pub fn delete_note(conn: &Connection, path: &str) -> Result<bool> {
    let rows_affected = conn.execute("DELETE FROM notes WHERE path = ?1", params![path])?;
    Ok(rows_affected > 0)
}

/// Update note path (for rename)
pub fn rename_note(conn: &Connection, old_path: &str, new_path: &str) -> Result<bool> {
    let rows_affected = conn.execute(
        "UPDATE notes SET path = ?1 WHERE path = ?2",
        params![new_path, old_path],
    )?;
    Ok(rows_affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::Database;

    #[test]
    fn test_upsert_and_get_note() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        let id = upsert_note(
            &conn,
            "test.md",
            "Test Note",
            Some("2024-01-01T00:00:00Z"),
            Some("2024-01-01T00:00:00Z"),
            "abc123",
            100,
        )
        .unwrap();

        assert!(id > 0);

        let note = get_note_by_path(&conn, "test.md").unwrap().unwrap();
        assert_eq!(note.title, "Test Note");
        assert_eq!(note.word_count, 100);
    }

    #[test]
    fn test_list_notes() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        upsert_note(&conn, "a.md", "Note A", None, None, "a", 10).unwrap();
        upsert_note(&conn, "b.md", "Note B", None, None, "b", 20).unwrap();

        let notes = list_notes(&conn).unwrap();
        assert_eq!(notes.len(), 2);
    }

    #[test]
    fn test_delete_note() {
        let db = Database::open_memory().unwrap();
        let conn = db.conn();

        upsert_note(&conn, "test.md", "Test", None, None, "x", 0).unwrap();
        assert!(get_note_by_path(&conn, "test.md").unwrap().is_some());

        delete_note(&conn, "test.md").unwrap();
        assert!(get_note_by_path(&conn, "test.md").unwrap().is_none());
    }
}
