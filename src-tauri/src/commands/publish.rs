//! Web publish commands - export notes as static website.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use tauri::command;

use crate::error::Result;
use crate::vault::Vault;

/// Configuration for web publishing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishConfig {
    /// Output directory for the generated site.
    pub output_dir: PathBuf,
    /// Site title.
    pub site_title: String,
    /// Site description.
    pub site_description: String,
    /// Include search functionality.
    pub include_search: bool,
    /// Include backlink graph.
    pub include_graph: bool,
    /// Custom CSS (optional).
    pub custom_css: Option<String>,
    /// Notes to exclude (glob patterns).
    pub exclude_patterns: Vec<String>,
}

impl Default for PublishConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("_site"),
            site_title: "My Notes".to_string(),
            site_description: "A personal knowledge base".to_string(),
            include_search: true,
            include_graph: true,
            custom_css: None,
            exclude_patterns: vec![],
        }
    }
}

/// A published note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedNote {
    /// URL slug for the note.
    pub slug: String,
    /// Original filename.
    pub filename: String,
    /// Note title.
    pub title: String,
    /// HTML content.
    pub html_content: String,
    /// Outgoing links.
    pub outgoing_links: Vec<String>,
    /// Incoming links (backlinks).
    pub backlinks: Vec<String>,
    /// Tags.
    pub tags: Vec<String>,
}

/// Backlink graph data for visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklinkGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// A node in the backlink graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub link_count: usize,
}

/// An edge in the backlink graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

/// Search index entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntry {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// Static site generator.
pub struct SiteGenerator {
    config: PublishConfig,
    vault_path: PathBuf,
}

impl SiteGenerator {
    /// Create a new site generator.
    pub fn new(config: PublishConfig, vault_path: PathBuf) -> Self {
        Self { config, vault_path }
    }

    /// Generate the static site.
    pub fn generate(&self) -> Result<()> {
        // Create output directory
        let output = self.vault_path.join(&self.config.output_dir);
        fs::create_dir_all(&output)?;

        // Collect all markdown files
        let notes = self.collect_notes()?;

        // Build backlink index
        let backlinks = self.build_backlink_index(&notes);

        // Generate HTML for each note
        let published: Vec<PublishedNote> = notes
            .iter()
            .map(|(path, content)| self.publish_note(path, content, &backlinks))
            .collect();

        // Write note pages
        for note in &published {
            let html = self.render_note_page(note);
            let path = output.join(format!("{}.html", note.slug));
            fs::write(path, html)?;
        }

        // Generate index page
        let index_html = self.render_index_page(&published);
        fs::write(output.join("index.html"), index_html)?;

        // Generate search index if enabled
        if self.config.include_search {
            let search_data = self.build_search_index(&published);
            let search_json = serde_json::to_string(&search_data)?;
            fs::write(output.join("search-index.json"), search_json)?;
            fs::write(output.join("search.js"), SEARCH_JS)?;
        }

        // Generate graph data if enabled
        if self.config.include_graph {
            let graph = self.build_graph(&published);
            let graph_json = serde_json::to_string(&graph)?;
            fs::write(output.join("graph.json"), graph_json)?;
            fs::write(output.join("graph.html"), self.render_graph_page())?;
        }

        // Write CSS
        let css = self.config.custom_css.as_deref().unwrap_or(DEFAULT_CSS);
        fs::write(output.join("style.css"), css)?;

        Ok(())
    }

    /// Collect all markdown notes from the vault.
    fn collect_notes(&self) -> Result<Vec<(PathBuf, String)>> {
        let mut notes = Vec::new();

        for entry in fs::read_dir(&self.vault_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "md").unwrap_or(false) {
                // Check exclude patterns
                let filename = path.file_name().unwrap().to_string_lossy();
                if self.should_exclude(&filename) {
                    continue;
                }

                let content = fs::read_to_string(&path)?;
                notes.push((path, content));
            }
        }

