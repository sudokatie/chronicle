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
- **Git Sync** - Sync your vault across devices with git
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

**Global**
| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + N` | New note |
| `Cmd/Ctrl + O` | Quick open (fuzzy finder) |
| `Cmd/Ctrl + P` | Command palette |
| `Cmd/Ctrl + S` | Save note |
| `Cmd/Ctrl + G` | Toggle graph view |
| `Cmd/Ctrl + Shift + F` | Focus search |
| `Cmd/Ctrl + ,` | Open settings |

**Editor**
| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + E` | Toggle live preview |
| `Cmd/Ctrl + B` | Bold |
| `Cmd/Ctrl + I` | Italic |
| `Cmd/Ctrl + K` | Insert wiki link |
| `Cmd/Ctrl + ]` | Increase heading level |
| `Cmd/Ctrl + [` | Decrease heading level |
| `Cmd/Ctrl + Click` | Follow wiki-link |

**Graph View**
| Shortcut | Action |
|----------|--------|
| `Scroll` | Zoom in/out |
| `Drag` | Pan view |
| `Click` | Select node |
| `Double-click` | Open note |
| `Escape` | Return to editor |

### Git Sync

Chronicle can sync your vault across devices using git. Click "Set up sync" in the sidebar to get started.

**Initial Setup**
1. Click "Set up sync" in the sidebar
2. (Optional) Enter a remote URL for a git repository
3. Click "Initialize" to create a git repo in your vault

**Syncing**
- Click the sync button to push local changes and pull remote changes
- If conflicts occur, a modal will show both versions side-by-side
- Choose "Keep Local", "Keep Remote", or "Keep Both" to resolve

**Status Indicators**
| Icon | Meaning |
|------|---------|
| ✓ (green) | Synced, no changes |
| ↑↓ (yellow) | Changes pending |
| ! (red) | Conflicts need resolution |
| ○ (gray) | Not yet initialized |

**Behind the Scenes**
- Chronicle uses libgit2 for git operations
- Local changes are automatically staged and committed on sync
- Conflicts are detected during pull and presented for resolution

### Graph View

Click "Graph" in the sidebar to visualize your knowledge network:

- **Nodes** = notes (size based on word count)
- **Edges** = links between notes
- **Scroll** to zoom
- **Drag** to pan
- **Click a node** to open that note

### Plugins

Chronicle supports plugins to extend functionality. Three built-in plugins ship with the app:

**Word Count** - Shows word count, character count, and estimated reading time in the status bar.

**Pomodoro Timer** - A sidebar panel with a configurable work/break timer for focused writing sessions.

**Daily Notes** - Adds a command to quickly open or create today's note with a template.

#### Managing Plugins

Open Settings (`Cmd/Ctrl + ,`) and scroll to the Plugins section to:
- Enable/disable plugins
- Configure plugin settings
- View plugin permissions

#### Plugin Settings

Each plugin can have its own settings:

| Plugin | Setting | Default |
|--------|---------|---------|
| Word Count | Show character count | Yes |
| Word Count | Show reading time | Yes |
| Word Count | Words per minute | 200 |
| Pomodoro | Work duration | 25 min |
| Pomodoro | Break duration | 5 min |
| Pomodoro | Long break duration | 15 min |
| Pomodoro | Sessions before long break | 4 |
| Daily Notes | Template | `# {{date}}` + tasks/notes |
| Daily Notes | Date format | YYYY-MM-DD |
| Daily Notes | Folder | `daily` |

#### Plugin API

Plugins can:
- React to note events (open, change, save, close)
- Add sidebar panels
- Add status bar items
- Register commands with keyboard shortcuts
- Store persistent data
- Access plugin settings

For plugin development documentation, see [PLUGINS.md](docs/PLUGINS.md).

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
