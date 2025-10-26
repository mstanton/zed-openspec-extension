# Specification Document: Zed-OpenSpec Extension

## Document Information
- **Project/Feature Name:** Zed Editor OpenSpec Integration Extension
- **Version:** 1.0
- **Author(s):** Development Team
- **Date Created:** October 25, 2025
- **Last Updated:** October 25, 2025
- **Status:** Draft
- **Reviewers:** [Pending]
- **Stakeholders:** Zed Users, OpenSpec Community, Development Teams

---

## 1. Executive Summary

This specification outlines a comprehensive Zed editor extension that integrates OpenSpec's spec-driven development workflow with advanced LLM-powered code generation, real-time auditing, and workflow tracking capabilities. The extension transforms Zed into a complete spec-driven development environment where specifications and implementation remain synchronized, auditable, and AI-enhanced.

The Zed-OpenSpec extension will provide developers with:
- **Native OpenSpec Integration**: First-class support for OpenSpec workflows directly within Zed
- **LLM-Powered Code Generation**: Intelligent code generation from specifications using multiple LLM providers
- **Real-time Audit Trail**: Complete tracking of all spec-to-code transformations and developer actions
- **Workflow Orchestration**: Visual management of proposal → implementation → archive lifecycle
- **Spec Coverage Analysis**: Real-time visibility into which code implements which specifications
- **Collaborative Spec Editing**: Multi-cursor spec editing with change tracking

**Key Points:**
- Enable spec-driven development directly in Zed without context switching
- Leverage multiple LLM providers (Claude, GPT-4, local models) for code generation
- Maintain complete audit trail of all spec → code transformations
- Provide visual workflow management for OpenSpec changes
- Support both greenfield and brownfield development patterns
- **Expected Timeline:** 12-16 weeks for MVP (Phases 1-2)
- **Critical Success Factors:** Developer adoption, LLM reliability, audit data integrity, performance at scale

---

## 2. Background & Context

### Problem Statement

Developers using spec-driven development with OpenSpec face several workflow challenges:

1. **Context Switching**: Moving between editor, terminal, and spec documents breaks flow
2. **Manual Synchronization**: Keeping specs and code in sync requires constant vigilance
3. **Limited LLM Integration**: OpenSpec works with AI tools but lacks deep code generation capabilities
4. **No Audit Trail**: No visibility into what code was generated from which spec, who approved it, when it changed
5. **Workflow Fragmentation**: Proposal → Implementation → Archive workflow happens across tools
6. **Coverage Blindness**: No way to visualize which parts of codebase are spec-driven vs ad-hoc
7. **Manual Validation**: Developers must manually run `openspec validate` and interpret results
8. **Limited Collaboration**: Spec reviews happen outside the editor, slowing feedback loops

### Current State

- OpenSpec provides CLI tools and integration via custom slash commands in various editors
- Zed supports extensions via WebAssembly but has limited OpenSpec integration
- Developers manually run OpenSpec commands in terminal while editing in Zed
- LLM code generation happens through separate AI coding assistants without audit trails
- No systematic tracking of spec → implementation transformations
- Spec coverage and compliance are invisible during development

### Motivation

- **Zed's Growing Adoption**: Zed is gaining traction as a fast, collaborative editor
- **Spec-Driven Development Trend**: Teams increasingly adopt spec-first approaches for AI collaboration
- **LLM Code Generation**: Need for controlled, auditable AI code generation from specifications
- **Compliance Requirements**: Teams need audit trails for generated code (SOC 2, regulatory)
- **Developer Experience**: Single-tool workflows significantly improve productivity
- **OpenSpec Ecosystem**: Strengthening OpenSpec's presence across development environments

### Related Work

- **OpenSpec CLI**: https://github.com/Fission-AI/OpenSpec
- **Zed Extension API**: https://zed.dev/docs/extensions/developing-extensions
- **Cursor OpenSpec Integration**: Custom slash commands for OpenSpec
- **Claude Code Integration**: `/openspec:proposal`, `/openspec:apply`, `/openspec:archive`
- **Prior Art**: VSCode extensions for spec management, Copilot code generation patterns
- **Lessons Learned**: 
  - Inline validation is more effective than terminal output
  - Visual workflow indicators improve adoption
  - Audit trails are essential for enterprise adoption
  - LLM integration requires fallback strategies

---

## 3. Goals & Non-Goals

### Goals

1. **Seamless OpenSpec Workflow**: Enable complete OpenSpec workflow (proposal → apply → archive) without leaving Zed
2. **Multi-LLM Code Generation**: Generate code from specs using Claude, GPT-4, or local models with provider fallback
3. **Complete Audit Trail**: Track every spec → code transformation with timestamp, LLM used, prompt, output, and developer approval
4. **Visual Workflow Management**: Provide intuitive UI for managing OpenSpec changes, tasks, and spec deltas
5. **Real-time Spec Validation**: Validate specifications as developers type with inline error highlighting
6. **Spec Coverage Visualization**: Show which code files are implemented from specs with visual indicators
7. **Collaborative Spec Editing**: Enable team collaboration on specs using Zed's collaborative features
8. **Performance at Scale**: Handle large codebases (100K+ LOC) and hundreds of specs without degradation
9. **Extensibility**: Provide plugin architecture for custom validators, generators, and audit processors
10. **Developer Adoption**: Achieve 70%+ adoption rate among OpenSpec users within 6 months

### Non-Goals

1. **Replace OpenSpec CLI**: Not building a complete replacement; the extension calls OpenSpec CLI
2. **Custom Spec Format**: Not inventing new spec formats; adhering to OpenSpec's format
3. **Built-in LLM Models**: Not bundling LLM models; using external APIs (with option for local)
4. **Real-time Collaboration Server**: Leveraging Zed's existing collaboration, not building custom infrastructure
5. **Spec Generation from Code**: Phase 1 focuses on spec → code; reverse engineering is future work
6. **Advanced Analytics Dashboard**: Not building comprehensive analytics; basic metrics only
7. **Custom Git Integration**: Using Zed's built-in Git support, not custom VCS features
8. **Multi-Workspace Management**: Phase 1 focuses on single-workspace; federation is future consideration
9. **Mobile Support**: Desktop-only for Phase 1
10. **Offline LLM Inference**: Offline code generation requires separate local model setup (not built-in)

### Success Metrics

| Metric | Baseline | Target | Timeline |
|--------|----------|--------|----------|
| Developer Adoption (OpenSpec Users) | 0% | 70% | 6 months |
| Code Generation Success Rate | N/A | 85%+ | 3 months |
| Spec Validation Response Time (p95) | N/A | < 500ms | Launch |
| Audit Data Completeness | N/A | 100% | Launch |
| Extension Load Time | N/A | < 2 seconds | Launch |
| Large File Performance (10K LOC) | N/A | < 100ms latency | Launch |
| LLM API Failure Recovery Rate | N/A | 95%+ | 3 months |
| Workflow Completion Time | Baseline manual | 50% reduction | 6 months |
| User Satisfaction (NPS) | N/A | > 40 | 6 months |
| Spec Coverage in Projects | Unknown | 60%+ average | 6 months |

---

## 4. User Stories & Use Cases

### Primary User Personas

- **Alex (Senior Engineer)**: Uses OpenSpec for all feature development. Needs fast, reliable code generation with full audit trails for compliance. Pain points: context switching, manual validation, unclear spec coverage.

- **Jordan (Tech Lead)**: Reviews spec proposals and approves code generation. Needs visibility into what's being generated, who approved it, and when. Pain points: scattered review workflows, no audit trail, difficulty tracking changes.

- **Sam (Junior Developer)**: Learning spec-driven development. Needs guidance on writing good specs and understanding how specs map to code. Pain points: spec format confusion, unclear workflow, learning curve.

- **Morgan (DevOps/Compliance)**: Needs audit trails for all AI-generated code for SOC 2 compliance. Pain points: no tracking of LLM usage, unclear provenance of generated code, manual audit collection.

### User Stories

**Story 1: Create and Refine Spec Proposal with Inline Validation**

As a Senior Engineer, I want to create OpenSpec proposals directly in Zed with real-time validation feedback, so that I can iterate quickly without running terminal commands.

- **Acceptance Criteria:**
  - [ ] Can create new OpenSpec change proposal via command palette (`openspec: new proposal`)
  - [ ] Extension scaffolds `openspec/changes/{name}/` with proposal.md, tasks.md, and specs/
  - [ ] Real-time validation highlights errors and warnings as I type
  - [ ] Inline error messages appear on hover with suggested fixes
  - [ ] Validation checks: spec format, requirement structure, scenario completeness
  - [ ] Validation completes within 500ms (p95) for typical specs
  - [ ] Can toggle validation on/off via settings
  - [ ] Validation errors appear in Problems panel with clickable navigation