        Ok(notes)
    }

    /// Check if a file should be excluded.
    fn should_exclude(&self, filename: &str) -> bool {
        for pattern in &self.config.exclude_patterns {
            if filename.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Build an index of backlinks.
    fn build_backlink_index(&self, notes: &[(PathBuf, String)]) -> HashMap<String, Vec<String>> {
        let mut backlinks: HashMap<String, Vec<String>> = HashMap::new();

        for (path, content) in notes {
            let source = self.path_to_slug(path);
            let links = extract_wiki_links(content);

            for link in links {
                backlinks
                    .entry(link)
                    .or_default()
                    .push(source.clone());
            }
        }

        backlinks
    }

    /// Publish a single note.
    fn publish_note(
        &self,
        path: &Path,
        content: &str,
        backlinks: &HashMap<String, Vec<String>>,
    ) -> PublishedNote {
        let slug = self.path_to_slug(path);
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let title = extract_title(content).unwrap_or_else(|| slug.clone());
        let outgoing_links = extract_wiki_links(content);
        let tags = extract_tags(content);

        // Convert markdown to HTML with wiki links
        let html_content = self.render_markdown(content);

        let note_backlinks = backlinks
            .get(&slug)
            .cloned()
            .unwrap_or_default();

        PublishedNote {
            slug,
            filename,
            title,
            html_content,
            outgoing_links,
            backlinks: note_backlinks,
            tags,
        }
    }

    /// Convert a path to a URL slug.
    fn path_to_slug(&self, path: &Path) -> String {
        path.file_stem()
            .unwrap()
            .to_string_lossy()
            .to_lowercase()
            .replace(' ', "-")
    }

    /// Render markdown content to HTML.
    fn render_markdown(&self, content: &str) -> String {
        // Convert wiki links to HTML links
        let content = convert_wiki_links(content);
        
        let parser = Parser::new(&content);
        let mut html = String::new();
        html::push_html(&mut html, parser);
        html
    }

    /// Render a note page.
    fn render_note_page(&self, note: &PublishedNote) -> String {
        let backlinks_html = if note.backlinks.is_empty() {
            String::new()
        } else {
            let links: Vec<String> = note
                .backlinks
                .iter()
                .map(|b| format!("<li><a href=\"{}.html\">{}</a></li>", b, b))
                .collect();
            format!(
                "<div class=\"backlinks\"><h3>Backlinks</h3><ul>{}</ul></div>",
                links.join("")
            )
        };

        let tags_html = if note.tags.is_empty() {
            String::new()
        } else {
            let tags: Vec<String> = note
                .tags
                .iter()
                .map(|t| format!("<span class=\"tag\">#{}</span>", t))
                .collect();
            format!("<div class=\"tags\">{}</div>", tags.join(" "))
        };

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title} - {site_title}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <nav>
        <a href="index.html">Home</a>
        {graph_link}
    </nav>
    <main>
        <article>
            <h1>{title}</h1>
            {tags}
            {content}
            {backlinks}
        </article>
    </main>
</body>
</html>"#,
            title = note.title,
            site_title = self.config.site_title,
            graph_link = if self.config.include_graph {
                "<a href=\"graph.html\">Graph</a>"
            } else {
                ""
            },
            tags = tags_html,
            content = note.html_content,
            backlinks = backlinks_html,
        )
    }

    /// Render the index page.
    fn render_index_page(&self, notes: &[PublishedNote]) -> String {
        let notes_html: Vec<String> = notes
            .iter()
            .map(|n| {
                format!(
                    "<li><a href=\"{}.html\">{}</a></li>",
                    n.slug, n.title
                )
            })
            .collect();

        let search_html = if self.config.include_search {
            r#"<div class="search">
                <input type="text" id="search-input" placeholder="Search notes...">
                <ul id="search-results"></ul>
            </div>
            <script src="search.js"></script>"#
        } else {
            ""
        };

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{site_title}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>{site_title}</h1>
        <p>{description}</p>
    </header>
    <main>
        {search}
        <section class="notes-list">
            <h2>All Notes</h2>
            <ul>
                {notes}
            </ul>
        </section>
    </main>
</body>
</html>"#,
            site_title = self.config.site_title,
            description = self.config.site_description,
            search = search_html,
            notes = notes_html.join("\n"),
        )
    }

    /// Build search index.
    fn build_search_index(&self, notes: &[PublishedNote]) -> Vec<SearchEntry> {
        notes
            .iter()
            .map(|n| SearchEntry {
                slug: n.slug.clone(),
                title: n.title.clone(),
                content: strip_html(&n.html_content),
                tags: n.tags.clone(),
            })
            .collect()
    }

    /// Build graph data.
    fn build_graph(&self, notes: &[PublishedNote]) -> BacklinkGraph {
        let mut nodes: Vec<GraphNode> = Vec::new();
        let mut edges: Vec<GraphEdge> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for note in notes {
            if !seen.contains(&note.slug) {
                nodes.push(GraphNode {
                    id: note.slug.clone(),
                    label: note.title.clone(),
                    link_count: note.outgoing_links.len() + note.backlinks.len(),
                });
                seen.insert(note.slug.clone());
            }

            for link in &note.outgoing_links {
                edges.push(GraphEdge {
                    source: note.slug.clone(),
                    target: link.clone(),
                });
            }
        }

        BacklinkGraph { nodes, edges }
    }

    /// Render the graph visualization page.
    fn render_graph_page(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Graph - {site_title}</title>
    <link rel="stylesheet" href="style.css">
    <script src="https://d3js.org/d3.v7.min.js"></script>
</head>
<body>
    <nav>
        <a href="index.html">Home</a>
    </nav>
    <main>
        <h1>Knowledge Graph</h1>
        <div id="graph"></div>
    </main>
    <script>
        fetch('graph.json')
            .then(r => r.json())
            .then(data => {{
                const width = 800;
                const height = 600;
                
                const svg = d3.select('#graph')
                    .append('svg')
                    .attr('width', width)
                    .attr('height', height);
                
                const simulation = d3.forceSimulation(data.nodes)
                    .force('link', d3.forceLink(data.edges).id(d => d.id))
                    .force('charge', d3.forceManyBody().strength(-100))
                    .force('center', d3.forceCenter(width / 2, height / 2));
                
                const link = svg.selectAll('line')
                    .data(data.edges)
                    .enter().append('line')
                    .style('stroke', '#999');
                
                const node = svg.selectAll('circle')
                    .data(data.nodes)
                    .enter().append('circle')
                    .attr('r', d => 5 + d.link_count)
                    .style('fill', '#69b3a2')
                    .on('click', (e, d) => window.location.href = d.id + '.html');
                
                const label = svg.selectAll('text')
                    .data(data.nodes)
                    .enter().append('text')
                    .text(d => d.label)
                    .style('font-size', '10px');
                
                simulation.on('tick', () => {{
                    link
                        .attr('x1', d => d.source.x)
                        .attr('y1', d => d.source.y)
                        .attr('x2', d => d.target.x)
                        .attr('y2', d => d.target.y);
                    
                    node
                        .attr('cx', d => d.x)
                        .attr('cy', d => d.y);
                    
                    label
                        .attr('x', d => d.x + 10)
                        .attr('y', d => d.y + 3);
                }});
            }});
    </script>
</body>
</html>"#,
            site_title = self.config.site_title,
        )
    }
}

