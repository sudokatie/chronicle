//! Graph data commands

use std::sync::Mutex;
use tauri::State;

use crate::commands::vault::AppState;
use crate::db::{notes::list_notes, links::get_outlinks};
use crate::error::ChronicleError;
use crate::models::{GraphData, GraphEdge, GraphNode};

/// Get graph data for visualization
#[tauri::command]
pub async fn get_graph_data(
    state: State<'_, Mutex<AppState>>,
) -> Result<GraphData, ChronicleError> {
    let app_state = state.lock().expect("Failed to lock state");
    let db = app_state.db.as_ref().ok_or(ChronicleError::NoVaultOpen)?;
    let conn = db.conn();
    
    // Get all notes as nodes
    let notes = list_notes(&conn)?;
    let nodes: Vec<GraphNode> = notes.iter().map(|n| GraphNode {
        id: n.path.clone(),
        title: n.title.clone(),
        word_count: n.word_count,
    }).collect();
    
    // Get all edges
    let mut edges = Vec::new();
    for note in &notes {
        let links = get_outlinks(&conn, note.id)?;
        for link in links {
            // Only add edge if target exists
            if notes.iter().any(|n| {
                n.path == link.target_path 
                || n.path == format!("{}.md", link.target_path)
            }) {
                edges.push(GraphEdge {
                    source: note.path.clone(),
                    target: link.target_path,
                });
            }
        }
    }
    
    Ok(GraphData { nodes, edges })
}