**Story 2: Generate Code from Specs with LLM Provider Choice**

As any developer, I want to generate code from my spec tasks using my preferred LLM (Claude, GPT-4, or local model) with a single command, so that implementation follows the agreed specification.

- **Acceptance Criteria:**
  - [ ] Can trigger code generation via command palette (`openspec: apply change`)
  - [ ] Extension prompts for LLM provider selection (Claude, OpenAI, Local, Default from settings)
  - [ ] Shows estimated token usage before generation
  - [ ] Generation runs asynchronously with progress indicator
  - [ ] Generated code appears in editor with diff view showing changes
  - [ ] Can review generated code before accepting
  - [ ] Each generation creates audit trail entry
  - [ ] Can regenerate specific tasks if output is unsatisfactory
  - [ ] Supports streaming generation with partial results visible
  - [ ] Handles generation failures gracefully with retry options

**Story 3: Track Complete Audit Trail of Generated Code**

As a DevOps engineer, I need a complete audit trail of all AI-generated code including LLM used, prompts, timestamps, and developer approvals, so that we meet compliance requirements.

- **Acceptance Criteria:**
  - [ ] Every code generation creates immutable audit entry
  - [ ] Audit entry includes: timestamp, developer, LLM provider, model version, prompt, generated code, acceptance status
  - [ ] Audit entries stored in `.openspec/audit/` directory
  - [ ] Can view audit trail via command palette (`openspec: view audit`)
  - [ ] Audit viewer shows filterable, sortable list of all generations
  - [ ] Can export audit data to JSON/CSV for external analysis
  - [ ] Audit entries cryptographically signed to prevent tampering
  - [ ] Audit data includes code hashes for verification
  - [ ] Can search audit by file, developer, LLM, date range
  - [ ] Audit retention policy configurable (default: 2 years)

**Story 4: Visualize Spec Coverage in Codebase**

As a Tech Lead, I want to see which parts of the codebase are implemented from specs vs ad-hoc code, so that I can understand spec adoption and identify gaps.

- **Acceptance Criteria:**
  - [ ] File explorer shows spec coverage indicators (✓ spec-driven, ⚠ partial, ✗ no spec)
  - [ ] Can view spec→code mapping for any file via context menu
  - [ ] Coverage panel shows project-wide statistics (% spec-covered)
  - [ ] Can drill down to see which specs cover which files
  - [ ] Coverage calculated from audit trail and OpenSpec metadata
  - [ ] Coverage updates automatically when code or specs change
  - [ ] Can filter files by coverage status
  - [ ] Coverage data exportable for reporting

**Story 5: Manage OpenSpec Change Workflow Visually**

As any developer, I want a visual panel showing all active OpenSpec changes with their status (proposal → implementing → archiving), so that I can track what's in flight.

- **Acceptance Criteria:**
  - [ ] Workflow panel accessible via sidebar
  - [ ] Panel lists all changes in `openspec/changes/` directory
  - [ ] Each change shows: name, status, task progress, last updated
  - [ ] Can click to open proposal.md, tasks.md, or spec deltas
  - [ ] Status badges: 🔵 Proposed, 🟡 In Progress, 🟢 Ready to Archive
  - [ ] Task checklist shows completion (3/7 tasks complete)
  - [ ] Can trigger `apply` or `archive` actions from panel
  - [ ] Panel updates in real-time as files change
  - [ ] Can search/filter changes

**Story 6: Collaborate on Specs in Real-time**

As a team member, I want to collaboratively edit specs with my teammates using Zed's collaborative features, so that we can align on requirements together.

