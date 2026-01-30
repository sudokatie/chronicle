//! Database module for Chronicle
//!
//! Handles SQLite database operations for note metadata,
//! full-text search, links, and tags.

pub mod links;
pub mod notes;
pub mod schema;
pub mod search;
pub mod tags;

pub use links::*;
pub use notes::*;
pub use schema::{init_db, Database};
pub use search::*;
pub use tags::*;
