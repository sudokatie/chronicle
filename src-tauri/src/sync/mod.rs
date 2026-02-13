//! Git sync module for Chronicle
//! 
//! Provides git-based synchronization between devices.
//! Notes are plain Markdown files, making git a natural transport.

pub mod git;
pub mod conflict;
pub mod status;

pub use git::{GitRepo, GitError};
pub use conflict::{ConflictInfo, ConflictResolution};
pub use status::SyncStatus;
