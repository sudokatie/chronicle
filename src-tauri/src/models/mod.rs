//! Data models for Chronicle

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub vault: VaultConfig,
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub graph: GraphConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VaultConfig {
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
    #[serde(default = "default_true")]
    pub word_wrap: bool,
    #[serde(default)]
    pub vim_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    #[serde(default = "default_true")]
    pub physics_enabled: bool,
    #[serde(default = "default_link_distance")]
    pub link_distance: u32,
    #[serde(default = "default_charge_strength")]
    pub charge_strength: i32,
    #[serde(default = "default_node_size")]
    pub node_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_sidebar_width")]
    pub sidebar_width: u32,
    #[serde(default = "default_panel_width")]
    pub panel_width: u32,
    #[serde(default = "default_true")]
    pub show_backlinks: bool,
    #[serde(default = "default_true")]
    pub show_tags: bool,
}

// Default value functions
fn default_font_family() -> String { "JetBrains Mono".to_string() }
fn default_font_size() -> u32 { 14 }
fn default_line_height() -> f32 { 1.6 }
fn default_true() -> bool { true }
fn default_link_distance() -> u32 { 100 }
fn default_charge_strength() -> i32 { -300 }
fn default_node_size() -> u32 { 8 }
fn default_sidebar_width() -> u32 { 250 }
fn default_panel_width() -> u32 { 250 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            vault: VaultConfig::default(),
            editor: EditorConfig::default(),
            graph: GraphConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_family: default_font_family(),
            font_size: default_font_size(),
            line_height: default_line_height(),
            word_wrap: true,
            vim_mode: false,
        }
    }
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            physics_enabled: true,
            link_distance: default_link_distance(),
            charge_strength: default_charge_strength(),
            node_size: default_node_size(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            sidebar_width: default_sidebar_width(),
            panel_width: default_panel_width(),
            show_backlinks: true,
            show_tags: true,
        }
    }
}

impl AppConfig {
    /// Get the config file path
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("chronicle")
            .join("config.toml")
    }
    
    /// Load config from file, or return default
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => toml::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }
    
    /// Save config to file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).unwrap_or_default();
        fs::write(path, content)
    }
}
