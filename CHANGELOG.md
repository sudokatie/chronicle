# Changelog

All notable changes to Chronicle will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-30

### Added

- **Editor**: CodeMirror 6 markdown editor with syntax highlighting
- **Wiki Links**: `[[double-bracket]]` syntax for linking notes
- **Wiki Link Plugin**: Custom highlighting and Cmd/Ctrl+Click navigation
- **Backlinks Panel**: Shows notes that link to the current note
- **Graph View**: D3 force-directed visualization of knowledge network
- **Full-Text Search**: SQLite FTS5 powered search across all notes
- **File Watching**: Automatic re-indexing when files change
- **Tags**: YAML frontmatter tag support with tag browser
- **Keyboard Shortcuts**: Cmd+N/S/G, Cmd+Shift+F
- **Settings Page**: Vault info and shortcuts reference
- **Dark Theme**: Default dark UI

### Technical

- Tauri 2.0 for cross-platform desktop app
- SvelteKit frontend with TypeScript
- Rust backend with SQLite database
- 26 unit tests for core functionality

## [Unreleased]

### Planned

- Light theme option
- Editor preferences (font size, line height)
- Quick open modal (fuzzy finder)
- Automatic link completion on [[
- Note templates
- Export to PDF/HTML
- Plugin system