- **Acceptance Criteria:**
  - [ ] OpenSpec markdown files support Zed's multi-cursor collaboration
  - [ ] Changes by other users appear in real-time
  - [ ] Each user's cursor/selection shows their avatar
  - [ ] Validation runs for all collaborators simultaneously
  - [ ] Conflict resolution follows Zed's standard approach
  - [ ] Change history visible in spec metadata
  - [ ] Can comment inline on specs (using Zed's comment system)
  - [ ] All collaborators see the same validation errors

---

## 5. Functional Requirements

### Core Features

**Feature 1: OpenSpec Command Integration**
- **Description**: Integrate all OpenSpec CLI commands as Zed command palette actions
- **Priority**: P0
- **Requirements**:
  - REQ-1.1: Command palette includes `openspec: init`, `openspec: new proposal`, `openspec: list`, `openspec: show`, `openspec: validate`, `openspec: apply`, `openspec: archive`
  - REQ-1.2: Commands call OpenSpec CLI with appropriate arguments
  - REQ-1.3: Command output displayed in Zed's terminal or output panel
  - REQ-1.4: Commands detect current workspace and OpenSpec directory structure
  - REQ-1.5: Error handling for missing OpenSpec installation with installation instructions
  - REQ-1.6: Command execution shown in status bar with progress indicator
  - REQ-1.7: Keybindings configurable for frequent commands (e.g., Ctrl+Shift+O for new proposal)

**Feature 2: Real-time Spec Validation**
- **Description**: Validate OpenSpec spec files as developers type with inline diagnostics
- **Priority**: P0
- **Requirements**:
  - REQ-2.1: Language server protocol (LSP) integration for OpenSpec markdown files
  - REQ-2.2: Validation checks: requirement format (SHALL/MUST), scenario structure, delta syntax
  - REQ-2.3: Inline error/warning highlighting with squiggly underlines
  - REQ-2.4: Hover tooltips show error details and suggested fixes
  - REQ-2.5: Problems panel lists all validation issues with clickable navigation
  - REQ-2.6: Validation debounced (500ms delay) to avoid performance issues
  - REQ-2.7: Validation rules configurable via `.openspec/validation-rules.json`
  - REQ-2.8: Quick fixes available via lightbulb icon (e.g., "Add missing Scenario block")
  - REQ-2.9: Validation respects OpenSpec spec format (requirements, scenarios, deltas)
  - REQ-2.10: Can disable validation for specific files or directories

**Feature 3: LLM-Powered Code Generation**
- **Description**: Generate code from spec tasks using configurable LLM providers
- **Priority**: P0
- **Requirements**:
  - REQ-3.1: Support multiple LLM providers: Anthropic Claude, OpenAI GPT-4, Azure OpenAI, Local models (via Ollama)
  - REQ-3.2: Provider selection via settings or per-generation prompt
  - REQ-3.3: API key management via secure storage (Zed's secret storage)
  - REQ-3.4: Generation uses OpenSpec spec deltas and tasks.md as context
  - REQ-3.5: Prompt engineering optimized for each LLM provider
  - REQ-3.6: Streaming generation with partial results visible in editor
  - REQ-3.7: Generated code appears in diff view for review before acceptance
  - REQ-3.8: Token usage estimation before generation with cost calculation
  - REQ-3.9: Fallback to secondary provider on primary failure
  - REQ-3.10: Rate limiting and retry logic with exponential backoff
  - REQ-3.11: Can cancel in-progress generation
  - REQ-3.12: Context window management (truncate large codebases intelligently)
  - REQ-3.13: Support for custom generation templates/prompts

**Feature 4: Comprehensive Audit Trail**
- **Description**: Log all spec → code transformations with immutable audit records
- **Priority**: P0
- **Requirements**:
  - REQ-4.1: Audit entries stored in `.openspec/audit/{timestamp}-{uuid}.json`
  - REQ-4.2: Audit entry schema includes: `timestamp`, `developer`, `llm_provider`, `model`, `change_id`, `task_id`, `prompt`, `generated_code`, `code_hash`, `accepted`, `files_modified`, `signature`
  - REQ-4.3: Cryptographic signature using developer's Git identity
  - REQ-4.4: Audit entries immutable (write-once, read-many)
  - REQ-4.5: Audit viewer UI shows filterable table of all entries
  - REQ-4.6: Can drill down into any audit entry to see full details
  - REQ-4.7: Export audit data to JSON, CSV, or human-readable report
  - REQ-4.8: Audit integrity verification command (`openspec: verify audit`)
  - REQ-4.9: Audit retention policy enforced (default 2 years, configurable)
  - REQ-4.10: Audit search by developer, LLM, file, date range, change ID
  - REQ-4.11: Audit data size management (compression, archival)

**Feature 5: Spec Coverage Analysis**
- **Description**: Calculate and visualize which code is spec-driven vs ad-hoc
- **Priority**: P1
- **Requirements**:
  - REQ-5.1: Coverage calculated from audit trail (files generated from specs)
  - REQ-5.2: File tree shows coverage badges (✓ full, ⚠ partial, ✗ none)
  - REQ-5.3: Coverage percentage calculated: `(spec_covered_lines / total_lines) * 100`
  - REQ-5.4: Coverage panel shows project-wide statistics and top-level metrics
  - REQ-5.5: Can view spec→file mapping via context menu
  - REQ-5.6: Coverage updates automatically on file changes
  - REQ-5.7: Hover on file shows which specs contributed to its implementation
  - REQ-5.8: Coverage data exportable to JSON for external tools
  - REQ-5.9: Coverage excludes test files by default (configurable)
  - REQ-5.10: Heat map visualization of coverage across directory structure

**Feature 6: Visual Workflow Management**
- **Description**: Provide sidebar panel for managing OpenSpec change workflow
- **Priority**: P1
- **Requirements**:
  - REQ-6.1: Sidebar panel lists all changes from `openspec/changes/`
  - REQ-6.2: Change cards show: name, status, task progress, last updated, assigned developer
  - REQ-6.3: Status inferred from files: 🔵 Proposal (no code yet), 🟡 In Progress (partial tasks), 🟢 Complete (all tasks done)
  - REQ-6.4: Task checklist extracted from tasks.md with completion tracking
  - REQ-6.5: Click to open proposal.md, tasks.md, or spec deltas
  - REQ-6.6: Context menu actions: Apply Change, Archive Change, Delete Change
  - REQ-6.7: Search and filter changes by name, status, assignee
  - REQ-6.8: Drag-and-drop to reorder changes (priority)
  - REQ-6.9: Panel updates via file system watcher (real-time)
  - REQ-6.10: Collapse/expand change cards for clean UI

**Feature 7: Collaborative Spec Editing**
- **Description**: Enable real-time collaborative editing of OpenSpec files
- **Priority**: P2
- **Requirements**:
  - REQ-7.1: OpenSpec markdown files support Zed's collaboration protocol
  - REQ-7.2: Multiple cursors visible with user avatars
  - REQ-7.3: Changes propagate in real-time to all collaborators
  - REQ-7.4: Validation runs for all users with consistent results
  - REQ-7.5: Presence indicators show who's viewing which files
  - REQ-7.6: Conflict resolution follows Zed's CRDT-based approach
  - REQ-7.7: Chat integration for discussing specs
  - REQ-7.8: Can leave inline comments on specs (annotations)

**Feature 8: Spec Snippet Library**
- **Description**: Provide reusable spec templates and snippets
- **Priority**: P2
- **Requirements**:
  - REQ-8.1: Built-in snippets for common spec patterns (REQ, SCENARIO, DELTA)
  - REQ-8.2: Custom snippet creation via settings
  - REQ-8.3: Snippets accessible via autocomplete in spec files
  - REQ-8.4: Snippet variables (e.g., `${requirement_name}`) filled via placeholders
  - REQ-8.5: Community snippet sharing (optional import from repository)

### Business Rules

- BR-1: All code generation requires explicit developer approval before writing to files
- BR-2: Audit entries cannot be deleted or modified once created
- BR-3: LLM API keys stored securely via Zed's credential management (never in plaintext)
- BR-4: Spec validation never blocks file saving (warnings only, not errors)
- BR-5: Coverage calculations exclude files matching `.gitignore` patterns
- BR-6: Change proposals must have unique names within a project
- BR-7: Archive command requires all tasks marked complete (can override with `--force`)
- BR-8: Extension defaults to OpenSpec CLI detection; fails gracefully if not installed
- BR-9: LLM generation limited to 10,000 tokens output per request (configurable)
- BR-10: Audit log writes must succeed before code generation commits to files

### Data Validation Rules

- DV-1: Change proposal name: alphanumeric, hyphens, underscores only; max 64 chars
- DV-2: Spec requirement headers must start with `### Requirement:` (case-insensitive)
- DV-3: Scenario headers must start with `#### Scenario:` (case-insensitive)
- DV-4: Delta sections must use `## ADDED`, `## MODIFIED`, or `## REMOVED`
- DV-5: Task IDs in tasks.md must be unique and follow format `X.Y` (e.g., `1.1`, `2.3`)
- DV-6: LLM provider must be one of: `claude`, `gpt-4`, `azure-openai`, `ollama`, `custom`
- DV-7: Audit entries must include all required fields (timestamp, developer, llm, change_id)
- DV-8: Spec file sizes limited to 1MB (warning at 500KB)
- DV-9: Generated code must be valid syntax for target language (pre-commit validation)

---

## 6. Technical Architecture

### System Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                        Zed Editor                             │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │            Extension Host (WebAssembly)                  │ │
│  │  ┌────────────────────────────────────────────────────┐  │ │
│  │  │      OpenSpec Extension (Rust → WASM)             │  │ │
│  │  │  ┌──────────────┐  ┌───────────────┐              │  │ │
│  │  │  │ Command      │  │ LSP Server    │              │  │ │
│  │  │  │ Handlers     │  │ (Validation)  │              │  │ │
│  │  │  └──────┬───────┘  └───────┬───────┘              │  │ │
│  │  │         │                   │                      │  │ │
│  │  │  ┌──────┴────────────┬──────┴───────┐             │  │ │
│  │  │  │ Workflow Manager  │ LLM Gateway  │             │  │ │
│  │  │  └──────┬────────────┴──────┬───────┘             │  │ │
│  │  │         │                    │                     │  │ │
│  │  │  ┌──────┴──────────┐  ┌──────┴────────┐           │  │ │
│  │  │  │ Audit Engine    │  │ Coverage      │           │  │ │
│  │  │  │ (Immutable Log) │  │ Analyzer      │           │  │ │
│  │  │  └─────────────────┘  └───────────────┘           │  │ │
│  │  └────────────────────────────────────────────────────┘  │ │
│  │                                                          │ │
│  │  ┌────────────────────────────────────────────────────┐  │ │
│  │  │            Zed Extension API                       │  │ │
│  │  │  - Commands, Keybindings, Menus                    │  │ │
│  │  │  - File System Watcher                             │  │ │
│  │  │  - Editor Integration (decorations, diagnostics)   │  │ │
│  │  │  - Secret Storage (API keys)                       │  │ │
│  │  └────────────────────────────────────────────────────┘  │ │
│  └──────────────────────────────────────────────────────────┘ │
└──────────────┬───────────────────────────┬────────────────────┘
               │                           │
               ▼                           ▼
┌──────────────────────────┐  ┌────────────────────────────┐
│   OpenSpec CLI           │  │   LLM Providers            │
│   (local installation)   │  │   - Anthropic Claude API   │
│   - openspec validate    │  │   - OpenAI GPT-4 API       │
│   - openspec list        │  │   - Azure OpenAI           │
│   - openspec archive     │  │   - Ollama (local)         │
└──────────────────────────┘  └────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────┐
│          Project File System                │
│  ┌────────────────────────────────────────┐  │
│  │  openspec/                             │  │
│  │  ├── specs/           (source truth)   │  │
│  │  ├── changes/         (active work)    │  │
│  │  ├── archive/         (completed)      │  │
│  │  ├── .audit/          (audit trail)    │  │
│  │  └── AGENTS.md        (AI instructions)│  │
│  └────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────┐  │
│  │  src/                (implementation)  │  │
│  │  tests/              (test code)       │  │
│  │  .openspec-config.json (settings)      │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

### Technology Stack

- **Extension Core**: 
  - Language: Rust (compiles to WebAssembly)
  - Build: Cargo, wasm-pack
  - Zed Extension API: `zed_extension_api` crate
  - Async Runtime: tokio (WASM-compatible)

- **LSP Server** (embedded in extension):
  - Framework: tower-lsp
  - Parser: tree-sitter-markdown, custom OpenSpec grammar
  - Validation Engine: Custom rule engine in Rust

- **LLM Integration**:
  - HTTP Client: reqwest (WASM-compatible)
  - Streaming: Server-Sent Events (SSE) or streaming JSON
  - Providers: Anthropic SDK, OpenAI SDK, Ollama REST API
  - Fallback Chain: Primary → Secondary → Error

- **Audit System**:
  - Storage: JSON files in `.openspec/audit/`
  - Signing: Ed25519 cryptographic signatures
  - Integrity: SHA-256 hashes of code
  - Indexing: In-memory SQLite (WASM) for fast queries

- **Coverage Analysis**:
  - Parser: tree-sitter for multiple languages
  - Algorithm: Line-level mapping from audit → code
  - Caching: In-memory cache with file watcher invalidation

- **UI Components**:
  - Sidebar Panel: Zed's panel API
  - Decorations: Zed's gutter and inline decoration API
  - Diagnostics: Zed's diagnostic API (LSP)
  - Diff View: Zed's built-in diff viewer

- **Data Persistence**:
  - Config: `.openspec-config.json` (per-project)
  - Audit: `.openspec/audit/*.json` (append-only)
  - Cache: In-memory (no persistent cache for Phase 1)

### Component Design

**Component 1: Command Handler**
- **Responsibility**: Dispatch Zed commands to appropriate handlers
- **Interfaces**:
  - `handle_command(cmd: Command) -> Result<(), Error>`
  - `register_commands() -> Vec<CommandDefinition>`
- **Dependencies**: OpenSpec CLI executor, Workflow Manager
- **Key Features**:
  - Command validation before execution
  - Progress reporting to status bar
  - Error handling with user-friendly messages
  - Keybinding registration

**Component 2: LSP Server (Validation)**
- **Responsibility**: Real-time spec validation via Language Server Protocol
- **Interfaces**:
  - `on_did_change(params: DidChangeTextDocumentParams)`
  - `on_hover(params: HoverParams) -> Hover`
  - `on_code_action(params: CodeActionParams) -> Vec<CodeAction>`
- **Dependencies**: OpenSpec spec parser, Validation rule engine
- **Key Features**:
  - Incremental parsing (only re-parse changed sections)
  - Debounced validation (500ms)
  - Quick fixes for common errors
  - Custom OpenSpec-aware syntax highlighting

**Component 3: LLM Gateway**
- **Responsibility**: Manage LLM API calls for code generation
- **Interfaces**:
  - `generate_code(spec: SpecDelta, tasks: Vec<Task>, provider: LLMProvider) -> Stream<CodeChunk>`
  - `select_provider() -> LLMProvider`
  - `estimate_tokens(spec: SpecDelta) -> TokenEstimate`
- **Dependencies**: HTTP client, LLM provider SDKs, Token counter
- **Key Features**:
  - Provider abstraction layer (uniform interface for all LLMs)
  - Streaming support with backpressure handling
  - Retry logic with exponential backoff
  - Fallback provider chain
  - Token usage tracking and limits
  - Prompt template system

**Component 4: Workflow Manager**
- **Responsibility**: Manage OpenSpec change lifecycle and state
- **Interfaces**:
  - `list_changes() -> Vec<Change>`
  - `get_change(id: ChangeId) -> Change`
  - `apply_change(id: ChangeId, llm: LLMProvider) -> Result<ApplyResult>`
  - `archive_change(id: ChangeId) -> Result<()>`
- **Dependencies**: File system watcher, Task parser, OpenSpec CLI
- **Key Features**:
  - Real-time change detection via FS watcher
  - Task completion tracking from tasks.md
  - Status inference (Proposed → In Progress → Complete)
  - Change validation before apply/archive

**Component 5: Audit Engine**
- **Responsibility**: Create and manage immutable audit trail
- **Interfaces**:
  - `record_generation(entry: AuditEntry) -> Result<AuditId>`
  - `query_audit(filter: AuditFilter) -> Vec<AuditEntry>`
  - `verify_integrity() -> IntegrityReport`
  - `export_audit(format: ExportFormat) -> Vec<u8>`
- **Dependencies**: Cryptographic signing, File system
- **Key Features**:
  - Append-only audit log (immutable)
  - Cryptographic signatures (Ed25519)
  - Fast queries via in-memory index
  - Compression for old entries
  - Tamper detection

**Component 6: Coverage Analyzer**
- **Responsibility**: Calculate spec coverage of codebase
- **Interfaces**:
  - `calculate_coverage() -> CoverageReport`
  - `get_file_coverage(path: FilePath) -> FileCoverage`
  - `get_spec_to_files(spec: SpecId) -> Vec<FilePath>`
- **Dependencies**: Audit Engine, tree-sitter parsers
- **Key Features**:
  - Line-level coverage calculation
  - Multi-language support via tree-sitter
  - Incremental updates on file changes
  - Efficient caching
  - Visualization data generation

### Integration Points

- **OpenSpec CLI**: Shell execution of OpenSpec commands via Zed's process API
- **LLM Providers**: HTTPS API calls to Anthropic, OpenAI, Azure, or local Ollama
- **Zed Editor**: WebAssembly extension API for commands, LSP, UI, file system
- **File System**: Direct read/write to project files and `.openspec/` directory
- **Git**: Read Git identity for audit signatures (via Zed's Git integration)

---

## 7. Data Model & Schema

### Entity Relationship Diagram

```
┌─────────────────────┐
│   Change            │
│   (from FS)         │
├─────────────────────┤
│ id (string)         │
│ name                │
│ status              │──────┐
│ proposal_path       │      │
│ tasks_path          │      │
│ spec_deltas[]       │      │
│ created_at          │      │
│ updated_at          │      │
└─────────────────────┘      │
         │ 1:N               │
         │                   │
         ▼                   │
┌─────────────────────┐      │
│   Task              │      │
├─────────────────────┤      │
│ id (string)         │      │
│ change_id (fk)      │      │
│ number (e.g., 1.1)  │      │
│ description         │      │
│ completed (bool)    │      │
│ assigned_to         │      │
└─────────────────────┘      │
                             │
                             │ N:M via audit
                             │
                             ▼
┌────────────────────────────────┐
│   AuditEntry                   │
├────────────────────────────────┤
│ id (uuid)                      │
│ timestamp                      │
│ change_id (fk)                 │
│ task_ids[] (fk)                │
│ developer (email)              │
│ llm_provider                   │
│ llm_model                      │
│ prompt_hash                    │
│ generated_code_hash            │
│ files_modified[]               │
│ accepted (bool)                │
│ signature (Ed25519)            │
└────────────────────────────────┘
         │ 1:N
         │
         ▼
┌─────────────────────┐
│   FileCoverage      │
├─────────────────────┤
│ path                │
│ total_lines         │
│ spec_covered_lines  │
│ coverage_percent    │
│ contributing_specs[]│──┐
│ last_updated        │  │
└─────────────────────┘  │
                         │ N:M
                         │
                         ▼
┌─────────────────────┐
│   SpecDelta         │
│   (from FS)         │
├─────────────────────┤
│ change_id (fk)      │
│ spec_path           │
│ added_reqs[]        │
│ modified_reqs[]     │
│ removed_reqs[]      │
└─────────────────────┘
```

### Data Entities

**Entity 1: Change**

```rust
struct Change {
    id: String,              // Change directory name (e.g., "add-2fa")
    name: String,            // Human-readable name
    status: ChangeStatus,    // Proposed | InProgress | Complete
    proposal_path: PathBuf,  // Path to proposal.md
    tasks_path: PathBuf,     // Path to tasks.md
    spec_deltas: Vec<SpecDelta>, // List of spec changes
    tasks: Vec<Task>,        // List of tasks
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    assigned_to: Option<String>, // Developer email
}

enum ChangeStatus {
    Proposed,    // Proposal created, no code yet
    InProgress,  // Some tasks completed
    Complete,    // All tasks done, ready to archive
}
```

**Entity 2: Task**

```rust
struct Task {
    id: String,           // Task number (e.g., "1.1", "2.3")
    change_id: String,    // Parent change
    description: String,  // Task description
    completed: bool,      // Checkbox state from tasks.md
    assigned_to: Option<String>,
    estimated_tokens: Option<u32>, // For LLM generation
}
```

**Entity 3: AuditEntry**

```json
{
  "id": "uuid - unique audit entry ID",
  "timestamp": "RFC 3339 timestamp",
  "change_id": "string - OpenSpec change ID",
  "task_ids": ["array of task IDs (e.g., ['1.1', '2.3'])"],
  "developer": "string - Git user.email",
  "git_commit": "string - Git commit hash (if available)",
  "llm_provider": "enum - claude | gpt-4 | azure-openai | ollama | custom",
  "llm_model": "string - Model version (e.g., claude-sonnet-4-20250514)",
  "prompt": {
    "spec_delta": "string - Spec delta used for generation",
    "task_description": "string - Task from tasks.md",
    "context_files": ["array of file paths included in context"],
    "prompt_template": "string - Template used",
    "estimated_tokens": "number - Token count"
  },
  "generation": {
    "generated_code": "string - Full generated code",
    "code_hash": "string - SHA-256 hash of generated code",
    "files_modified": [
      {
        "path": "string",
        "action": "enum - created | modified | deleted",
        "lines_added": "number",
        "lines_removed": "number"
      }
    ],
    "generation_time_ms": "number"
  },
  "acceptance": {
    "accepted": "boolean - Did developer accept?",
    "accepted_at": "timestamp (if accepted)",
    "modifications": "string - User edits after generation (if any)"
  },
  "signature": "string - Ed25519 signature of entry data",
  "metadata": {
    "extension_version": "string",
    "zed_version": "string",
    "openspec_version": "string"
  }
}
```

**Constraints:**
- `id`: UUID v4
- `timestamp`: Immutable, server-generated
- `signature`: Cryptographically verifies entry integrity
- `change_id`: Must reference existing change
- All fields required except `acceptance.modifications`

**Entity 4: SpecDelta**

```rust
struct SpecDelta {
    change_id: String,
    spec_path: PathBuf,      // Relative path to spec file
    spec_type: SpecType,     // Source | Delta
    added_requirements: Vec<Requirement>,
    modified_requirements: Vec<RequirementModification>,
    removed_requirements: Vec<String>, // Requirement IDs
}

struct Requirement {
    id: String,           // Auto-generated or explicit
    title: String,        // "User Authentication"
    description: String,  // SHALL/MUST statement
    scenarios: Vec<Scenario>,
}

struct RequirementModification {
    requirement_id: String,
    old_description: String,
    new_description: String,
}

enum SpecType {
    Source,  // From openspec/specs/
    Delta,   // From openspec/changes/{id}/specs/
}
```

**Entity 5: FileCoverage**

```rust
struct FileCoverage {
    path: PathBuf,
    total_lines: usize,
    spec_covered_lines: usize,
    coverage_percent: f32,       // (spec_covered_lines / total_lines) * 100
    contributing_specs: Vec<String>, // Spec IDs that contributed
    contributing_changes: Vec<String>, // Change IDs
    last_updated: DateTime<Utc>,
}
```

### Data Flow

1. **Change Creation**: User runs `openspec: new proposal` → Extension scaffolds `openspec/changes/{name}/` → Change entity created in memory
2. **Spec Editing**: User edits spec files → LSP validates on change → Diagnostics appear in editor → SpecDelta parsed
3. **Code Generation**: User runs `openspec: apply` → LLM Gateway sends spec + tasks to LLM → Streaming response → Generated code shown in diff view → User accepts → AuditEntry created → Files written
4. **Audit Recording**: Code accepted → AuditEntry generated → Signed with developer's Git identity → Written to `.openspec/audit/{timestamp}-{uuid}.json` → Immutable
5. **Coverage Calculation**: File modified → Coverage Analyzer checks audit trail → Determines if file was spec-generated → Updates FileCoverage → UI refreshes
6. **Change Archival**: All tasks complete → User runs `openspec: archive` → OpenSpec CLI merges spec deltas → Change moved to `openspec/archive/` → Coverage updated

---

## 8. API Specifications

### Extension Commands (Command Palette)

**Command 1: `openspec: init`**
- **Description**: Initialize OpenSpec in current workspace
- **Keybinding**: None (run once per project)
- **Implementation**: Executes `openspec init` CLI command
- **Input**: None (uses current workspace directory)
- **Output**: Terminal shows initialization progress
- **Error Handling**: If OpenSpec not installed, show installation instructions

---

**Command 2: `openspec: new proposal`**
- **Description**: Create a new OpenSpec change proposal
- **Keybinding**: `Ctrl+Shift+N` (configurable)
- **Implementation**: 
  1. Prompt user for proposal name
  2. Execute `openspec proposal {name}`
  3. Open proposal.md in editor
- **Input**: Proposal name (alphanumeric, hyphens, underscores)
- **Output**: New change directory created, proposal.md opened
- **Validation**: Name must be unique, < 64 chars

---

**Command 3: `openspec: apply change`**
- **Description**: Generate code for a change using LLM
- **Keybinding**: `Ctrl+Shift+A` (configurable)
- **Implementation**:
  1. Show change picker (list from workflow panel)
  2. Prompt for LLM provider selection
  3. Show token estimate and cost
  4. Stream code generation with progress
  5. Show diff view for review
  6. On acceptance: write files + create audit entry
- **Input**: Change ID, LLM provider
- **Output**: Generated code in diff view
- **Error Handling**: LLM API failures → retry with fallback provider

---

**Command 4: `openspec: archive change`**
- **Description**: Archive completed change
- **Keybinding**: `Ctrl+Shift+R` (configurable)
- **Implementation**:
  1. Verify all tasks complete
  2. Execute `openspec archive {change} --yes`
  3. Update workflow panel
- **Input**: Change ID
- **Output**: Terminal shows archive results
- **Validation**: All tasks must be marked complete

---

**Command 5: `openspec: view audit`**
- **Description**: Open audit trail viewer
- **Keybinding**: `Ctrl+Shift+L` (configurable)
- **Implementation**: Open audit viewer panel (sidebar)
- **Input**: None
- **Output**: Audit panel with filterable entries
- **Features**: Search, filter, export

---

**Command 6: `openspec: validate current file`**
- **Description**: Manually trigger validation for current file
- **Keybinding**: `Ctrl+Shift+V` (configurable)
- **Implementation**: Run LSP validation on active spec file
- **Input**: Current file (must be .md in openspec/)
- **Output**: Diagnostics in Problems panel
- **Note**: Validation normally runs automatically on save

---

**Command 7: `openspec: show coverage`**
- **Description**: Open coverage panel
- **Keybinding**: `Ctrl+Shift+C` (configurable)
- **Implementation**: Open coverage panel showing project statistics
- **Input**: None
- **Output**: Coverage panel with file tree and metrics

---

### LSP Methods

**Method: `textDocument/didChange`**
- **Trigger**: User types in spec file
- **Action**: Debounced validation (500ms) → Parse spec → Run validation rules → Return diagnostics
- **Response**: Array of `Diagnostic` with severity, message, range

**Method: `textDocument/hover`**
- **Trigger**: User hovers over spec element
- **Action**: Show additional information (e.g., requirement details, task status)
- **Response**: `Hover` with markdown content

**Method: `textDocument/codeAction`**
- **Trigger**: User clicks lightbulb on diagnostic
- **Action**: Provide quick fixes (e.g., "Add missing Scenario block")
- **Response**: Array of `CodeAction` with edits

---

### Configuration Schema

**File: `.openspec-config.json`**

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
      },
      "ollama": {
        "model": "codellama",
        "endpoint": "http://localhost:11434",
        "max_tokens": 4000
      }
    },
    "fallback_chain": ["claude", "gpt-4"],
    "generation_timeout_seconds": 120
  },
  "validation": {
    "enabled": true,
    "debounce_ms": 500,
    "rules": {
      "require_scenarios": true,
      "require_shall_must": true,
      "max_spec_size_kb": 1024
    },
    "custom_rules_path": ".openspec/validation-rules.json"
  },
  "audit": {
    "enabled": true,
    "retention_days": 730,
    "signature_required": true,
    "export_format": "json"
  },
  "coverage": {
    "exclude_patterns": ["**/test/**", "**/*.test.*", "**/node_modules/**"],
    "minimum_coverage_percent": 60
  },
  "workflow": {
    "auto_archive_on_complete": false,
    "require_all_tasks_complete": true
  }
}
```

---

## 9. User Interface Specifications

### Sidebar Panel: Workflow Manager

**Location**: Left sidebar (below file explorer)

**Sections**:

1. **Active Changes** (collapsible)
   - List of changes from `openspec/changes/`
   - Each change shows:
     - Name (bold)
     - Status badge (🔵 Proposed, 🟡 In Progress, 🟢 Complete)
     - Task progress (e.g., "3/7 tasks")
     - Last updated timestamp
   - Click to expand: shows task checklist
   - Right-click context menu: Apply, Archive, Open Files

2. **Archived Changes** (collapsible)
   - List of archived changes
   - Shows: name, archived date
   - Click to view historical details

3. **Actions Toolbar** (bottom)
   - Button: "New Proposal"
   - Button: "Refresh"
   - Search box: Filter changes by name

**Interactions**:
- Click change name → Opens proposal.md
- Click task → Opens relevant spec file
- Drag to reorder changes (priority)

---

### Panel: Audit Viewer

**Location**: Bottom panel (alongside terminal)

**Layout**: Table view with columns:
- Timestamp (sortable)
- Developer (filterable)
- Change ID (clickable → opens change)
- LLM Provider (filterable)
- Model
- Files Modified (count)
- Status (✓ Accepted, ✗ Rejected)
- Actions (View Details, Export)

**Filters** (top toolbar):
- Date range picker
- Developer dropdown
- LLM provider dropdown
- Change ID search box
- Status toggle (Accepted/Rejected/All)

**Details View** (click "View Details"):
- Full audit entry in JSON viewer
- Prompt used
- Generated code (syntax highlighted)
- Files modified (with line counts)
- Signature verification status

---

### Panel: Coverage Analysis

**Location**: Right sidebar (toggleable)

**Sections**:

1. **Project Summary** (top card)
   - Overall coverage percentage (big number)
   - Total files analyzed
   - Spec-covered files count
   - Ad-hoc files count
   - Color-coded gauge (green > 70%, yellow 40-70%, red < 40%)

2. **File Tree** (main section)
   - Hierarchical directory view
   - Each file/folder shows coverage badge:
     - ✓ (green) = 100% spec-covered
     - ⚠ (yellow) = 50-99% spec-covered
     - ✗ (red) = 0-49% spec-covered
   - Hover shows: coverage %, contributing specs
   - Click file → jumps to file in editor

3. **Coverage Details** (bottom)
   - Selected file's coverage breakdown
   - Line-level: which lines are spec-generated
   - List of contributing specs (clickable)

---

### Editor Decorations

**Gutter Icons**:
- 🟢 Green dot = Line generated from spec (hover shows audit ID)
- 🔵 Blue dot = Line manually written (no spec)

**Inline Diagnostics** (spec files):
- Red squiggly underline = Validation error
- Yellow squiggly = Validation warning
- Blue squiggly = Info/suggestion

**Status Bar Items**:
- Left: Current OpenSpec change (if applicable)
- Right: Spec coverage for active file (e.g., "Coverage: 85%")

---

### Diff View (Code Review)

**Trigger**: After LLM code generation

**Layout**: Side-by-side diff
- Left: Current code (or empty if new file)
- Right: Generated code

**Actions** (toolbar):
- ✓ Accept (write to file + create audit entry)
- ✗ Reject (discard generation)
- ✏️ Edit (modify generated code before accepting)
- 🔄 Regenerate (try again with same/different LLM)

---

## 10. Security Considerations

### Authentication & Authorization

- **LLM API Keys**: 
  - Stored in Zed's secure credential storage (encrypted at rest)
  - Never logged or included in audit trail
  - Accessed only during LLM API calls
  - User prompted to configure on first use
  - Supports environment variables (e.g., `ANTHROPIC_API_KEY`)

- **Audit Signatures**:
  - Uses developer's Git identity (`user.name`, `user.email`)
  - Ed25519 private key generated per-developer (stored in `~/.openspec/signing-key`)
  - Public keys published to `.openspec/trusted-keys/` for verification
  - Signatures prevent audit entry tampering

- **Authorization**: No explicit auth; relies on file system permissions
  - Only users with write access to `.openspec/audit/` can create audit entries
  - Audit verification checks signatures against trusted keys

### Data Security

- **Encryption at Rest**: 
  - LLM API keys: Encrypted via Zed's credential storage
  - Audit entries: Plain JSON (integrity via signatures, not encryption)
  - Local LLM models: User-managed, no extension responsibility

- **Encryption in Transit**:
  - All LLM API calls: TLS 1.3 minimum
  - HTTPS required for all external requests
  - No unencrypted data transmission

- **PII Handling**:
  - Developer emails in audit trail (from Git config)
  - No collection of additional PII
  - Audit data remains local (no external transmission without explicit export)

- **Code Security**:
  - Generated code syntax validation before writing to files
  - Sandboxed execution environment (WebAssembly)
  - No arbitrary code execution

### Security Testing

- [ ] Input validation: All user inputs sanitized (change names, file paths)
- [ ] Path traversal prevention: Validate all file paths within project directory
- [ ] API key exposure: Verify keys never logged or exposed
- [ ] Audit integrity: Test signature verification against tampered entries
- [ ] LLM prompt injection: Sanitize specs to prevent malicious prompts
- [ ] Dependency scanning: Regular scans of Rust dependencies for vulnerabilities
- [ ] WASM sandboxing: Verify extension cannot escape Zed's sandbox

### Compliance

- **SOC 2 Readiness**:
  - Complete audit trail (access, change, generation logs)
  - Immutable audit records
  - Cryptographic integrity verification
  - Retention policy enforcement

- **GDPR**:
  - Developer emails treated as PII
  - No data transmission outside user's environment (unless explicitly exported)
  - User can delete audit entries locally (though signature verification will fail)

---

## 11. Performance Requirements

### Response Time

- Extension load time: < 2 seconds
- Spec validation (p95): < 500ms for typical spec (< 1000 lines)
- LLM code generation (wall-clock):
  - Small tasks (< 100 LOC): < 30 seconds
  - Medium tasks (100-500 LOC): < 60 seconds
  - Large tasks (500+ LOC): < 120 seconds
- Audit entry creation: < 100ms
- Coverage calculation: < 5 seconds for project (100K LOC)
- Workflow panel refresh: < 200ms
- Command execution: < 100ms (excluding actual work like generation)

### Scalability

- Large codebases: 100K+ LOC without degradation
- Spec count: 500+ specs without performance impact
- Audit entries: 10,000+ entries queryable in < 1 second
- Concurrent changes: 50+ active changes manageable
- File watching: Monitor 10,000+ files without lag

### Memory

- Extension baseline: < 50 MB RAM
- With large project loaded: < 200 MB RAM
- Audit index: < 100 MB RAM (for 10,000 entries)
- LSP server: < 50 MB RAM

### Availability

- Uptime: Extension should not crash; Zed's process isolation protects editor
- Error recovery: Graceful degradation on LLM API failures (fallback to manual)
- Offline mode: Core features (validation, workflow management) work offline
- LLM-dependent features (code generation) show clear offline state

### Performance Testing

- Load testing: 100K LOC project with 500 specs
- Stress testing: 10 concurrent code generations
- Memory leak testing: 24-hour continuous use
- Large file testing: 10K LOC spec file validation

---

## 12. Testing Strategy

### Testing Approach

**Unit Tests** (Rust)
- Coverage target: 80% for core logic
- Critical paths:
  - Spec parser (requirement extraction, delta parsing)
  - Validation rule engine
  - Audit entry creation and signing
  - Coverage calculation algorithm
  - LLM gateway (mocked API responses)
- Framework: Rust's built-in `cargo test`
- Mocking: `mockall` crate for LLM APIs
- Execution: CI/CD on every commit

**Integration Tests** (Rust + WASM)
- Scenarios:
  1. Full workflow: Create proposal → Validate → Generate code → Archive
  2. LLM failover: Primary fails → Fallback succeeds
  3. Audit integrity: Create entry → Verify signature → Detect tampering
  4. Coverage calculation: Generate code → Verify coverage updates
  5. File watching: Modify spec → Validation triggers
- Dependencies: Mock file system, mock LLM APIs
- Framework: `cargo test --test integration`
- Execution: Nightly builds

**End-to-End Tests** (Zed + Extension)
- User flows:
  1. Install extension → Initialize OpenSpec → Create proposal → Generate code → Archive
  2. Collaborate on spec with teammate (multi-cursor)
  3. View audit trail → Filter by developer → Export
  4. Analyze coverage → Navigate to uncovered files
- Test environment: Zed preview build with test project
- Framework: Manual testing + automated UI tests (if Zed supports)
- Execution: Before each release

**Manual Testing**
- Exploratory testing: Edge cases (malformed specs, large files, network failures)
- User acceptance testing: Stakeholder validation against acceptance criteria
- Performance testing: Large project load testing
- LLM variety testing: Test with Claude, GPT-4, Ollama

### Test Data

- Test projects:
  - Small (100 files, 10K LOC)
  - Medium (1K files, 50K LOC)
  - Large (10K files, 100K LOC)
- Test specs: 50+ sample OpenSpec specs covering various patterns
- Test audit data: Pre-generated 1,000 audit entries for query testing

---

## 13. Error Handling & Edge Cases

### Error Scenarios

| Scenario | Expected Behavior | User-Facing Message |
|----------|------------------|---------------------|
| **OpenSpec CLI Not Installed** | Detect on init, show installation instructions | "OpenSpec CLI not found. Install: `npm install -g @fission-ai/openspec@latest`" |
| **LLM API Key Missing** | Prompt user to configure before generation | "LLM API key not configured. Go to Settings → OpenSpec → LLM Providers" |
| **LLM API Rate Limit** | Exponential backoff retry (3 attempts), then fail | "Rate limited by LLM provider. Retrying in 30s..." |
| **LLM Generation Timeout** | Abort after 120s, return partial results if available | "Generation timed out. Partial results available. Regenerate?" |
| **Network Failure During Generation** | Retry with backoff, fallback to secondary provider | "Network error. Trying fallback provider..." |
| **Invalid Spec Format** | Show validation errors inline, prevent generation | "Spec validation failed: Missing required Scenario blocks" |
| **Audit Signature Verification Failed** | Mark entry as tampered, alert user | "⚠️ Audit entry {id} failed integrity check (possible tampering)" |
| **Disk Full** | Fail gracefully, don't lose data | "Cannot write audit entry: Disk full" |
| **File Locked** | Retry write (3 attempts), then fail | "Cannot write to {file}: File is locked" |
| **Concurrent Edit Conflict** | Detect via file modification time, prompt user | "Spec was modified externally. Reload to see latest?" |
| **Coverage Calculation Error** | Log error, show stale coverage | "Coverage calculation failed for {file}. Using cached data." |

### Edge Cases

- **Empty Specs**: Allow but warn ("Spec has no requirements")
- **Circular Spec References**: Detect and warn, but don't block
- **Very Large Specs**: > 1MB → Warn about performance impact
- **Very Long Generation**: > 10,000 tokens → Warn about cost
- **Malformed Tasks.md**: Show validation errors, can't apply change
- **Change Name Collision**: Prevent creation, suggest alternative name
- **Audit Entry Too Large**: > 10MB → Compress or split
- **LLM Returns Invalid Code**: Syntax check before showing diff, reject if invalid
- **Developer Changes Git Identity**: New signatures with new identity, old entries remain valid
- **Offline Mode**: Disable LLM generation, show clear state
- **Zed Version Incompatibility**: Detect on load, show upgrade prompt

### Monitoring & Alerting

- Metrics to track:
  - LLM API success rate (per provider)
  - LLM API latency (p50, p95, p99)
  - Validation execution time
  - Audit entry write failures
  - Coverage calculation failures
  - Extension crash rate
  - Command execution time
  - Memory usage
- Alert thresholds:
  - LLM API failure rate > 10%: Alert
  - Validation latency p95 > 2 seconds: Warning
  - Extension crashes > 5 per hour: Critical
  - Audit write failures > 0: Critical
- Logging strategy:
  - Structured logging to Zed's log file
  - Log levels: ERROR, WARN, INFO, DEBUG
  - Include request IDs for tracing
  - Redact sensitive data (API keys, prompts with PII)

---

## 14. Dependencies & Integration

### External Dependencies

- **OpenSpec CLI** (`@fission-ai/openspec`):
  - Purpose: Core OpenSpec operations (init, list, archive, validate)
  - Version: >= 1.0.0 (ensure compatibility)
  - Installation: `npm install -g @fission-ai/openspec@latest`
  - Fallback: Extension detects missing CLI, prompts install

- **LLM Provider APIs**:
  - **Anthropic Claude API** (v1):
    - Purpose: Code generation
    - Authentication: API key
    - Rate Limits: TBD (respect 429 responses)
    - Endpoint: `https://api.anthropic.com/v1/messages`
  - **OpenAI API** (v1):
    - Purpose: Code generation (fallback or primary)
    - Authentication: API key
    - Rate Limits: Tier-based (respect 429)
    - Endpoint: `https://api.openai.com/v1/chat/completions`
  - **Azure OpenAI**:
    - Purpose: Enterprise LLM option
    - Authentication: API key + endpoint
  - **Ollama** (local):
    - Purpose: Offline code generation
    - Installation: User-managed
    - Endpoint: `http://localhost:11434`

