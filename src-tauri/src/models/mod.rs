//! Data models for Chronicle

use serde::{Deserialize, Serialize};

/// Information about the current vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultInfo {
    pub path: String,
    pub note_count: usize,
    pub is_open: bool,
}

/// Full note content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub path: String,
    pub title: String,
    pub content: String,
    pub word_count: i32,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub tags: Vec<String>,
}

/// Graph data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Node in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub word_count: i32,
}

/// Edge in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}
