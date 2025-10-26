# Zed-OpenSpec Extension

A comprehensive Zed editor extension that integrates OpenSpec's spec-driven development workflow with LLM-powered code generation, real-time validation, and complete audit trails.

## Features

### ✅ Phase 1: Foundation (Current)

- **OpenSpec CLI Integration**: Execute all OpenSpec commands directly from Zed
- **Change Management**: Create, list, and archive OpenSpec changes
- **Basic Validation**: Preliminary spec validation
- **Configuration**: Extensible configuration system

### 🚧 Coming Soon

- **Phase 2**: Real-time LSP-based spec validation
- **Phase 3**: Multi-LLM code generation (Claude, GPT-4, Ollama)
- **Phase 4**: Complete cryptographic audit trail
- **Phase 5**: Visual workflow management UI
- **Phase 6**: Spec coverage analysis and visualization

## Installation

### Quick Install (Recommended)

```bash
# Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# Run the installation script (handles everything)
./install.sh

# Then in Zed: Cmd+Shift+P → "zed: install dev extension" → select this directory
```

**See [INSTALL.md](INSTALL.md) for detailed installation instructions and troubleshooting.**

### Manual Installation

**Prerequisites:**
1. **Rust** - [Install from rustup.rs](https://rustup.rs)
2. **OpenSpec CLI** - `npm install -g @fission-ai/openspec@latest`
3. **Zed Editor** - [Download from zed.dev](https://zed.dev)

**Build Steps:**

```bash
# 1. Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# 2. Add WASM target (IMPORTANT!)
rustup target add wasm32-wasip1

# 3. Build the extension (THIS STEP IS REQUIRED)
cargo build --release --target wasm32-wasip1

# 4. Install in Zed
# Open Zed → Cmd+Shift+P → "zed: install dev extension" → select this directory
```

**⚠️ Important:** You **must** build the extension before installing it in Zed. The extension is compiled to WebAssembly and cannot run without building first.

## Usage

### Commands

All commands are accessible via the Zed command palette (Cmd+Shift+P / Ctrl+Shift+P):

#### Initialize OpenSpec

```
openspec:init
```

Initializes OpenSpec in the current workspace. Creates the `openspec/` directory structure.

#### Create New Proposal

```
openspec:new-proposal
```

Creates a new change proposal. You'll be prompted for a name.

**Example:**
- Name: `add-authentication`
- Creates: `openspec/changes/add-authentication/`
  - `proposal.md` - Change description
  - `tasks.md` - Task list
  - `specs/` - Spec delta files

#### List Changes

```
openspec:list-changes
```

Shows all active OpenSpec changes in the workspace.

#### Apply Change (Coming in Phase 3)

```
openspec:apply-change
```

Generates code for a change using LLM. Select change and provider.

#### Archive Change

```
openspec:archive-change
```

Archives a completed change after all tasks are done.

#### Validate File

```
openspec:validate-file
```

Validates the current spec file for common issues.

#### View Audit Trail (Coming in Phase 4)

```
openspec:view-audit
```

Opens the audit trail viewer showing all code generations.

#### Show Coverage (Coming in Phase 6)

```
openspec:show-coverage
```

Displays spec coverage analysis for the project.

## Configuration

Configuration will be stored in `.openspec-config.json` at the workspace root:

```json
{
  "llm": {
    "default_provider": "claude",
    "providers": {
      "claude": {
        "model": "claude-sonnet-4-20250514",
        "api_key_env": "ANTHROPIC_API_KEY",
        "max_tokens": 8000
      },
      "gpt-4": {
        "model": "gpt-4-turbo",
        "api_key_env": "OPENAI_API_KEY",
        "max_tokens": 8000
      }
    },
    "fallback_chain": ["claude", "gpt-4"]
  },
  "validation": {
    "enabled": true,
    "debounce_ms": 500
  },
  "audit": {
    "enabled": true,
    "retention_days": 730
  }
}
```

## Development Workflow

### Typical OpenSpec Workflow in Zed

1. **Initialize** (once per project)
   ```
   openspec:init
   ```

2. **Create Proposal**
   ```
   openspec:new-proposal
   → Enter name: "add-user-profile"
   ```

3. **Edit Specs**
   - Open `openspec/changes/add-user-profile/proposal.md`
   - Add specs to `openspec/changes/add-user-profile/specs/`
   - Define tasks in `openspec/changes/add-user-profile/tasks.md`

4. **Validate** (Phase 2+)
   ```
   openspec:validate-file
   ```

5. **Generate Code** (Phase 3+)
   ```
   openspec:apply-change
   → Select change: add-user-profile
   → Select provider: claude
   ```

6. **Review & Accept**
   - Review generated code in diff view
   - Accept or reject changes
   - Audit entry automatically created

7. **Archive When Complete**
   ```
   openspec:archive-change
   → Select change: add-user-profile
   ```

## Architecture

```
┌─────────────────────────────────────────┐
│         Zed Editor                     │
│  ┌───────────────────────────────────┐  │
│  │  OpenSpec Extension (WASM)        │  │
│  │  ┌─────────────┐  ┌──────────┐   │  │
│  │  │ Commands    │  │ Config   │   │  │
│  │  └──────┬──────┘  └────────┬─┘   │  │
│  │         │                   │     │  │
│  │  ┌──────┴───────────────────┴──┐  │  │
│  │  │  OpenSpec CLI Integration  │  │  │
│  │  └────────────────────────────┘  │  │
│  └───────────────────────────────────┘  │
└──────────────┬──────────────────────────┘
               │
               ▼
        ┌──────────────┐
        │ OpenSpec CLI │
        └──────────────┘
```

## Future Features

### Phase 2: Real-time Validation
- LSP server for inline diagnostics
- As-you-type spec validation
- Quick fixes for common errors
- Custom validation rules

### Phase 3: LLM Code Generation
- Multi-provider support (Claude, GPT-4, Ollama)
- Streaming code generation
- Token estimation and cost calculation
- Automatic fallback chains

### Phase 4: Audit Trail
- Cryptographically signed audit entries
- Complete spec→code transformation tracking
- Fast querying (10,000+ entries)
- Export for compliance (SOC 2, GDPR)

### Phase 5: Workflow UI
- Sidebar panel for change management
- Visual status indicators
- Task progress tracking
- One-click apply/archive

### Phase 6: Coverage Analysis
- Line-level spec coverage
- File tree coverage indicators
- Project-wide metrics
- Coverage heat maps

## Contributing

See the main specification documents for detailed implementation guidance:

- `Zed_OpenSpec_Extension_Spec.md` - Complete product specification
- `Zed_OpenSpec_Technical_Implementation_Guide.md` - Technical patterns
- `QUICK_START.md` - Developer quick-start guide

## License

MIT License - see LICENSE file for details

## Support

- **Issues**: [GitHub Issues](https://github.com/mstanton/zed-openspec-extension/issues)
- **OpenSpec Discord**: [Join the community](https://discord.gg/YctCnvvshC)
- **Documentation**: See the `docs/` directory

## Acknowledgments

Built on top of:
- [Zed Editor](https://zed.dev)
- [OpenSpec](https://github.com/Fission-AI/OpenSpec)
- [Anthropic Claude](https://www.anthropic.com)
- [OpenAI](https://openai.com)

---

**Status**: Phase 1 Implementation Complete ✓

**Next**: Phase 2 - LSP Server & Real-time Validation
