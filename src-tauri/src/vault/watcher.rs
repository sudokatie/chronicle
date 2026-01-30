//! File system watcher for vault changes

use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("Channel receive error")]
    ChannelError,
}

/// Events emitted by the vault watcher
#[derive(Debug, Clone)]
pub enum VaultEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

/// File system watcher for a vault directory
pub struct VaultWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<VaultEvent>,
    vault_path: PathBuf,
}

impl VaultWatcher {
    /// Create a new watcher for the vault directory
    pub fn new(vault_path: PathBuf) -> Result<Self, WatchError> {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let vault_path_clone = vault_path.clone();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    Self::handle_event(&event, &tx_clone, &vault_path_clone);
                }
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )?;

        watcher.watch(&vault_path, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
            vault_path,
        })
    }

    /// Process raw notify event into VaultEvent
    fn handle_event(event: &Event, tx: &Sender<VaultEvent>, vault_path: &Path) {
        let paths: Vec<_> = event
            .paths
            .iter()
            .filter(|p| Self::is_markdown_file(p))
            .filter(|p| !Self::is_hidden(p, vault_path))
            .cloned()
            .collect();

        if paths.is_empty() {
            return;
        }

        match &event.kind {
            EventKind::Create(CreateKind::File) => {
                for path in paths {
                    let _ = tx.send(VaultEvent::Created(path));
                }
            }
            EventKind::Modify(ModifyKind::Data(_) | ModifyKind::Any) => {
                for path in paths {
                    let _ = tx.send(VaultEvent::Modified(path));
                }
            }
            EventKind::Remove(RemoveKind::File) => {
                for path in paths {
                    let _ = tx.send(VaultEvent::Deleted(path));
                }
            }
            EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                if paths.len() >= 2 {
                    let _ = tx.send(VaultEvent::Renamed {
                        from: paths[0].clone(),
                        to: paths[1].clone(),
                    });
                }
            }
            EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
                for path in paths {
                    let _ = tx.send(VaultEvent::Deleted(path));
                }
            }
            EventKind::Modify(ModifyKind::Name(RenameMode::To)) => {
                for path in paths {
                    let _ = tx.send(VaultEvent::Created(path));
                }
            }
            _ => {}
        }
    }

    /// Check if path is a markdown file
    fn is_markdown_file(path: &Path) -> bool {
        path.extension()
            .map(|e| e.to_string_lossy().to_lowercase() == "md")
            .unwrap_or(false)
    }

    /// Check if path contains hidden components
    fn is_hidden(path: &Path, vault_path: &Path) -> bool {
        path.strip_prefix(vault_path)
            .ok()
            .map(|rel| {
                rel.components()
                    .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
            })
            .unwrap_or(false)
    }

    /// Get the next event (non-blocking)
    pub fn try_recv(&self) -> Option<VaultEvent> {
        self.receiver.try_recv().ok()
    }

    /// Get all pending events
    pub fn drain_events(&self) -> Vec<VaultEvent> {
        let mut events = Vec::new();
        while let Some(event) = self.try_recv() {
            events.push(event);
        }
        events
    }

    /// Get vault path
    pub fn vault_path(&self) -> &Path {
        &self.vault_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_is_markdown_file() {
        assert!(VaultWatcher::is_markdown_file(Path::new("note.md")));
        assert!(VaultWatcher::is_markdown_file(Path::new("note.MD")));
        assert!(!VaultWatcher::is_markdown_file(Path::new("note.txt")));
        assert!(!VaultWatcher::is_markdown_file(Path::new("note")));
    }

    #[test]
    fn test_is_hidden() {
        let vault = Path::new("/vault");

        assert!(VaultWatcher::is_hidden(
            Path::new("/vault/.hidden/note.md"),
            vault
        ));
        assert!(VaultWatcher::is_hidden(
            Path::new("/vault/.git/config"),
            vault
        ));
        assert!(!VaultWatcher::is_hidden(Path::new("/vault/note.md"), vault));
        assert!(!VaultWatcher::is_hidden(
            Path::new("/vault/subdir/note.md"),
            vault
        ));
    }

    // Note: Integration tests for file watching are timing-dependent
    // and may be flaky. In production, use manual testing.
    #[test]
    fn test_watcher_creation() {
        let temp = TempDir::new().unwrap();
        let watcher = VaultWatcher::new(temp.path().to_path_buf());
        assert!(watcher.is_ok());
    }
}
