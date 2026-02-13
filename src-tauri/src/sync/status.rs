//! Sync status tracking

use serde::{Deserialize, Serialize};

/// Current synchronization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Whether the vault is a git repository
    pub initialized: bool,
    /// Remote URL if configured
    pub remote_url: Option<String>,
    /// Current branch name
    pub branch: String,
    /// Number of commits ahead of remote
    pub ahead: usize,
    /// Number of commits behind remote
    pub behind: usize,
    /// Files with merge conflicts
    pub conflicts: Vec<String>,
    /// Last successful sync timestamp (ISO 8601)
    pub last_sync: Option<String>,
    /// Whether there are uncommitted changes
    pub dirty: bool,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            initialized: false,
            remote_url: None,
            branch: String::from("main"),
            ahead: 0,
            behind: 0,
            conflicts: Vec::new(),
            last_sync: None,
            dirty: false,
        }
    }
}

impl SyncStatus {
    /// Create status for uninitialized repo
    pub fn uninitialized() -> Self {
        Self::default()
    }

    /// Check if sync is needed
    pub fn needs_sync(&self) -> bool {
        self.dirty || self.ahead > 0 || self.behind > 0
    }

    /// Check if there are conflicts to resolve
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_status() {
        let status = SyncStatus::default();
        assert!(!status.initialized);
        assert!(!status.needs_sync());
        assert!(!status.has_conflicts());
    }

    #[test]
    fn test_needs_sync_when_dirty() {
        let status = SyncStatus {
            dirty: true,
            ..Default::default()
        };
        assert!(status.needs_sync());
    }

    #[test]
    fn test_needs_sync_when_ahead() {
        let status = SyncStatus {
            ahead: 1,
            ..Default::default()
        };
        assert!(status.needs_sync());
    }

    #[test]
    fn test_has_conflicts() {
        let status = SyncStatus {
            conflicts: vec!["note.md".to_string()],
            ..Default::default()
        };
        assert!(status.has_conflicts());
    }
}
