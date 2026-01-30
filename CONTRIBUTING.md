# Contributing to Chronicle

Thanks for your interest in contributing to Chronicle!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/chronicle.git`
3. Install dependencies: `npm install`
4. Run in development: `npm run tauri dev`

## Development Requirements

- Node.js 18+
- Rust 1.70+
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Project Structure

```
chronicle/
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── api/           # Tauri API wrappers
│   │   ├── components/    # Svelte components
│   │   └── stores/        # Svelte stores
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Rust backend
│   └── src/
│       ├── commands/      # Tauri commands
│       ├── db/            # SQLite operations
│       ├── models/        # Data structures
│       └── vault/         # File watching, indexing
└── static/                # Static assets
```

## Running Tests

```bash
# Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# TypeScript type checking
npm run check
```

## Code Style

- **Rust**: Run `cargo fmt` and `cargo clippy` before committing
- **TypeScript/Svelte**: Run `npm run format` and `npm run lint`

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with clear commit messages
3. Ensure all tests pass
4. Update documentation if needed
5. Submit a PR with a description of your changes

## Reporting Issues

Please include:
- Chronicle version
- Operating system
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
