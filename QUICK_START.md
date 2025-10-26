# Zed-OpenSpec Extension: Developer Quick-Start Guide

## Document Information
- **Document Type:** Developer Quick-Start Guide
- **Version:** 1.0
- **Date:** October 25, 2025
- **Purpose:** Practical guide for developers building the Zed-OpenSpec extension

---

## Table of Contents

1. [Getting Started](#1-getting-started)
2. [Project Structure](#2-project-structure)
3. [Development Workflow](#3-development-workflow)
4. [Key Implementation Examples](#4-key-implementation-examples)
5. [Testing Your Extension](#5-testing-your-extension)
6. [Debugging & Troubleshooting](#6-debugging--troubleshooting)
7. [Deployment Checklist](#7-deployment-checklist)
8. [Workflow Diagrams](#8-workflow-diagrams)

---

## 1. Getting Started

### Prerequisites

```bash
# 1. Install Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi

# 2. Install OpenSpec CLI
npm install -g @fission-ai/openspec@latest

# 3. Install Zed editor
# Download from https://zed.dev

# 4. Verify installations
rustc --version      # Should show rustc 1.75+
openspec --version   # Should show 1.0+
zed --version        # Should show Zed version
```

### Create Extension Project

```bash
# Create project directory
mkdir zed-openspec-extension
cd zed-openspec-extension

# Initialize Git
git init

# Create extension structure
mkdir -p src languages themes

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "zed-openspec"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["sync"] }
anyhow = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
ed25519-dalek = "2.0"
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
futures = "0.3"
EOF

# Create extension.toml
cat > extension.toml << 'EOF'
id = "openspec"
name = "OpenSpec"
version = "0.1.0"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "OpenSpec integration with LLM-powered code generation and audit trails"
repository = "https://github.com/yourusername/zed-openspec-extension"
EOF

# Create basic lib.rs
cat > src/lib.rs << 'EOF'
use zed_extension_api as zed;

struct OpenSpecExtension;

impl zed::Extension for OpenSpecExtension {
    fn new() -> Self {
        OpenSpecExtension
    }
}

zed::register_extension!(OpenSpecExtension);
EOF
```

### Build and Install (Dev Mode)

```bash
# Build the extension
cargo build --release --target wasm32-wasi

# In Zed:
# 1. Open Command Palette (Cmd+Shift+P / Ctrl+Shift+P)
# 2. Run: "zed: install dev extension"
# 3. Select this directory

# Verify installation
# The extension should appear in Extensions panel
```

---

## 2. Project Structure

```
zed-openspec-extension/
├── Cargo.toml                    # Rust dependencies
├── extension.toml                # Extension metadata
├── README.md                     # Documentation
├── LICENSE                       # MIT or Apache 2.0
│
├── src/
│   ├── lib.rs                    # Extension entry point
│   ├── commands/                 # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs              # openspec: init
│   │   ├── proposal.rs          # openspec: new proposal
│   │   ├── apply.rs             # openspec: apply change
│   │   └── archive.rs           # openspec: archive change
│   │
│   ├── lsp/                      # LSP server for validation
│   │   ├── mod.rs
│   │   ├── server.rs            # LSP server implementation
│   │   ├── validator.rs         # Spec validation logic
│   │   └── diagnostics.rs       # Diagnostic generation
│   │
│   ├── llm/                      # LLM integration
│   │   ├── mod.rs
│   │   ├── gateway.rs           # LLM Gateway
│   │   ├── providers/
│   │   │   ├── mod.rs
│   │   │   ├── claude.rs        # Claude provider
│   │   │   ├── openai.rs        # OpenAI provider
│   │   │   └── ollama.rs        # Ollama provider
│   │   └── context.rs           # Context builder
│   │
│   ├── audit/                    # Audit trail system
│   │   ├── mod.rs
│   │   ├── engine.rs            # Audit engine
│   │   ├── entry.rs             # Audit entry types
│   │   ├── index.rs             # Audit indexing
│   │   └── signature.rs         # Cryptographic signing
│   │
│   ├── workflow/                 # Workflow management
│   │   ├── mod.rs
│   │   ├── manager.rs           # Workflow manager
│   │   ├── change.rs            # Change types
│   │   └── state.rs             # State machine
│   │
│   ├── coverage/                 # Coverage analysis
│   │   ├── mod.rs
│   │   ├── analyzer.rs          # Coverage calculator
│   │   └── visualizer.rs        # UI components
│   │
│   ├── ui/                       # UI components
│   │   ├── mod.rs
│   │   ├── workflow_panel.rs    # Sidebar panel
│   │   ├── audit_viewer.rs      # Audit viewer
│   │   └── coverage_panel.rs    # Coverage panel
│   │
│   └── utils/                    # Utilities
│       ├── mod.rs
│       ├── config.rs            # Config management
│       ├── fs.rs                # File system helpers
│       └── errors.rs            # Error types
│
├── tests/                        # Integration tests
│   ├── integration_test.rs
│   └── fixtures/
│       └── sample_project/
│
└── docs/                         # Documentation
    ├── architecture.md
    ├── api.md
    └── user_guide.md
```

---

## 3. Development Workflow

### Step-by-Step Development Process

#### Phase 1: Foundation (Week 1-2)

**Day 1-2: Extension Scaffolding**
```rust
// src/lib.rs
use zed_extension_api as zed;

struct OpenSpecExtension {
    // Extension state
}

impl zed::Extension for OpenSpecExtension {
    fn new() -> Self {
        Self {}
    }
    
    fn command(
        &mut self,
        command: String,
        _args: Vec<String>,
        _worktree: &zed::Worktree,
    ) -> Result<String, String> {
        match command.as_str() {
            "openspec:init" => Ok("Extension loaded!".to_string()),
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}

zed::register_extension!(OpenSpecExtension);
```

**Day 3-5: OpenSpec CLI Integration**
```rust
// src/commands/init.rs
use std::process::Command;

pub async fn init_openspec(project_root: &Path) -> Result<String> {
    let output = Command::new("openspec")
        .arg("init")
        .current_dir(project_root)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
```

#### Phase 2: LSP Server (Week 3-4)

**Implement Basic LSP Server**
```rust
// src/lsp/server.rs
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct OpenSpecLspServer {
    client: Client,
    validator: SpecValidator,
}

#[tower_lsp::async_trait]
impl LanguageServer for OpenSpecLspServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }
    
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.content_changes[0].text.clone();
        
        // Validate spec
        let diagnostics = self.validator.validate(&text);
        
        // Publish diagnostics
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}
```

#### Phase 3: LLM Integration (Week 5-6)

**Implement Claude Provider**
```rust
// src/llm/providers/claude.rs
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct ClaudeProvider {
    client: Client,
    api_key: String,
}

#[async_trait]
impl LLMProvider for ClaudeProvider {
    async fn generate_code(
        &self,
        context: &GenerationContext,
    ) -> Result<GenerationResult> {
        let prompt = self.build_prompt(context);
        
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-sonnet-4-20250514",
                "max_tokens": 4000,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await?;
        
        let data: ClaudeResponse = response.json().await?;
        
        Ok(GenerationResult {
            code: data.content[0].text.clone(),
            model: data.model,
            tokens_used: TokenUsage {
                input: data.usage.input_tokens,
                output: data.usage.output_tokens,
            },
        })
    }
}
```

---

## 4. Key Implementation Examples

### Example 1: Command Handler

```rust
// src/lib.rs
impl zed::Extension for OpenSpecExtension {
    fn command(
        &mut self,
        command: String,
        args: Vec<String>,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        runtime.block_on(async {
            match command.as_str() {
                "openspec:new-proposal" => {
                    let name = args.get(0)
                        .ok_or("Proposal name required")?;
                    
                    commands::proposal::create_proposal(
                        &worktree.root_path(),
                        name,
                    ).await
                }
                
                "openspec:apply-change" => {
                    let change_id = args.get(0)
                        .ok_or("Change ID required")?;
                    
                    let llm_provider = args.get(1)
                        .map(|s| s.as_str())
                        .unwrap_or("claude");
                    
                    commands::apply::apply_change(
                        &worktree.root_path(),
                        change_id,
                        llm_provider,
                    ).await
                }
                
                "openspec:view-audit" => {
                    ui::audit_viewer::show_audit_panel(
                        &worktree.root_path(),
                    ).await
                }
                
                _ => Err(format!("Unknown command: {}", command)),
            }
        })
    }
}
```

### Example 2: Spec Validation

```rust
// src/lsp/validator.rs
use regex::Regex;

pub struct SpecValidator {
    requirement_regex: Regex,
    scenario_regex: Regex,
}

impl SpecValidator {
    pub fn new() -> Self {
        Self {
            requirement_regex: Regex::new(r"###\s+Requirement:\s+(.+)").unwrap(),
            scenario_regex: Regex::new(r"####\s+Scenario:\s+(.+)").unwrap(),
        }
    }
    
    pub fn validate(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // Check for requirements without scenarios
        let requirements = self.find_requirements(content);
        
        for req in requirements {
            if !self.has_scenarios(content, &req) {
                diagnostics.push(Diagnostic {
                    range: req.range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: format!(
                        "Requirement '{}' has no scenarios",
                        req.title
                    ),
                    ..Default::default()
                });
            }
        }
        
        // Check for SHALL/MUST in requirements
        for req in requirements {
            if !req.description.contains("SHALL") 
                && !req.description.contains("MUST") {
                diagnostics.push(Diagnostic {
                    range: req.range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: "Requirements should use SHALL or MUST".to_string(),
                    ..Default::default()
                });
            }
        }
        
        diagnostics
    }
}
```

### Example 3: Audit Entry Creation

```rust
// src/audit/engine.rs
impl AuditEngine {
    pub async fn record_generation(
        &self,
        context: &GenerationContext,
        result: &GenerationResult,
        acceptance: &AcceptanceDetails,
    ) -> Result<Uuid> {
        // Build entry
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            change_id: context.change_id.clone(),
            task_ids: vec![context.task.id.clone()],
            developer: self.get_developer_info()?,
            llm: LLMDetails {
                provider: result.provider.clone(),
                model: result.model.clone(),
                temperature: context.temperature,
                max_tokens: context.max_tokens,
                actual_tokens_used: result.tokens_used.clone(),
            },
            generation: GenerationDetails {
                generated_code: result.code.clone(),
                code_hash: sha256_hash(&result.code),
                language: detect_language(&result.code),
                syntax_valid: syntax_check(&result.code),
            },
            acceptance: acceptance.clone(),
            signature: Signature::default(),
            // ... other fields
        };
        
        // Sign
        let signature = self.sign_entry(&entry)?;
        let entry = AuditEntry { signature, ..entry };
        
        // Write to disk
        let filename = format!(
            "{}-{}.json",
            entry.timestamp.format("%Y%m%d_%H%M%S"),
            entry.id
        );
        
        let json = serde_json::to_string_pretty(&entry)?;
        fs::write(self.audit_dir.join(&filename), json)?;
        
        // Update index
        self.index.write().await.add_entry(&entry)?;
        
        Ok(entry.id)
    }
}
```

### Example 4: Workflow Panel UI

```rust
// src/ui/workflow_panel.rs
use zed_extension_api::*;

pub struct WorkflowPanel {
    changes: Vec<ChangeStateMachine>,
}

impl WorkflowPanel {
    pub fn render(&self) -> PanelContent {
        let mut content = String::new();
        
        content.push_str("# OpenSpec Changes\n\n");
        
        for change in &self.changes {
            let status_icon = match change.state {
                ChangeState::Proposed => "🔵",
                ChangeState::InProgress { .. } => "🟡",
                ChangeState::ReadyToArchive => "🟢",
                ChangeState::Archived { .. } => "⚫",
            };
            
            let progress = change.progress_percentage();
            
            content.push_str(&format!(
                "{} **{}** ({}%)\n",
                status_icon,
                change.change.name,
                progress as u32
            ));
            
            // Task list
            for task in &change.change.tasks {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                content.push_str(&format!(
                    "  {} {} {}\n",
                    checkbox,
                    task.id,
                    task.description
                ));
            }
            
            content.push_str("\n");
        }
        
        PanelContent::Markdown(content)
    }
}
```

---

## 5. Testing Your Extension

### Unit Tests

```rust
// tests/integration_test.rs
use zed_openspec::*;

#[tokio::test]
async fn test_spec_validation() {
    let validator = SpecValidator::new();
    
    let content = r#"
### Requirement: User Authentication
The system SHALL authenticate users.

#### Scenario: Valid credentials
- WHEN user submits valid credentials
- THEN access is granted
"#;
    
    let diagnostics = validator.validate(content);
    assert_eq!(diagnostics.len(), 0);
}

#[tokio::test]
async fn test_missing_scenario() {
    let validator = SpecValidator::new();
    
    let content = r#"
### Requirement: User Authentication
The system SHALL authenticate users.
"#;
    
    let diagnostics = validator.validate(content);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].message, "Requirement 'User Authentication' has no scenarios");
}
```

### Integration Tests

```bash
#!/bin/bash
# tests/integration.sh

# Setup test project
mkdir -p test-project
cd test-project

# Initialize OpenSpec
openspec init

# Create sample proposal
openspec proposal add-auth

# Test extension commands
zed --test openspec:new-proposal test-feature
zed --test openspec:validate test-feature
```

---

## 6. Debugging & Troubleshooting

### Enable Debug Logging

```bash
# Start Zed with verbose logging
zed --foreground

# Logs will appear in terminal
```

### Common Issues

**Issue: Extension not loading**
```
Solution: Check Cargo.toml has correct crate-type:
[lib]
crate-type = ["cdylib"]

Rebuild:
cargo clean && cargo build --release --target wasm32-wasi
```

**Issue: Commands not appearing**
```
Solution: Verify command registration in extension.toml:
[commands]
"openspec:init" = "Initialize OpenSpec"
"openspec:new-proposal" = "New Proposal"
```

**Issue: LSP not working**
```
Solution: Check LSP server is running:
- Look for "LSP server started" in Zed log
- Verify file extensions match: .md files in openspec/
```

---

## 7. Deployment Checklist

### Pre-Release Checklist

- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Manual testing completed
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in extension.toml and Cargo.toml
- [ ] License file present (MIT or Apache 2.0)
- [ ] README.md comprehensive
- [ ] Examples provided

### Publishing Steps

```bash
# 1. Fork zed-industries/extensions repo
git clone https://github.com/YOUR_USERNAME/extensions
cd extensions

# 2. Add your extension as submodule
git submodule add https://github.com/YOUR_USERNAME/zed-openspec-extension.git extensions/openspec

# 3. Update extensions.toml
cat >> extensions.toml << EOF
[openspec]
submodule = "extensions/openspec"
version = "0.1.0"
EOF

# 4. Sort extensions
pnpm sort-extensions

# 5. Commit and push
git add .
git commit -m "Add OpenSpec extension"
git push

# 6. Create pull request
# Open PR on GitHub from your fork to zed-industries/extensions
```

---

## 8. Workflow Diagrams

### User Workflow: Creating and Applying Change

```
┌─────────────────────────────────────────────────────────────┐
│                    Developer Workflow                       │
└─────────────────────────────────────────────────────────────┘

1. Create Proposal
   │
   ├─► User: Cmd+Shift+P → "openspec: new proposal"
   ├─► Input: Proposal name (e.g., "add-2fa")
   ├─► Extension: Calls `openspec proposal add-2fa`
   └─► Result: Creates openspec/changes/add-2fa/
                - proposal.md
                - tasks.md
                - specs/

2. Edit Specs
   │
   ├─► User: Opens proposal.md and specs/ in Zed
   ├─► LSP: Real-time validation as user types
   ├─► Extension: Shows inline diagnostics
   └─► User: Iterates until validation passes

3. Apply Change
   │
   ├─► User: Cmd+Shift+P → "openspec: apply change"
   ├─► Extension: Prompts for change selection
   ├─► Extension: Prompts for LLM provider (Claude/GPT-4/Ollama)
   ├─► Extension: Shows token estimate & cost
   │
   ├─► User: Confirms generation
   │
   ├─► Extension: For each task:
   │   ├─► Builds generation context (spec + files)
   │   ├─► Calls LLM API
   │   ├─► Streams code to diff view
   │   ├─► User reviews diff
   │   └─► User accepts/rejects
   │
   ├─► If accepted:
   │   ├─► Extension: Writes files
   │   ├─► Extension: Creates audit entry
   │   └─► Extension: Marks task complete
   │
   └─► Result: Code generated, tracked, auditable

4. Archive Change
   │
   ├─► User: All tasks complete
   ├─► User: Cmd+Shift+P → "openspec: archive change"
   ├─► Extension: Calls `openspec archive add-2fa --yes`
   └─► Result: Change archived, specs updated
```

### Technical Flow: LLM Code Generation

```
┌─────────────────────────────────────────────────────────────┐
│            LLM Code Generation Pipeline                     │
└─────────────────────────────────────────────────────────────┘

User triggers apply
    │
    ▼
┌─────────────────────┐
│ Load Change Data    │
│ - Spec deltas       │
│ - Tasks list        │
│ - Context files     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Build Context       │
│ - Extract refs      │
│ - Prioritize files  │
│ - Fit token budget  │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Select LLM Provider │
│ - Primary: Claude   │
│ - Fallback: GPT-4   │
│ - Local: Ollama     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Generate Prompt     │
│ - Spec instructions │
│ - Task description  │
│ - Context files     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Call LLM API        │
│ - Streaming enabled │
│ - Retry with backoff│
│ - Timeout: 120s     │
└──────┬──────────────┘
       │
       ├─► Success ─────┐
       │                │
       └─► Failure      │
            │           │
            ▼           │
       ┌─────────────┐  │
       │  Try Fallback│  │
       │  Provider    │  │
       └──────┬───────┘  │
              │          │
              └──────────┘
                   │
                   ▼
          ┌─────────────────────┐
          │ Stream Code Chunks  │
          │ - Parse SSE         │
          │ - Accumulate code   │
          │ - Update progress   │
          └──────┬──────────────┘
                 │
                 ▼
          ┌─────────────────────┐
          │ Syntax Validation   │
          │ - Parse with        │
          │   tree-sitter       │
          │ - Reject if invalid │
          └──────┬──────────────┘
                 │
                 ▼
          ┌─────────────────────┐
          │ Show Diff View      │
          │ - Side-by-side      │
          │ - Syntax highlight  │
          │ - User review       │
          └──────┬──────────────┘
                 │
                 ├─► Accept ─────┐
                 │               │
                 └─► Reject      │
                      │          │
                      ▼          │
                   Cancel        │
                                 │
                    ┌────────────┘
                    │
                    ▼
          ┌─────────────────────┐
          │ Write Files         │
          │ - Create/modify     │
          │ - Apply changes     │
          └──────┬──────────────┘
                 │
                 ▼
          ┌─────────────────────┐
          │ Create Audit Entry  │
          │ - Sign with Ed25519 │
          │ - Write JSON        │
          │ - Update index      │
          └──────┬──────────────┘
                 │
                 ▼
          ┌─────────────────────┐
          │ Mark Task Complete  │
          │ - Update tasks.md   │
          │ - Notify workflow   │
          └─────────────────────┘
```

### Data Flow: Audit Trail

```
┌─────────────────────────────────────────────────────────────┐
│                 Audit Trail Data Flow                       │
└─────────────────────────────────────────────────────────────┘

Code Generation
    │
    ├─► Capture:
    │   - Timestamp
    │   - Developer (from Git)
    │   - LLM provider & model
    │   - Prompt (hashed)
    │   - Generated code (hashed)
    │   - Files modified
    │
    ▼
Build Audit Entry
    │
    ├─► Serialize to JSON
    │   (except signature)
    │
    ▼
Sign Entry
    │
    ├─► Hash entry data (SHA-256)
    ├─► Sign hash (Ed25519)
    ├─► Add signature to entry
    │
    ▼
Write to Disk
    │
    ├─► Filename: YYYYMMDD_HHMMSS-{uuid}.json
    ├─► Location: .openspec/audit/
    ├─► Append-only (no overwrites)
    │
    ▼
Update Index
    │
    ├─► In-memory index:
    │   - by_developer: Map<String, Vec<Uuid>>
    │   - by_change: Map<String, Vec<Uuid>>
    │   - by_llm: Map<String, Vec<Uuid>>
    │   - by_date: BTreeMap<DateTime, Vec<Uuid>>
    │
    ▼
Query Audit
    │
    ├─► Fast lookup via index
    ├─► Load entries from disk
    ├─► Verify signatures
    │
    ▼
Audit Viewer
    │
    ├─► Display in table
    ├─► Filter/search
    ├─► Export to JSON/CSV
```

---

## Quick Commands Reference

```bash
# Development
cargo build --release --target wasm32-wasi    # Build extension
cargo test                                     # Run tests
cargo clippy                                   # Lint code

# Zed Commands (in editor)
Cmd+Shift+P → "openspec: init"                # Initialize OpenSpec
Cmd+Shift+P → "openspec: new proposal"        # Create proposal
Cmd+Shift+P → "openspec: apply change"        # Generate code
Cmd+Shift+P → "openspec: archive change"      # Archive change
Cmd+Shift+P → "openspec: view audit"          # View audit trail
Cmd+Shift+P → "openspec: show coverage"       # Show coverage

# OpenSpec CLI (terminal)
openspec init                                  # Initialize project
openspec list                                  # List changes
openspec show <change>                         # Show change details
openspec validate <change>                     # Validate specs
openspec archive <change> --yes                # Archive change
```

---

## Resources

- **Zed Extension API**: https://zed.dev/docs/extensions/developing-extensions
- **OpenSpec GitHub**: https://github.com/Fission-AI/OpenSpec
- **OpenSpec Docs**: https://github.com/Fission-AI/OpenSpec#readme
- **Rust WASM Book**: https://rustwasm.github.io/docs/book/
- **Tower-LSP**: https://github.com/ebkalderon/tower-lsp
- **Ed25519**: https://docs.rs/ed25519-dalek/

---

## Next Steps

1. **Start Simple**: Begin with command integration (init, list, show)
2. **Add Validation**: Implement LSP server for real-time spec validation
3. **Integrate LLM**: Add Claude provider for code generation
4. **Build Audit System**: Implement audit trail with signatures
5. **Add UI Components**: Create workflow panel and audit viewer
6. **Test Thoroughly**: Write comprehensive tests
7. **Polish UX**: Refine error messages, progress indicators
8. **Document**: Write user guide and API docs
9. **Publish**: Open PR to zed-industries/extensions

---

## Support

- **Issues**: https://github.com/yourusername/zed-openspec-extension/issues
- **Discussions**: https://github.com/yourusername/zed-openspec-extension/discussions
- **Discord**: Join OpenSpec Discord for help

---

**Happy coding! 🚀**

