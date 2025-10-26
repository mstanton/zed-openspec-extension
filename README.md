# Zed-OpenSpec Extension

A comprehensive Zed editor extension that integrates OpenSpec's spec-driven development workflow with LLM-powered code generation, real-time validation, and complete audit trails.

## 🚀 Quick Start

### macOS (Recommended for Intel Mac)
```bash
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension
./install-macos.sh  # macOS-optimized installer
# Then: Cmd+Shift+P → "zed: install dev extension" → select this directory
```

### Linux / Other Platforms
```bash
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension
./install.sh
# Then: Cmd+Shift+P → "zed: install dev extension" → select this directory
```

**📖 Full guides:** [INSTALL.md](INSTALL.md) • [macOS Troubleshooting](INSTALL_MACOS.md)

---

## What This Extension Does

- ✅ **Phase 1 (Current)**: OpenSpec CLI integration directly in Zed
- 🚧 **Phase 2**: Real-time spec validation with LSP
- 🚧 **Phase 3**: Multi-LLM code generation (Claude, GPT-4, Ollama)
- 🚧 **Phase 4**: Cryptographic audit trail for compliance
- 🚧 **Phase 5**: Visual workflow management UI
- 🚧 **Phase 6**: Spec coverage analysis

---

## Documentation

- **[INSTALL.md](INSTALL.md)** - Installation guide (start here!)
- **[README_EXTENSION.md](README_EXTENSION.md)** - User guide and commands
- **[BUILD.md](BUILD.md)** - Build instructions and troubleshooting
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[CHANGELOG.md](CHANGELOG.md)** - Version history
- **[IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md)** - Development progress

---

## Available Commands

Access via Zed command palette (`Cmd+Shift+P`):

- `openspec:init` - Initialize OpenSpec in workspace
- `openspec:new-proposal` - Create new change proposal
- `openspec:list-changes` - List all changes
- `openspec:archive-change` - Archive completed change
- `openspec:validate-file` - Validate spec file
- `openspec:view-audit` - View audit trail (coming in Phase 4)
- `openspec:show-coverage` - Show coverage (coming in Phase 6)
- `openspec:apply-change` - Generate code (coming in Phase 3)

---

## Requirements

- **Rust** 1.75+ (for building)
- **OpenSpec CLI** 1.0+ (for functionality)
- **Zed Editor** (any recent version)

---

## Project Status

**Current Version:** 0.1.0
**Phase:** 1 of 6 complete
**License:** MIT

See [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for detailed progress.

---

## Architecture

Built with:
- Rust → WebAssembly compilation
- Zed Extension API
- OpenSpec CLI integration
- Future: LSP, LLM providers, audit system

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- Pull request process

---

## Support

- **Issues:** [GitHub Issues](https://github.com/mstanton/zed-openspec-extension/issues)
- **Discussions:** [GitHub Discussions](https://github.com/mstanton/zed-openspec-extension/discussions)
- **OpenSpec Discord:** [Join community](https://discord.gg/YctCnvvshC)

---

## License

MIT License - see [LICENSE](LICENSE) for details

---

**Built with** [OpenSpec](https://github.com/Fission-AI/OpenSpec) • [Zed Editor](https://zed.dev) • [Rust](https://rust-lang.org)
