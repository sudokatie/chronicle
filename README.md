# Chronicle

Personal knowledge graph that grows as you write. Your second brain, visualized.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-yellow.svg)](https://tauri.app)

## Features

- **Markdown Editor** - CodeMirror 6 with syntax highlighting and live preview
- **Wiki-Style Links** - Connect notes with `[[double-bracket]]` syntax
- **Backlinks Panel** - See what links to your current note
- **Knowledge Graph** - D3-powered force-directed visualization
- **Full-Text Search** - Fast SQLite FTS5 search across all notes
- **File-Based Storage** - Plain Markdown files, git-friendly
- **Keyboard Shortcuts** - Efficient workflow with Cmd/Ctrl shortcuts
- **Dark Theme** - Easy on the eyes

## Screenshots

*Coming soon*

## Installation

### macOS

The built application is available at:
```
src-tauri/target/release/bundle/macos/chronicle.app
```

Drag `chronicle.app` to your Applications folder to install.

### Download (when available)

Releases will be available at:

- [macOS (.dmg)](https://github.com/sudokatie/chronicle/releases)
- [Windows (.msi)](https://github.com/sudokatie/chronicle/releases)
- [Linux (.AppImage)](https://github.com/sudokatie/chronicle/releases)

### Build from Source

Requirements:
- Node.js 18+
- Rust 1.70+
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

```bash
# Clone the repository
git clone https://github.com/sudokatie/chronicle.git
cd chronicle

# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

### Getting Started

1. Launch Chronicle
2. Click "Open Vault" and select a folder containing your Markdown notes
3. Chronicle will index all `.md` files in that folder

### Creating Notes

- Click the "+" button in the sidebar, or
- Press `Cmd/Ctrl + N`
- Enter a title and start writing

### Linking Notes

Type `[[` to start a link, then enter the note name:

```markdown
This links to [[another-note]].
You can also use [[note-name|display text]].
```

`Cmd/Ctrl + Click` on a link to navigate to that note.

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + N` | New note |
| `Cmd/Ctrl + S` | Save note |
| `Cmd/Ctrl + G` | Toggle graph view |
| `Cmd/Ctrl + Shift + F` | Focus search |
| `Cmd/Ctrl + Click` | Follow wiki-link |

### Graph View

Click "Graph" in the sidebar to visualize your knowledge network:

- **Nodes** = notes (size based on word count)
- **Edges** = links between notes
- **Scroll** to zoom
- **Drag** to pan
- **Click a node** to open that note

## Architecture

- **Frontend**: SvelteKit + TailwindCSS + CodeMirror 6 + D3.js
- **Backend**: Tauri 2.0 (Rust)
- **Database**: SQLite with FTS5 for full-text search
- **Storage**: Plain Markdown files in your chosen directory

## Development

```bash
# Run development server
npm run tauri dev

# Run tests
cargo test --manifest-path src-tauri/Cargo.toml

# Type check
npm run check

# Format code
cargo fmt --manifest-path src-tauri/Cargo.toml
npm run format
```

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

Built by [Katie](https://blackabee.com) with love and Rust.