- **Zed Extension API** (`zed_extension_api`):
  - Purpose: Interface with Zed editor
  - Version: Compatible with Zed Stable
  - Features: Commands, LSP, File system, UI components

### Internal Dependencies

- **Rust Crates**:
  - `tokio`: Async runtime (WASM-compatible)
  - `serde`, `serde_json`: Serialization
  - `reqwest`: HTTP client
  - `tower-lsp`: LSP implementation
  - `tree-sitter`: Code parsing
  - `ed25519-dalek`: Cryptographic signing
  - `sha2`: Hashing
  - `regex`: Spec parsing
  - `chrono`: Timestamps

### Backwards Compatibility

- **Extension Versioning**: Semantic versioning (MAJOR.MINOR.PATCH)
- **Config File**: Forward-compatible (ignore unknown fields)
- **Audit Schema**: Versioned (`schema_version: 1`)
- **OpenSpec CLI**: Support v1.x and v2.x (detect version)
- **Breaking Changes**: Document migration path in CHANGELOG
- **Deprecation**: 6-month notice for deprecated features

---

## 15. Deployment & Rollout Plan

### Deployment Strategy

- **Distribution**: Zed Extension Registry (via GitHub PR)
- **Installation**: Users install via Zed's Extensions panel
- **Updates**: Automatic via Zed's extension updater
- **Rollback**: Users can downgrade to previous version if issues occur