/// Extract wiki links [[like this]] from content.
fn extract_wiki_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next(); // consume second [
            let mut link = String::new();
            while let Some(c) = chars.next() {
                if c == ']' && chars.peek() == Some(&']') {
                    chars.next(); // consume second ]
                    if !link.is_empty() {
                        // Handle aliases: [[target|alias]]
                        let target = link.split('|').next().unwrap().to_string();
                        links.push(target.to_lowercase().replace(' ', "-"));
                    }
                    break;
                }
                link.push(c);
            }
        }
    }

    links
}

/// Convert wiki links to HTML links.
fn convert_wiki_links(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next();
            let mut link = String::new();
            while let Some(c) = chars.next() {
                if c == ']' && chars.peek() == Some(&']') {
                    chars.next();
                    // Handle aliases
                    let parts: Vec<&str> = link.split('|').collect();
                    let target = parts[0].to_lowercase().replace(' ', "-");
                    let display = parts.last().unwrap();
                    result.push_str(&format!("[{}]({}.html)", display, target));
                    break;
                }
                link.push(c);
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Extract the title from markdown content (first # heading).
fn extract_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return Some(trimmed[2..].to_string());
        }
    }
    None
}

/// Extract tags from content (#tag format).
fn extract_tags(content: &str) -> Vec<String> {
    let mut tags = Vec::new();
    for word in content.split_whitespace() {
        if word.starts_with('#') && word.len() > 1 {
            let tag = word[1..].trim_end_matches(|c: char| !c.is_alphanumeric());
            if !tag.is_empty() {
                tags.push(tag.to_string());
            }
        }
    }
    tags.sort();
    tags.dedup();
    tags
}

