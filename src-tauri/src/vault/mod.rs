//! Vault module for Chronicle
//!
//! Handles vault operations: parsing notes, indexing, file watching.

mod indexer;
mod parser;
mod watcher;

pub use indexer::*;
pub use parser::*;
pub use watcher::*;
