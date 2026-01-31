# Changelog

All notable changes to Chronicle will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2026-01-31

### Added

- **Live Preview**: Split-pane markdown preview with `Cmd+E` toggle. Renders headings, bold/italic, code blocks, wiki links (clickable), blockquotes, lists, and more.
- **Live Word Count**: Status bar now shows word and character count updating in real-time as you type.
- **Modified Indicator**: Visible amber dot in status bar when note has unsaved changes.

### Fixed

- Graph settings (link distance, charge strength, node size, physics toggle) now actually control the graph visualization instead of being ignored.
- Graph recreates automatically when settings change.
- Status bar word count now calculated from current content, not saved metadata.

## [0.1.5] - 2026-01-31

### Added

- **E2E Test Suite**: Playwright-based end-to-end tests covering:
  - Note creation and editing (7 tests)
  - Search and navigation (9 tests)
  - Graph view interaction (7 tests)
  - Keyboard shortcuts (10 tests)
  - 19 of 33 tests passing - foundation for continuous testing

### Fixed

- App now checks vault status on startup, enabling proper test mocking
- Tauri API wrapper gracefully handles missing Tauri context (browser-only or E2E tests)

### Technical

- Added Playwright config and test fixtures with Tauri mock injection
- Tests mock all Tauri commands with realistic data
- Added `checkVaultStatus()` to load vault info on app startup

## [0.1.4] - 2026-01-31

### Added

- **Tag Filter in File Browser**: Click "filter" next to any tag to filter the file tree to only show notes with that tag. Visual indicator shows active filter.
- **Vim Mode**: Fully wired up vim keybindings using @replit/codemirror-vim. Toggle in Settings.
- **Dynamic Editor Styling**: Font family, font size, line height from config now apply to the editor in real-time.
- **Configurable Panel Widths**: Sidebar and backlinks panel widths can be set in config.toml and apply on load.
- **Show/Hide Backlinks and Tags**: `show_backlinks` and `show_tags` settings control visibility of those panel sections.

### Fixed

- Editor preferences (font, size, line height, word wrap) now actually apply to CodeMirror instead of being stored but ignored.
- Vim mode toggle in settings now enables/disables vim keybindings live.
- Config store properly loads on app startup.

### Technical

- Added `config.ts` store with reactive updates to editor and UI components
- Added `tagFilter` and `filteredNotePaths` to vault store for tag filtering
- CodeMirror now uses compartments for live config updates (vim mode, theme)

## [0.1.3] - 2026-01-30

### Added

- **Editable Tags**: Click "Edit" in the metadata panel to add/remove tags from notes
- **TOML Config**: Settings now persist to `~/.config/chronicle/config.toml` instead of localStorage
- **Font Family**: Choose from JetBrains Mono, Fira Code, Monaco, Menlo, Consolas, or system monospace
- **Vim Mode**: Optional vim keybindings in the editor (enable in settings)
- **Graph Physics Settings**: Configure link distance, charge strength, and node size
- **Graph Toggle**: Enable/disable physics simulation

### Technical

- Added `update_note_tags` command for frontmatter manipulation
- Added `get_config` and `save_config` commands for TOML persistence
- Config stored at platform-appropriate location (e.g., `~/.config/chronicle/config.toml` on macOS/Linux)

## [0.1.2] - 2026-01-30

### Added

- **Link Autocomplete**: Type `[[` to trigger autocomplete dropdown with all notes
- **Backlinks Context**: Backlinks panel now shows surrounding text from source notes
- **File Watcher Events**: Real-time updates when notes are created/modified/deleted externally
- **Graph Double-Click**: Double-click a node to open the note (single-click selects)
- **Graph Escape**: Press Escape in graph view to return to editor

### Fixed

- Link autocomplete now properly wired up using CodeMirror autocompletion
- File watcher events now emit to frontend via poll_vault_events command
- Graph view interaction matches spec (double-click to open, Escape to return)

## [0.1.1] - 2026-01-30

### Added

- **Quick Open** (`Cmd+O`): Fuzzy finder for notes with instant search
- **Command Palette** (`Cmd+P`): Access all commands from one place
- **Editor Formatting**: Bold (`Cmd+B`), Italic (`Cmd+I`), Wiki link (`Cmd+K`)
- **Heading Shortcuts**: Increase (`Cmd+]`) and decrease (`Cmd+[`) heading levels
- **Settings Shortcut** (`Cmd+,`): Quick access to settings
- **Note Metadata Panel**: Shows created date, modified date, word count, tags
- **Graph Tag Filter**: Filter the knowledge graph by tag
- **Editor Preferences**: Font size, line height, word wrap settings
- **CONTRIBUTING.md**: Guidelines for contributors

### Fixed

- Backlinks panel now shows display text context
- README now accurately documents all shortcuts

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
- Automatic link completion on [[
- Note templates
- Export to PDF/HTML
- Plugin system
