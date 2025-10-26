# Building the Zed-OpenSpec Extension

This guide explains how to build the extension from source.

## Prerequisites

### 1. Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-wasip1

# Verify installation
rustc --version
cargo --version
```

### 2. Install OpenSpec CLI

```bash
# Install via npm
npm install -g @fission-ai/openspec@latest

# Verify installation
openspec --version
```

### 3. Install Zed Editor

Download from [https://zed.dev](https://zed.dev)

## Building the Extension

### Development Build

```bash
# Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# Build for development (with debug symbols)
cargo build --target wasm32-wasip1

# The compiled extension will be at:
# target/wasm32-wasip1/debug/zed_openspec.wasm
```

### Release Build

```bash
# Build optimized release version
cargo build --release --target wasm32-wasip1

# The compiled extension will be at:
# target/wasm32-wasip1/release/zed_openspec.wasm
```

### Build Options

The release profile in `Cargo.toml` is optimized for size:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
strip = true         # Strip symbols
```

This produces a smaller WASM binary suitable for distribution.

## Installing in Zed

### Option 1: Dev Extension (for development)

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "zed: install dev extension"
4. Select the `zed-openspec-extension` directory
5. Zed will load the extension from your local directory

Any changes you make will require rebuilding and reloading Zed.

### Option 2: Published Extension (future)

Once published to the Zed extension registry:

1. Open Zed
2. Go to Extensions panel
3. Search for "OpenSpec"
4. Click "Install"

## Development Workflow

### Making Changes

1. Edit source files in `src/`
2. Rebuild the extension:
   ```bash
   cargo build --target wasm32-wasip1
   ```
3. Reload Zed to load the new version

### Running Tests

```bash
# Run unit tests
cargo test

# Run tests for a specific module
cargo test commands::proposal

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Check code without building
cargo check --target wasm32-wasip1

# Run clippy (linter)
cargo clippy --target wasm32-wasip1

# Format code
cargo fmt
```

## Troubleshooting

### WASM Target Not Found

If you get an error about `wasm32-wasip1` not being installed:

```bash
rustup target add wasm32-wasip1
```

### Build Fails with Missing Dependencies

Make sure you have a stable internet connection to download dependencies from crates.io:

```bash
# Update dependencies
cargo update

# Try building again
cargo build --target wasm32-wasip1
```

### Extension Not Loading in Zed

1. Check Zed's extension logs:
   - macOS: `~/Library/Logs/Zed/Zed.log`
   - Linux: `~/.local/share/zed/logs/Zed.log`

2. Verify the extension.toml is valid

3. Rebuild with verbose output:
   ```bash
   cargo build --target wasm32-wasip1 --verbose
   ```

### OpenSpec Commands Not Working

1. Verify OpenSpec CLI is installed:
   ```bash
   openspec --version
   ```

2. Check that OpenSpec is initialized in your workspace:
   ```bash
   ls openspec/
   ```

3. Run commands from terminal first to verify they work:
   ```bash
   cd your-project
   openspec init
   openspec proposal test-proposal
   ```

## Project Structure

```
zed-openspec-extension/
├── Cargo.toml              # Rust package manifest
├── extension.toml          # Zed extension metadata
├── src/
│   ├── lib.rs             # Extension entry point
│   ├── commands/          # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── proposal.rs
│   │   ├── apply.rs
│   │   ├── archive.rs
│   │   ├── list.rs
│   │   ├── audit.rs
│   │   ├── validate.rs
│   │   └── coverage.rs
│   └── utils/             # Utility modules
│       ├── mod.rs
│       ├── config.rs
│       ├── errors.rs
│       └── fs.rs
├── tests/                 # Integration tests
└── docs/                  # Documentation
```

## Next Steps

After building the extension:

1. See [README_EXTENSION.md](README_EXTENSION.md) for usage instructions
2. Read the [specification documents](.) for architecture details
3. Check the [issue tracker](https://github.com/mstanton/zed-openspec-extension/issues) for tasks to work on

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details
