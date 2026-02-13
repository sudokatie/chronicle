//! Conflict detection and resolution

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Information about a conflicting file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    /// Relative path to the conflicting file
    pub path: String,
    /// Local (ours) version content
    pub local_content: String,
    /// Remote (theirs) version content
    pub remote_content: String,
    /// Common ancestor content (if available)
    pub base_content: Option<String>,
}

/// How to resolve a conflict
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    /// Keep local version, discard remote
    KeepLocal,
    /// Keep remote version, discard local
    KeepRemote,
    /// Keep both versions (create note-conflict-1.md, note-conflict-2.md)
    KeepBoth,
}

/// Parse git conflict markers from content
pub fn parse_conflict_markers(content: &str) -> Option<(String, String, Option<String>)> {
    let mut local = String::new();
    let mut remote = String::new();
    let mut base = String::new();
    let mut section = 0; // 0=before, 1=local, 2=base, 3=remote, 4=after

    for line in content.lines() {
        if line.starts_with("<<<<<<<") {
            section = 1;
        } else if line.starts_with("|||||||") {
            section = 2;
        } else if line.starts_with("=======") {
            section = 3;
        } else if line.starts_with(">>>>>>>") {
            section = 4;
        } else {
            match section {
                1 => {
                    local.push_str(line);
                    local.push('\n');
                }
                2 => {
                    base.push_str(line);
                    base.push('\n');
                }
                3 => {
                    remote.push_str(line);
                    remote.push('\n');
                }
                _ => {}
            }
        }
    }

    if section == 4 {
        let base_opt = if base.is_empty() { None } else { Some(base) };
        Some((local, remote, base_opt))
    } else {
        None
    }
}

/// Resolve conflict by writing the chosen content
pub fn resolve_conflict(
    vault_path: &Path,
    relative_path: &str,
    resolution: ConflictResolution,
    local_content: &str,
    remote_content: &str,
) -> std::io::Result<Vec<String>> {
    let file_path = vault_path.join(relative_path);
    let mut created_files = Vec::new();

    match resolution {
        ConflictResolution::KeepLocal => {
            fs::write(&file_path, local_content)?;
            created_files.push(relative_path.to_string());
        }
        ConflictResolution::KeepRemote => {
            fs::write(&file_path, remote_content)?;
            created_files.push(relative_path.to_string());
        }
        ConflictResolution::KeepBoth => {
            // Create separate files for both versions
            let stem = Path::new(relative_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("note");
            let ext = Path::new(relative_path)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("md");
            let parent = Path::new(relative_path).parent().unwrap_or(Path::new(""));

            let local_path = parent.join(format!("{}-local.{}", stem, ext));
            let remote_path = parent.join(format!("{}-remote.{}", stem, ext));

            fs::write(vault_path.join(&local_path), local_content)?;
            fs::write(vault_path.join(&remote_path), remote_content)?;

            // Remove original conflicted file
            fs::remove_file(&file_path)?;

            created_files.push(local_path.to_string_lossy().to_string());
            created_files.push(remote_path.to_string_lossy().to_string());
        }
    }

    Ok(created_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_conflict_markers() {
        let content = r#"<<<<<<< HEAD
local content
=======
remote content
>>>>>>> origin/main
"#;
        let result = parse_conflict_markers(content);
        assert!(result.is_some());
        let (local, remote, base) = result.unwrap();
        assert_eq!(local.trim(), "local content");
        assert_eq!(remote.trim(), "remote content");
        assert!(base.is_none());
    }

    #[test]
    fn test_parse_conflict_with_base() {
        let content = r#"<<<<<<< HEAD
local content
||||||| merged common ancestors
base content
=======
remote content
>>>>>>> origin/main
"#;
        let result = parse_conflict_markers(content);
        assert!(result.is_some());
        let (local, remote, base) = result.unwrap();
        assert_eq!(local.trim(), "local content");
        assert_eq!(remote.trim(), "remote content");
        assert_eq!(base.unwrap().trim(), "base content");
    }

    #[test]
    fn test_resolve_keep_local() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.md");
        fs::write(&file_path, "conflicted").unwrap();

        let result = resolve_conflict(
            temp.path(),
            "test.md",
            ConflictResolution::KeepLocal,
            "local content",
            "remote content",
        );

        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(&file_path).unwrap(), "local content");
    }

    #[test]
    fn test_resolve_keep_remote() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.md");
        fs::write(&file_path, "conflicted").unwrap();

        let result = resolve_conflict(
            temp.path(),
            "test.md",
            ConflictResolution::KeepRemote,
            "local content",
            "remote content",
        );

        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(&file_path).unwrap(), "remote content");
    }

    #[test]
    fn test_resolve_keep_both() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.md");
        fs::write(&file_path, "conflicted").unwrap();

        let result = resolve_conflict(
            temp.path(),
            "test.md",
            ConflictResolution::KeepBoth,
            "local content",
            "remote content",
        );

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        assert!(temp.path().join("test-local.md").exists());
        assert!(temp.path().join("test-remote.md").exists());
        assert!(!file_path.exists());
    }
}
