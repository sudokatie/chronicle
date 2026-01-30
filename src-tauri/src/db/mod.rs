//! Database module for Chronicle
//! 
//! Handles SQLite database operations for note metadata,
//! full-text search, links, and tags.

pub mod schema;
pub mod notes;
pub mod links;
pub mod search;
pub mod tags;

pub use schema::{init_db, Database};
pub use notes::*;
pub use links::*;
pub use search::*;
pub use tags::*;
