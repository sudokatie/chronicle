//! Vault module for Chronicle
//! 
//! Handles vault operations: parsing notes, indexing, file watching.

mod parser;
mod indexer;
mod watcher;

pub use parser::*;
pub use indexer::*;
pub use watcher::*;