### Build & Release Process

1. **Development**: Local dev extension (Install Dev Extension)
2. **Testing**: Run full test suite (unit, integration, manual)
3. **Versioning**: Bump version in `extension.toml` and `Cargo.toml`
4. **Build**: `cargo build --release --target wasm32-wasi`
5. **Publish**: Open PR to `zed-industries/extensions` repo
6. **CI**: Zed's CI builds and validates extension
7. **Merge**: Zed team merges PR → Extension published to registry
8. **Announcement**: Post to Zed Discord, OpenSpec Discord, Twitter

### Rollout Phases

**Phase 1: Alpha (Internal Testing)** (Week 1-2)
- Audience: Development team (3 developers)
- Goals: Verify core functionality, catch critical bugs
- Success Criteria:
  - No crashes during normal use
  - All P0 features functional
  - Performance targets met
- Feedback: Daily standups, GitHub issues

**Phase 2: Beta (Community Testing)** (Week 3-6)
- Audience: OpenSpec community volunteers (20-30 users)
- Goals: Real-world usage, gather feedback, identify edge cases
- Success Criteria:
  - User satisfaction > 3.5/5
  - No critical bugs
  - Adoption rate > 50% of beta users
- Feedback: Surveys, GitHub issues, Discord discussions

**Phase 3: General Availability** (Week 7+)
- Audience: All Zed users
- Goals: Full public release
- Success Criteria:
  - Extension listed on Zed Extensions Registry
  - Documentation complete
  - No known critical issues
  - Positive community reception