/// Strip HTML tags from content.
fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    result
}

const DEFAULT_CSS: &str = r#"
:root {
    --bg: #1a1a2e;
    --fg: #eee;
    --accent: #e94560;
    --link: #0f3460;
}

* { box-sizing: border-box; }

body {
    font-family: system-ui, -apple-system, sans-serif;
    background: var(--bg);
    color: var(--fg);
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    line-height: 1.6;
}

nav {
    margin-bottom: 2rem;
}

nav a {
    color: var(--accent);
    margin-right: 1rem;
}

h1, h2, h3 { color: var(--accent); }

a { color: var(--accent); }

.tag {
    display: inline-block;
    background: var(--link);
    padding: 0.2rem 0.5rem;
    border-radius: 3px;
    margin-right: 0.5rem;
    font-size: 0.9em;
}

.backlinks {
    margin-top: 3rem;
    padding-top: 1rem;
    border-top: 1px solid #333;
}

.search input {
    width: 100%;
    padding: 0.5rem;
    margin-bottom: 1rem;
    background: #16213e;
    border: 1px solid #333;
    color: var(--fg);
}

#graph {
    width: 100%;
    height: 600px;
}

article code {
    background: #16213e;
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
}

article pre {
    background: #16213e;
    padding: 1rem;
    overflow-x: auto;
    border-radius: 5px;
}
"#;

const SEARCH_JS: &str = r#"
let searchIndex = [];

fetch('search-index.json')
    .then(r => r.json())
    .then(data => { searchIndex = data; });

document.getElementById('search-input').addEventListener('input', function(e) {
    const query = e.target.value.toLowerCase();
    const results = document.getElementById('search-results');
    results.innerHTML = '';
    
    if (query.length < 2) return;
    
    const matches = searchIndex.filter(note => 
        note.title.toLowerCase().includes(query) ||
        note.content.toLowerCase().includes(query) ||
        note.tags.some(t => t.toLowerCase().includes(query))
    ).slice(0, 10);
    
    matches.forEach(note => {
        const li = document.createElement('li');
        const a = document.createElement('a');
        a.href = note.slug + '.html';
        a.textContent = note.title;
        li.appendChild(a);
        results.appendChild(li);
    });
});
"#;

/// Publish vault as static website.
#[command]
pub async fn publish_vault(
    vault_path: String,
    config: PublishConfig,
) -> Result<String> {
    let generator = SiteGenerator::new(config, PathBuf::from(&vault_path));
    generator.generate()?;
    Ok(format!("Published to {}", vault_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wiki_links() {
        let content = "See [[Note One]] and [[Note Two|display text]].";
        let links = extract_wiki_links(content);
        assert_eq!(links, vec!["note-one", "note-two"]);
    }

    #[test]
    fn test_extract_wiki_links_empty() {
        let content = "No links here.";
        let links = extract_wiki_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_convert_wiki_links() {
        let content = "See [[My Note]] for details.";
        let result = convert_wiki_links(content);
        assert!(result.contains("[My Note](my-note.html)"));
    }

    #[test]
    fn test_convert_wiki_links_with_alias() {
        let content = "See [[Target|display]] for details.";
        let result = convert_wiki_links(content);
        assert!(result.contains("[display](target.html)"));
    }

    #[test]
    fn test_extract_title() {
        let content = "# My Title\n\nSome content.";
        let title = extract_title(content);
        assert_eq!(title, Some("My Title".to_string()));
    }

    #[test]
    fn test_extract_title_none() {
        let content = "No heading here.";
        let title = extract_title(content);
        assert!(title.is_none());
    }

    #[test]
    fn test_extract_tags() {
        let content = "Hello #tag1 world #tag2 and #tag1 again.";
        let tags = extract_tags(content);
        assert_eq!(tags, vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_strip_html() {
        let html = "<p>Hello <strong>world</strong>!</p>";
        let text = strip_html(html);
        assert_eq!(text, "Hello world!");
    }

    #[test]
    fn test_publish_config_default() {
        let config = PublishConfig::default();
        assert!(config.include_search);
        assert!(config.include_graph);
        assert_eq!(config.site_title, "My Notes");
    }
}
