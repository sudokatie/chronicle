//! Markdown parser for extracting links, frontmatter, and metadata

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Parsed note with extracted metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNote {
    pub title: String,
    pub frontmatter: Option<Frontmatter>,
    pub links: Vec<ExtractedLink>,
    pub word_count: usize,
    pub content: String,
}

/// YAML frontmatter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Extracted wiki-style link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLink {
    pub target: String,
    pub display: Option<String>,
    pub line_number: usize,
}

// Regex patterns
static WIKI_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").expect("Invalid wiki link regex")
});

static FRONTMATTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)^---\r?\n(.+?)\r?\n---\r?\n?").expect("Invalid frontmatter regex")
});

static HEADING_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s+(.+)$").expect("Invalid heading regex"));

/// Parse a Markdown note
pub fn parse_note(content: &str, filename: &str) -> ParsedNote {
    let (frontmatter, body_start) = parse_frontmatter(content);
    let body = &content[body_start..];

    let title = determine_title(&frontmatter, body, filename);
    let links = extract_links(content);
    let word_count = count_words(body);

    ParsedNote {
        title,
        frontmatter,
        links,
        word_count,
        content: content.to_string(),
    }
}

/// Parse YAML frontmatter from content
fn parse_frontmatter(content: &str) -> (Option<Frontmatter>, usize) {
    if let Some(captures) = FRONTMATTER_RE.captures(content) {
        let yaml_str = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let frontmatter: Option<Frontmatter> = serde_yaml::from_str(yaml_str).ok();
        let body_start = captures.get(0).map(|m| m.end()).unwrap_or(0);
        (frontmatter, body_start)
    } else {
        (None, 0)
    }
}

/// Determine title from frontmatter, first heading, or filename
fn determine_title(frontmatter: &Option<Frontmatter>, body: &str, filename: &str) -> String {
    // Try frontmatter title
    if let Some(fm) = frontmatter {
        if let Some(title) = &fm.title {
            if !title.is_empty() {
                return title.clone();
            }
        }
    }

    // Try first heading
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(captures) = HEADING_RE.captures(trimmed) {
            if let Some(title) = captures.get(1) {
                return title.as_str().to_string();
            }
        }
        // Skip empty lines at start
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            break;
        }
    }

    // Fallback to filename without extension
    filename.strip_suffix(".md").unwrap_or(filename).to_string()
}

/// Extract wiki-style links from content
pub fn extract_links(content: &str) -> Vec<ExtractedLink> {
    let mut links = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        for captures in WIKI_LINK_RE.captures_iter(line) {
            let target = captures
                .get(1)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            let display = captures.get(2).map(|m| m.as_str().trim().to_string());

            if !target.is_empty() {
                links.push(ExtractedLink {
                    target,
                    display,
                    line_number: line_num + 1, // 1-indexed
                });
            }
        }
    }

    links
}

/// Count words in text (simple whitespace split)
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note_simple() {
        let content = "# Hello World\n\nThis is a test note.";
        let parsed = parse_note(content, "test.md");

        assert_eq!(parsed.title, "Hello World");
        // Word count includes heading: # Hello World This is a test note.
        assert_eq!(parsed.word_count, 8);
        assert!(parsed.links.is_empty());
    }

    #[test]
    fn test_parse_note_with_frontmatter() {
        let content = r#"---
title: My Custom Title
tags:
  - rust
  - programming
---

# Heading

Content here."#;

        let parsed = parse_note(content, "test.md");

        assert_eq!(parsed.title, "My Custom Title");
        assert!(parsed.frontmatter.is_some());

        let fm = parsed.frontmatter.unwrap();
        assert_eq!(fm.tags, vec!["rust", "programming"]);
    }

    #[test]
    fn test_parse_note_title_fallback() {
        let content = "Just some text without a heading.";
        let parsed = parse_note(content, "my-note.md");

        assert_eq!(parsed.title, "my-note");
    }

    #[test]
    fn test_extract_links() {
        let content =
            "Link to [[other-note]] and [[another|display text]].\nSecond line with [[third]].";
        let links = extract_links(content);

        assert_eq!(links.len(), 3);

        assert_eq!(links[0].target, "other-note");
        assert!(links[0].display.is_none());
        assert_eq!(links[0].line_number, 1);

        assert_eq!(links[1].target, "another");
        assert_eq!(links[1].display, Some("display text".to_string()));
        assert_eq!(links[1].line_number, 1);

        assert_eq!(links[2].target, "third");
        assert_eq!(links[2].line_number, 2);
    }

    #[test]
    fn test_extract_links_empty() {
        let content = "No links here.";
        let links = extract_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_word_count() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one two three four five"), 5);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   whitespace   "), 1);
    }

    #[test]
    fn test_frontmatter_parsing() {
        let content = "---\ncreated: 2024-01-01\nmodified: 2024-01-02\n---\nBody";
        let (fm, body_start) = parse_frontmatter(content);

        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.created, Some("2024-01-01".to_string()));
        assert_eq!(fm.modified, Some("2024-01-02".to_string()));
        assert!(body_start > 0);
    }
}