- Support: GitHub issues, Discord channel

### Feature Flags

(Note: Zed extensions don't have built-in feature flags; use config file)

- **`llm.enabled`**: Enable/disable LLM code generation
- **`audit.enabled`**: Enable/disable audit trail
- **`validation.enabled`**: Enable/disable real-time validation
- **`coverage.enabled`**: Enable/disable coverage analysis

---

## 16. Timeline & Milestones

| Milestone | Description | Owner | Target Date | Status |
|-----------|-------------|-------|-------------|--------|
| Spec Complete | This spec approved by stakeholders | Product Lead | Oct 27, 2025 | In Progress |
| Architecture Review | Technical design reviewed | Tech Lead | Oct 30, 2025 | Pending |
| Dev Environment Setup | Rust toolchain, Zed dev extension, test project | Dev Team | Nov 1, 2025 | Pending |
| Core Extension Scaffolding | Zed extension structure, command registration | Dev Team | Nov 8, 2025 | Pending |
| OpenSpec CLI Integration | Execute OpenSpec commands from extension | Dev Team | Nov 15, 2025 | Pending |
| LSP Server (Validation) | Real-time spec validation | Dev Team | Nov 29, 2025 | Pending |
| LLM Gateway | Multi-provider code generation | Dev Team | Dec 13, 2025 | Pending |
| Audit Engine | Immutable audit trail with signatures | Dev Team | Dec 20, 2025 | Pending |
| Workflow Manager | Sidebar panel for change management | Dev Team | Dec 27, 2025 | Pending |
| Coverage Analyzer | Spec coverage calculation and visualization | Dev Team | Jan 10, 2026 | Pending |
| UI Polish | Themes, icons, animations | UI/UX | Jan 17, 2026 | Pending |
| Testing & QA | Full test suite, manual testing | QA Team | Jan 24, 2026 | Pending |
| Documentation | User guide, API docs, tutorials | Docs Team | Jan 31, 2026 | Pending |
| Alpha Release | Internal testing | Dev Team | Feb 1, 2026 | Pending |
| Beta Release | Community testing | Dev Team | Feb 15, 2026 | Pending |
| GA Launch | Public release on Zed registry | Dev Team | Mar 1, 2026 | Pending |

**Critical Path**:
- OpenSpec CLI Integration → LLM Gateway → Audit Engine (sequential)
- LSP Server parallel with CLI Integration
- Workflow Manager and Coverage Analyzer can proceed in parallel after CLI/LLM work

---

## 17. Risks & Mitigation

| Risk | Impact | Probability | Mitigation Strategy |
|------|--------|-------------|---------------------|
| **Zed Extension API Limitations** | Cannot implement all features as designed | Medium | Prototype early to validate API capabilities; engage with Zed team for feature requests |
| **LLM API Instability** | Code generation failures, user frustration | High | Implement robust retry logic, fallback providers, clear error messages |
| **Performance Degradation on Large Projects** | Slow validation, coverage calculation | Medium | Optimize algorithms (incremental parsing, caching), performance testing with 100K LOC projects |
| **Audit Data Growth** | Disk space exhaustion, slow queries | Medium | Implement compression, archival policy, retention limits, warn users at 1GB |
| **OpenSpec CLI Breaking Changes** | Extension breaks on OpenSpec updates | Low | Version detection, support multiple CLI versions, document compatibility matrix |
| **Security Vulnerabilities** | Exploits via WASM sandbox escape | Low | Regular security audits, dependency scanning, follow Zed security guidelines |
| **Low Adoption** | Users don't switch from current workflow | High | Focus on developer experience, clear onboarding, showcase value (time savings, audit trail) |
| **LLM Prompt Injection** | Malicious specs generate harmful code | Low | Sanitize specs before sending to LLM, validate generated code syntax, user review required |
| **Collaboration Conflicts** | Multi-user editing causes data loss | Medium | Leverage Zed's CRDT-based collaboration, document conflict resolution patterns |
| **Audit Signature Key Loss** | Cannot verify historical entries | Low | Backup keys to secure location, document key recovery process, grace period for re-signing |

---

## 18. Open Questions

- [ ] **Q1: Zed Extension API Capabilities**
  - **Question**: Does Zed Extension API support all required features (LSP, panels, decorations, file watching)?
  - **Owner**: Tech Lead
  - **Due**: October 28, 2025
  - **Resolution**: Pending prototype validation

- [ ] **Q2: LLM Provider SLAs**
  - **Question**: What are guaranteed rate limits and SLAs from Anthropic, OpenAI?
  - **Owner**: Product Manager
  - **Due**: October 30, 2025
  - **Resolution**: Pending provider documentation review

- [ ] **Q3: Audit Data Retention Policy**
  - **Question**: Should we enforce automatic archival/deletion after 2 years?
  - **Owner**: Security Lead
  - **Due**: October 29, 2025
  - **Resolution**: Pending compliance review

- [ ] **Q4: Local LLM Performance**
  - **Question**: Can local models (Ollama) generate code fast enough for good UX?
  - **Owner**: Dev Lead
  - **Due**: November 10, 2025 (after prototype)
  - **Resolution**: Pending performance testing

- [ ] **Q5: Extension Distribution License**
  - **Question**: Which open-source license? (MIT, Apache 2.0, GPL)
  - **Owner**: Legal/Product
  - **Due**: October 27, 2025
  - **Resolution**: Pending decision (recommend MIT for broad adoption)

---

## 19. Alternatives Considered

**Alternative 1: VSCode Extension Instead of Zed**

- **Description**: Build the same features for VSCode (larger user base)
- **Pros**:
  - Larger market (millions of VSCode users)
  - Mature extension API
  - Extensive documentation and community support
- **Cons**:
  - Highly competitive extension marketplace
  - VSCode extensions for OpenSpec already exist (less differentiation)
  - Zed offers unique collaborative features and performance
  - Missing opportunity to strengthen Zed ecosystem
- **Why not chosen**: Zed is strategic platform for future; first-mover advantage

**Alternative 2: Standalone Desktop App (Electron)**

- **Description**: Build separate desktop app for OpenSpec workflow management
- **Pros**:
  - Full control over UI/UX
  - Can integrate more deeply with system (file watchers, Git)
  - Not constrained by editor APIs
- **Cons**:
  - Requires context switching (defeats main goal)
  - Significant additional development (authentication, file management, editor)
  - Harder to integrate with developer workflow
  - Maintenance burden of full application
- **Why not chosen**: Extension approach keeps users in flow; less development effort

**Alternative 3: Web-based Spec Management Platform**

- **Description**: Build cloud-hosted web app for OpenSpec management (like Notion integration)
- **Pros**:
  - Accessible from anywhere
  - Easier collaboration (no local files)
  - Can add features not possible in editor
  - Centralized audit trail
- **Cons**:
  - Requires sync with local files (complexity)
  - Privacy concerns (specs on third-party servers)
  - Subscription model required for sustainability
  - Less integrated with code editing workflow
- **Why not chosen**: OpenSpec philosophy is local-first, offline-capable

**Alternative 4: Terminal UI (TUI) Instead of Editor Extension**

- **Description**: Build rich terminal interface for OpenSpec (like `lazygit`)
- **Pros**:
  - Works across all editors
  - Simpler implementation (no WASM)
  - Keyboard-driven power user experience
- **Cons**:
  - Still requires context switching
  - No inline validation in editor
  - Limited to terminal-savvy users
  - Cannot integrate with editor collaboration features
- **Why not chosen**: Extension approach provides better UX and deeper integration

---

## 20. Future Considerations

Features out of scope for Phase 1 but worth considering for future releases:

- **Reverse Engineering (Code → Spec)**: Analyze existing code and generate spec deltas
- **AI Pair Programming Mode**: Real-time AI suggestions as developer writes specs
- **Spec Templates Marketplace**: Community-shared spec templates for common patterns
- **Multi-Workspace Management**: Manage OpenSpec across multiple projects
- **Advanced Analytics Dashboard**: Trend analysis (coverage over time, generation success rates)
- **Custom LLM Fine-Tuning**: Train model on team's historical specs and code
- **GitHub Integration**: Sync specs with GitHub Issues, PRs, and Project boards
- **Jira/Linear Integration**: Link specs to tickets for bidirectional traceability
- **Mobile Companion App**: View specs, review audit trail on mobile devices
- **Spec Diff Tool**: Visual diff for comparing spec versions (like Git diff)
- **Automated Spec Review**: AI-powered spec review (completeness, clarity, consistency)
- **Test Generation from Specs**: Automatically generate test cases from scenarios
- **Spec-to-Documentation**: Generate API docs, user guides from specs
- **Multi-Language Spec Support**: Specs in languages other than English
- **Voice Input for Specs**: Dictate specs using speech recognition
- **Spec Linting**: Style guide enforcement for specs (like ESLint for code)
- **Collaborative Spec Workshops**: Facilitated real-time spec writing sessions
- **Spec Approval Workflows**: Formal approval process with e-signatures
- **Compliance Reporting**: Automated SOC 2, GDPR compliance reports from audit data
- **AI Model Comparison**: A/B test code generation across different LLMs

---

## 21. Appendix

### Glossary

- **Audit Entry**: Immutable record of spec → code transformation
- **Change**: OpenSpec unit of work (proposal + tasks + spec deltas)
- **Coverage**: Percentage of codebase generated from specs vs ad-hoc
- **LSP**: Language Server Protocol - enables editor features like validation
- **OpenSpec**: Spec-driven development framework for AI collaboration
- **Spec Delta**: Diff showing changes to a spec (ADDED, MODIFIED, REMOVED)
- **Task**: Individual implementation unit from tasks.md
- **WASM**: WebAssembly - binary format for running code in sandboxed environment
- **Zed**: Fast, collaborative code editor built on Rust

### References

- Zed Extension API: https://zed.dev/docs/extensions/developing-extensions
- OpenSpec GitHub: https://github.com/Fission-AI/OpenSpec
- OpenSpec Documentation: https://github.com/Fission-AI/OpenSpec#readme
- Anthropic Claude API: https://docs.anthropic.com/
- OpenAI API: https://platform.openai.com/docs/
- Ed25519 Signatures: https://ed25519.cr.yp.to/
- LSP Specification: https://microsoft.github.io/language-server-protocol/

### Related Specifications

- OpenSpec CLI Specification: (OpenSpec project documentation)
- Zed Extension Guidelines: https://zed.dev/docs/extensions
- Notion-OpenSpec Integration Spec: (see project documentation)

### Meeting Notes

- **October 25, 2025**: Initial spec review
  - Decisions: Prioritize LLM integration and audit trail; defer advanced analytics
  - Attendees: Product Lead, Tech Lead, OpenSpec maintainer
  - Next: Architecture review October 30

---

## Change Log

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | Oct 25, 2025 | Development Team | Initial comprehensive specification including: OpenSpec integration, LLM code generation, audit trail, coverage analysis, workflow management, UI designs, technical architecture, security considerations, testing strategy, and rollout plan |

