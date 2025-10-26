# Changelog

All notable changes to the Zed-OpenSpec extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase 2: LSP Server (Planned)
- Real-time spec validation via Language Server Protocol
- Inline diagnostics for spec errors
- Quick fixes for common issues
- Custom validation rules

### Phase 3: LLM Integration (Planned)
- Multi-provider LLM support (Claude, GPT-4, Ollama)
- Streaming code generation
- Token estimation and cost calculation
- Automatic fallback chains
- Context window management

### Phase 4: Audit Trail (Planned)
- Cryptographically signed audit entries (Ed25519)
- Immutable audit log storage
- Fast querying (10,000+ entries < 1s)
- Export for compliance (JSON, CSV)
- Integrity verification

### Phase 5: Workflow UI (Planned)
- Sidebar panel for change management
- Visual status indicators
- Task progress tracking
- Real-time workflow updates
- One-click apply/archive actions

### Phase 6: Coverage Analysis (Planned)
- Line-level spec coverage calculation
- File tree coverage decorations
- Project-wide coverage metrics
- Coverage heat maps
- Spec→code mapping visualization

## [0.1.0] - 2025-10-26

### Added - Phase 1: Foundation

#### Core Extension
- Extension scaffolding with Zed Extension API
- Command handler system
- Configuration management
- Error handling utilities
- File system helpers

#### OpenSpec Integration
- `openspec:init` - Initialize OpenSpec in workspace
- `openspec:new-proposal` - Create new change proposal
- `openspec:list-changes` - List all changes
- `openspec:archive-change` - Archive completed change
- `openspec:validate-file` - Basic spec validation
- `openspec:view-audit` - Audit trail viewer (placeholder)
- `openspec:show-coverage` - Coverage analysis (placeholder)
- `openspec:apply-change` - Code generation (placeholder)

#### Configuration
- Extensible configuration system
- LLM provider configuration
- Validation settings
- Audit settings
- Coverage settings
- Default configuration with Claude, GPT-4, Ollama

#### Documentation
- Comprehensive README
- Build instructions
- Contributing guidelines
- Complete specification documents
- Technical implementation guide
- Developer quick-start guide

#### Testing
- Basic integration tests
- Proposal name validation tests
- Test infrastructure setup

### Technical Details
- Built with Rust, compiled to WebAssembly
- Targets `wasm32-wasip1`
- Dependencies:
  - `zed_extension_api` 0.1.0
  - `serde` 1.0 (serialization)
  - `anyhow` 1.0 (error handling)
  - `tokio` 1.35 (async runtime)
  - `ed25519-dalek` 2.0 (signatures)
  - `sha2` 0.10 (hashing)
  - And more...

### Known Limitations (Phase 1)
- No real-time LSP validation yet
- No actual LLM code generation yet
- No audit trail persistence yet
- No workflow UI yet
- No coverage calculation yet

These features are planned for future phases.

## Release Notes Format

### [Version] - YYYY-MM-DD

#### Added
- New features

#### Changed
- Changes to existing functionality

#### Deprecated
- Soon-to-be removed features

#### Removed
- Removed features

#### Fixed
- Bug fixes

#### Security
- Security improvements

---

## Upcoming Milestones

### v0.2.0 - Phase 2: LSP Server
**Target**: Q1 2026
- Real-time validation
- Inline diagnostics
- Quick fixes

### v0.3.0 - Phase 3: LLM Integration
**Target**: Q2 2026
- Code generation
- Multi-provider support
- Streaming responses

### v0.4.0 - Phase 4: Audit Trail
**Target**: Q2 2026
- Cryptographic audit log
- Compliance exports

### v0.5.0 - Phase 5: Workflow UI
**Target**: Q3 2026
- Visual panels
- Status indicators

### v0.6.0 - Phase 6: Coverage Analysis
**Target**: Q3 2026
- Coverage calculation
- Visualizations

### v1.0.0 - General Availability
**Target**: Q4 2026
- Complete feature set
- Production-ready
- Published to Zed registry

---

For detailed implementation plans, see:
- `Zed_OpenSpec_Extension_Spec.md`
- `Zed_OpenSpec_Technical_Implementation_Guide.md`
- `QUICK_START.md`
