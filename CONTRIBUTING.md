# Contributing to Zed-OpenSpec Extension

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## Development Setup

See [BUILD.md](BUILD.md) for detailed build instructions.

Quick start:
```bash
# Prerequisites
rustup target add wasm32-wasip1
npm install -g @fission-ai/openspec@latest

# Clone and build
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension
cargo build --target wasm32-wasip1
```

## Development Phases

The extension is being built in phases:

### ✅ Phase 1: Foundation (Complete)
- OpenSpec CLI integration
- Basic command handlers
- Configuration system
- Project scaffolding

### 🚧 Phase 2: LSP Server (In Progress)
- Real-time spec validation
- Language Server Protocol implementation
- Inline diagnostics
- Quick fixes

### 📋 Phase 3: LLM Integration (Planned)
- Multi-provider LLM support
- Code generation pipeline
- Streaming responses
- Fallback chains

### 📋 Phase 4: Audit Trail (Planned)
- Cryptographic signatures
- Immutable audit log
- Fast querying
- Compliance exports

### 📋 Phase 5: Workflow UI (Planned)
- Sidebar panels
- Visual indicators
- Task tracking
- Progress views

### 📋 Phase 6: Coverage Analysis (Planned)
- Line-level coverage
- File tree decorations
- Coverage metrics
- Heat maps

## How to Contribute

### Finding Tasks

1. Check the [GitHub Issues](https://github.com/mstanton/zed-openspec-extension/issues)
2. Look for issues labeled `good-first-issue` or `help-wanted`
3. Review the phase roadmap above for upcoming work
4. Read the specification documents for detailed requirements

### Before Starting Work

1. Comment on the issue to let others know you're working on it
2. Fork the repository
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Read the relevant specification sections

### Making Changes

1. **Write Tests**: Add tests for new functionality
   ```bash
   cargo test
   ```

2. **Follow Rust Conventions**:
   - Use `rustfmt` to format code:
     ```bash
     cargo fmt
     ```
   - Run clippy for lints:
     ```bash
     cargo clippy --target wasm32-wasip1
     ```

3. **Document Your Code**:
   - Add doc comments for public functions
   - Update README if adding user-facing features
   - Include examples where appropriate

4. **Keep Commits Clean**:
   - Write clear commit messages
   - One logical change per commit
   - Reference issues: "Fixes #123"

### Code Style

Follow standard Rust style guidelines:

```rust
// Good: Clear, documented, error handling
/// Handle OpenSpec init command
///
/// Initializes OpenSpec in the workspace directory.
/// Returns success message or error if CLI is not installed.
pub fn handle_init(workspace_path: &Path) -> Result<String> {
    let output = Command::new("openspec")
        .arg("init")
        .current_dir(workspace_path)
        .output()
        .context("Failed to execute openspec init")?;

    if output.status.success() {
        Ok("OpenSpec initialized".to_string())
    } else {
        Err(anyhow!("Init failed"))
    }
}

// Bad: No docs, poor error handling
pub fn init(path: &Path) -> String {
    Command::new("openspec")
        .arg("init")
        .current_dir(path)
        .output()
        .unwrap();
    "done".to_string()
}
```

### Submitting Pull Requests

1. **Update Your Branch**:
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Run Full Test Suite**:
   ```bash
   cargo test
   cargo clippy --target wasm32-wasip1
   cargo fmt --check
   ```

3. **Push to Your Fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create Pull Request**:
   - Use a clear title
   - Describe what changed and why
   - Reference related issues
   - Include screenshots for UI changes
   - Check the "Allow edits from maintainers" box

5. **PR Template**:
   ```markdown
   ## Summary
   Brief description of changes

   ## Changes
   - Added X
   - Fixed Y
   - Updated Z

   ## Testing
   - [ ] Unit tests added/updated
   - [ ] Manual testing completed
   - [ ] No clippy warnings

   ## Related Issues
   Fixes #123
   Related to #456

   ## Screenshots (if applicable)
   [Add screenshots here]
   ```

## Testing Guidelines

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_name_validation() {
        assert!(is_valid_proposal_name("valid-name"));
        assert!(!is_valid_proposal_name("invalid name"));
    }
}
```

### Integration Tests

Add integration tests to `tests/`:

```rust
// tests/command_integration.rs
#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();
    // Test init command
}
```

## Architecture Guidelines

### Module Organization

```
src/
├── lib.rs                 # Extension entry point
├── commands/              # Command handlers (one file per command)
├── llm/                   # LLM integration (Phase 3)
├── lsp/                   # LSP server (Phase 2)
├── audit/                 # Audit system (Phase 4)
├── workflow/              # Workflow management (Phase 5)
├── coverage/              # Coverage analysis (Phase 6)
├── ui/                    # UI components (Phase 5)
└── utils/                 # Shared utilities
```

### Error Handling

Always use `Result<T, E>` for fallible operations:

```rust
use anyhow::{Result, Context};

pub fn load_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;

    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}
```

### Configuration

Use the centralized config system:

```rust
use crate::utils::config::ExtensionConfig;

let config = ExtensionConfig::load_or_default();
let provider = &config.llm.default_provider;
```

## Documentation

### Code Documentation

Use doc comments for public APIs:

```rust
/// Generates code for an OpenSpec change using the specified LLM provider.
///
/// # Arguments
///
/// * `workspace_path` - Path to the workspace root
/// * `change_id` - ID of the change to apply
/// * `llm_provider` - LLM provider to use (e.g., "claude", "gpt-4")
/// * `config` - Extension configuration
///
/// # Returns
///
/// Success message or error description
///
/// # Errors
///
/// Returns an error if:
/// - Change not found
/// - LLM API fails
/// - Generated code is invalid
pub fn handle_apply_change(
    workspace_path: &Path,
    change_id: &str,
    llm_provider: &str,
    config: &ExtensionConfig,
) -> Result<String> {
    // Implementation
}
```

### User Documentation

Update README_EXTENSION.md when adding user-facing features.

## Review Process

1. **Automated Checks**: CI will run tests and lints
2. **Code Review**: Maintainer reviews code quality and architecture
3. **Testing**: Verify changes work as expected
4. **Merge**: Approved PRs are merged to main

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/mstanton/zed-openspec-extension/discussions)
- **Bugs**: File an [Issue](https://github.com/mstanton/zed-openspec-extension/issues)
- **Chat**: Join the [OpenSpec Discord](https://discord.gg/YctCnvvshC)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the problem, not the person
- Help others learn and grow

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md
- Release notes
- Extension credits

Thank you for contributing to Zed-OpenSpec! 🚀
