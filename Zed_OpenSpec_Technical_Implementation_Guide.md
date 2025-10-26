# Zed-OpenSpec Extension: Technical Implementation Guide

## Document Information
- **Document Type:** Technical Implementation Guide (Supplementary to Main Spec)
- **Version:** 1.0
- **Date:** October 25, 2025
- **Purpose:** Deep-dive technical guidance for implementing LLM integration, audit systems, and workflow tracking

---

## Table of Contents

1. [LLM Integration Architecture](#1-llm-integration-architecture)
2. [Audit Trail System Design](#2-audit-trail-system-design)
3. [Workflow Tracking & State Management](#3-workflow-tracking--state-management)
4. [Code Generation Pipeline](#4-code-generation-pipeline)
5. [Coverage Analysis Implementation](#5-coverage-analysis-implementation)
6. [Performance Optimization Strategies](#6-performance-optimization-strategies)
7. [Error Recovery & Resilience](#7-error-recovery--resilience)
8. [Testing & Validation Framework](#8-testing--validation-framework)

---

## 1. LLM Integration Architecture

### 1.1 Provider Abstraction Layer

The extension uses a trait-based abstraction to support multiple LLM providers uniformly:

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Provider metadata
    fn name(&self) -> &str;
    fn model(&self) -> &str;
    
    /// Token estimation
    async fn estimate_tokens(&self, input: &str) -> Result<TokenEstimate>;
    
    /// Code generation
    async fn generate_code(
        &self,
        context: &GenerationContext,
    ) -> Result<Pin<Box<dyn Stream<Item = CodeChunk> + Send>>>;
    
    /// Health check
    async fn health_check(&self) -> Result<ProviderHealth>;
}

pub struct GenerationContext {
    pub spec_delta: SpecDelta,
    pub task: Task,
    pub context_files: Vec<FileContext>,
    pub additional_instructions: Option<String>,
    pub max_tokens: usize,
    pub temperature: f32,
}

pub struct CodeChunk {
    pub content: String,
    pub chunk_type: ChunkType, // Code | Explanation | Error
    pub metadata: ChunkMetadata,
}

pub struct TokenEstimate {
    pub input_tokens: usize,
    pub estimated_output_tokens: usize,
    pub estimated_cost_usd: f64,
}
```

### 1.2 Provider Implementations

#### Claude Provider

```rust
pub struct ClaudeProvider {
    client: Client,
    api_key: String,
    model: String,
    endpoint: String,
}

impl ClaudeProvider {
    pub fn new(config: ClaudeConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.api_key,
            model: config.model.unwrap_or("claude-sonnet-4-20250514".to_string()),
            endpoint: "https://api.anthropic.com/v1/messages".to_string(),
        }
    }
    
    async fn build_prompt(&self, context: &GenerationContext) -> String {
        format!(
            r#"You are an expert software engineer implementing code from specifications.

# Specification Delta
{}

# Task to Implement
Task {}: {}

# Context Files
{}

# Instructions
- Implement ONLY the task specified above
- Follow the specification requirements exactly
- Write clean, production-ready code
- Include error handling
- Add comments for complex logic
- Return ONLY the code, no explanations unless explicitly needed

Generate the implementation:"#,
            context.spec_delta.to_markdown(),
            context.task.id,
            context.task.description,
            self.format_context_files(&context.context_files)
        )
    }
}

#[async_trait]
impl LLMProvider for ClaudeProvider {
    fn name(&self) -> &str { "claude" }
    fn model(&self) -> &str { &self.model }
    
    async fn generate_code(
        &self,
        context: &GenerationContext,
    ) -> Result<Pin<Box<dyn Stream<Item = CodeChunk> + Send>>> {
        let prompt = self.build_prompt(context).await;
        
        let request = json!({
            "model": self.model,
            "max_tokens": context.max_tokens,
            "temperature": context.temperature,
            "stream": true,
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        });
        
        let response = self.client
            .post(&self.endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;
        
        // Convert SSE stream to CodeChunk stream
        let stream = response
            .bytes_stream()
            .map(|result| self.parse_sse_chunk(result))
            .filter_map(|result| async move { result.ok() });
        
        Ok(Box::pin(stream))
    }
    
    async fn estimate_tokens(&self, input: &str) -> Result<TokenEstimate> {
        // Approximate: 1 token ~= 4 characters
        let input_tokens = (input.len() as f64 / 4.0).ceil() as usize;
        let estimated_output = (input_tokens as f64 * 0.5).ceil() as usize;
        
        // Claude Sonnet 4 pricing (example)
        let input_cost = input_tokens as f64 * 0.003 / 1000.0;
        let output_cost = estimated_output as f64 * 0.015 / 1000.0;
        
        Ok(TokenEstimate {
            input_tokens,
            estimated_output_tokens: estimated_output,
            estimated_cost_usd: input_cost + output_cost,
        })
    }
}
```

#### Provider Registry & Fallback Chain

```rust
pub struct LLMGateway {
    providers: HashMap<String, Box<dyn LLMProvider>>,
    fallback_chain: Vec<String>,
    retry_config: RetryConfig,
}

impl LLMGateway {
    pub fn new() -> Self {
        let mut gateway = Self {
            providers: HashMap::new(),
            fallback_chain: vec![],
            retry_config: RetryConfig::default(),
        };
        
        // Register providers
        gateway.register_provider(Box::new(ClaudeProvider::new(/* config */)));
        gateway.register_provider(Box::new(OpenAIProvider::new(/* config */)));
        gateway.register_provider(Box::new(OllamaProvider::new(/* config */)));
        
        gateway
    }
    
    pub async fn generate_with_fallback(
        &self,
        context: &GenerationContext,
        primary_provider: &str,
    ) -> Result<GenerationResult> {
        let mut attempted_providers = vec![primary_provider.to_string()];
        
        // Try primary provider
        match self.try_generation(primary_provider, context).await {
            Ok(result) => return Ok(result),
            Err(e) if !e.is_retryable() => return Err(e),
            Err(e) => {
                log::warn!("Primary provider {} failed: {}", primary_provider, e);
            }
        }
        
        // Try fallback chain
        for provider_name in &self.fallback_chain {
            if attempted_providers.contains(provider_name) {
                continue;
            }
            
            attempted_providers.push(provider_name.clone());
            
            match self.try_generation(provider_name, context).await {
                Ok(result) => {
                    log::info!("Fallback to {} succeeded", provider_name);
                    return Ok(result);
                }
                Err(e) => {
                    log::warn!("Fallback provider {} failed: {}", provider_name, e);
                }
            }
        }
        
        Err(anyhow!("All providers failed. Attempted: {:?}", attempted_providers))
    }
    
    async fn try_generation(
        &self,
        provider_name: &str,
        context: &GenerationContext,
    ) -> Result<GenerationResult> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| anyhow!("Provider {} not found", provider_name))?;
        
        // Retry logic with exponential backoff
        let mut attempt = 0;
        let mut delay = self.retry_config.initial_delay;
        
        loop {
            attempt += 1;
            
            match provider.generate_code(context).await {
                Ok(stream) => {
                    return self.collect_generation(stream, provider_name).await;
                }
                Err(e) if attempt >= self.retry_config.max_attempts => {
                    return Err(anyhow!("Max retries exceeded: {}", e));
                }
                Err(e) if e.is_retryable() => {
                    log::warn!("Attempt {} failed, retrying in {:?}", attempt, delay);
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
                Err(e) => {
                    return Err(e); // Non-retryable error
                }
            }
        }
    }
}
```

### 1.3 Context Window Management

Large codebases can exceed LLM context limits. The extension intelligently selects relevant context:

```rust
pub struct ContextBuilder {
    max_tokens: usize,
    prioritizer: Box<dyn ContextPrioritizer>,
}

impl ContextBuilder {
    pub async fn build_context(
        &self,
        spec_delta: &SpecDelta,
        task: &Task,
        project: &Project,
    ) -> Result<Vec<FileContext>> {
        // Extract file references from spec and task
        let mut candidates = self.extract_file_candidates(spec_delta, task, project).await?;
        
        // Prioritize files by relevance
        self.prioritizer.prioritize(&mut candidates, spec_delta, task);
        
        // Fit within token budget
        let mut selected = vec![];
        let mut token_count = 0;
        
        for candidate in candidates {
            let file_tokens = self.estimate_file_tokens(&candidate);
            
            if token_count + file_tokens > self.max_tokens {
                break;
            }
            
            selected.push(candidate);
            token_count += file_tokens;
        }
        
        Ok(selected)
    }
}

pub trait ContextPrioritizer {
    fn prioritize(
        &self,
        candidates: &mut Vec<FileCandidate>,
        spec: &SpecDelta,
        task: &Task,
    );
}

pub struct SmartPrioritizer;

impl ContextPrioritizer for SmartPrioritizer {
    fn prioritize(
        &self,
        candidates: &mut Vec<FileCandidate>,
        spec: &SpecDelta,
        task: &Task,
    ) {
        candidates.sort_by(|a, b| {
            let score_a = self.calculate_relevance_score(a, spec, task);
            let score_b = self.calculate_relevance_score(b, spec, task);
            score_b.partial_cmp(&score_a).unwrap()
        });
    }
}
```

---

## 2. Audit Trail System Design

### 2.1 Audit Entry Schema (Detailed)

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct AuditEntry {
    // Identity
    pub id: Uuid,
    pub schema_version: u8, // Current: 1
    
    // Temporal
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    
    // Context
    pub change_id: String,
    pub task_ids: Vec<String>,
    
    // Developer
    pub developer: DeveloperInfo,
    
    // LLM Details
    pub llm: LLMDetails,
    
    // Generation Process
    pub prompt: PromptDetails,
    pub generation: GenerationDetails,
    
    // Code Changes
    pub changes: Vec<FileChange>,
    
    // Acceptance
    pub acceptance: AcceptanceDetails,
    
    // Integrity
    pub signature: Signature,
    
    // Metadata
    pub metadata: AuditMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeveloperInfo {
    pub email: String,
    pub name: String,
    pub git_commit: Option<String>,
    pub git_branch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LLMDetails {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub actual_tokens_used: TokenUsage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptDetails {
    pub spec_delta_hash: String,
    pub task_description: String,
    pub context_files: Vec<ContextFile>,
    pub full_prompt_hash: String, // SHA-256 of complete prompt
    pub prompt_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerationDetails {
    pub generated_code: String,
    pub code_hash: String, // SHA-256
    pub language: String,
    pub syntax_valid: bool,
    pub generation_tokens: usize,
    pub streamed_chunks: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileChange {
    pub path: PathBuf,
    pub action: FileAction,
    pub before_hash: Option<String>,
    pub after_hash: String,
    pub diff_unified: String,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub lines_modified: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileAction {
    Created,
    Modified,
    Deleted,
    Renamed { from: PathBuf },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptanceDetails {
    pub accepted: bool,
    pub accepted_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub modifications_made: bool,
    pub post_generation_edits: Option<String>, // Diff of user edits
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    pub algorithm: String, // "Ed25519"
    pub public_key: String,
    pub signature_base64: String,
    pub signed_data_hash: String, // Hash of all data except signature
}
```

### 2.2 Audit Engine Implementation

```rust
pub struct AuditEngine {
    audit_dir: PathBuf,
    signing_key: Ed25519KeyPair,
    index: Arc<RwLock<AuditIndex>>,
}

impl AuditEngine {
    pub fn new(project_root: &Path) -> Result<Self> {
        let audit_dir = project_root.join(".openspec").join("audit");
        fs::create_dir_all(&audit_dir)?;
        
        let signing_key = Self::load_or_create_signing_key()?;
        let index = Arc::new(RwLock::new(AuditIndex::load_or_create(&audit_dir)?));
        
        Ok(Self {
            audit_dir,
            signing_key,
            index,
        })
    }
    
    pub async fn record_generation(
        &self,
        context: &GenerationContext,
        result: &GenerationResult,
        acceptance: &AcceptanceDetails,
    ) -> Result<AuditEntryId> {
        // Build audit entry
        let mut entry = AuditEntry {
            id: Uuid::new_v4(),
            schema_version: 1,
            timestamp: Utc::now(),
            duration_ms: result.duration.as_millis() as u64,
            change_id: context.change_id.clone(),
            task_ids: context.task_ids.clone(),
            developer: self.get_developer_info()?,
            llm: self.build_llm_details(context, result),
            prompt: self.build_prompt_details(context),
            generation: self.build_generation_details(result),
            changes: self.build_file_changes(result),
            acceptance: acceptance.clone(),
            signature: Signature::default(), // Filled below
            metadata: self.build_metadata(),
        };
        
        // Sign entry
        entry.signature = self.sign_entry(&entry)?;
        
        // Write to disk (append-only)
        let filename = format!("{}-{}.json", 
            entry.timestamp.format("%Y%m%d_%H%M%S"),
            entry.id
        );
        let filepath = self.audit_dir.join(&filename);
        
        let json = serde_json::to_string_pretty(&entry)?;
        fs::write(&filepath, json)?;
        
        // Update index
        self.index.write().await.add_entry(&entry, &filepath)?;
        
        Ok(entry.id)
    }
    
    fn sign_entry(&self, entry: &AuditEntry) -> Result<Signature> {
        // Serialize entry without signature
        let mut entry_copy = entry.clone();
        entry_copy.signature = Signature::default();
        
        let data = serde_json::to_vec(&entry_copy)?;
        let hash = Sha256::digest(&data);
        
        // Sign hash
        let signature = self.signing_key.sign(&hash);
        
        Ok(Signature {
            algorithm: "Ed25519".to_string(),
            public_key: base64::encode(self.signing_key.public_key()),
            signature_base64: base64::encode(signature.to_bytes()),
            signed_data_hash: hex::encode(hash),
        })
    }
    
    pub async fn verify_entry(&self, entry: &AuditEntry) -> Result<VerificationResult> {
        // Reconstruct signed data
        let mut entry_copy = entry.clone();
        entry_copy.signature = Signature::default();
        
        let data = serde_json::to_vec(&entry_copy)?;
        let hash = Sha256::digest(&data);
        
        // Verify hash matches
        if hex::encode(&hash) != entry.signature.signed_data_hash {
            return Ok(VerificationResult::HashMismatch);
        }
        
        // Verify signature
        let public_key = PublicKey::from_bytes(
            &base64::decode(&entry.signature.public_key)?
        )?;
        
        let signature = ed25519_dalek::Signature::from_bytes(
            &base64::decode(&entry.signature.signature_base64)?
        )?;
        
        match public_key.verify(&hash, &signature) {
            Ok(_) => Ok(VerificationResult::Valid),
            Err(_) => Ok(VerificationResult::InvalidSignature),
        }
    }
    
    pub async fn query(
        &self,
        filter: AuditFilter,
    ) -> Result<Vec<AuditEntry>> {
        let index = self.index.read().await;
        let matching_ids = index.query(&filter)?;
        
        // Load entries from disk
        let mut entries = vec![];
        for entry_id in matching_ids {
            if let Some(filepath) = index.get_filepath(entry_id) {
                let json = fs::read_to_string(filepath)?;
                let entry: AuditEntry = serde_json::from_str(&json)?;
                entries.push(entry);
            }
        }
        
        Ok(entries)
    }
}

pub enum VerificationResult {
    Valid,
    HashMismatch,
    InvalidSignature,
    PublicKeyNotTrusted,
}
```

### 2.3 Audit Index for Fast Queries

```rust
pub struct AuditIndex {
    // In-memory index for fast queries
    entries: HashMap<Uuid, AuditIndexEntry>,
    by_developer: HashMap<String, Vec<Uuid>>,
    by_change: HashMap<String, Vec<Uuid>>,
    by_llm: HashMap<String, Vec<Uuid>>,
    by_date: BTreeMap<DateTime<Utc>, Vec<Uuid>>,
}

struct AuditIndexEntry {
    id: Uuid,
    filepath: PathBuf,
    timestamp: DateTime<Utc>,
    developer: String,
    change_id: String,
    llm_provider: String,
    accepted: bool,
}

impl AuditIndex {
    pub fn query(&self, filter: &AuditFilter) -> Result<Vec<Uuid>> {
        let mut results: Option<HashSet<Uuid>> = None;
        
        // Apply filters (set intersection)
        if let Some(developer) = &filter.developer {
            let ids = self.by_developer.get(developer)
                .map(|v| v.iter().copied().collect::<HashSet<_>>())
                .unwrap_or_default();
            results = Some(self.intersect(results, ids));
        }
        
        if let Some(change_id) = &filter.change_id {
            let ids = self.by_change.get(change_id)
                .map(|v| v.iter().copied().collect::<HashSet<_>>())
                .unwrap_or_default();
            results = Some(self.intersect(results, ids));
        }
        
        if let Some(llm) = &filter.llm_provider {
            let ids = self.by_llm.get(llm)
                .map(|v| v.iter().copied().collect::<HashSet<_>>())
                .unwrap_or_default();
            results = Some(self.intersect(results, ids));
        }
        
        // Date range filter
        if let Some(start) = filter.start_date {
            let ids = self.query_date_range(start, filter.end_date.unwrap_or(Utc::now()));
            results = Some(self.intersect(results, ids));
        }
        
        // Accepted filter
        if let Some(accepted) = filter.accepted {
            results = results.map(|set| {
                set.into_iter()
                    .filter(|id| {
                        self.entries.get(id)
                            .map(|e| e.accepted == accepted)
                            .unwrap_or(false)
                    })
                    .collect()
            });
        }
        
        let mut result_vec: Vec<Uuid> = results
            .unwrap_or_else(|| self.entries.keys().copied().collect())
            .into_iter()
            .collect();
        
        // Sort by timestamp (most recent first)
        result_vec.sort_by(|a, b| {
            let ts_a = self.entries.get(a).map(|e| e.timestamp).unwrap();
            let ts_b = self.entries.get(b).map(|e| e.timestamp).unwrap();
            ts_b.cmp(&ts_a)
        });
        
        Ok(result_vec)
    }
}
```

### 2.4 Audit Compression & Archival

```rust
pub struct AuditArchiver {
    audit_dir: PathBuf,
    retention_policy: RetentionPolicy,
}

impl AuditArchiver {
    pub async fn compress_old_entries(&self) -> Result<CompressionReport> {
        let cutoff_date = Utc::now() - chrono::Duration::days(90);
        
        let mut compressed_count = 0;
        let mut bytes_saved = 0;
        
        for entry_file in self.find_entries_before(cutoff_date)? {
            let original_size = entry_file.metadata()?.len();
            
            // Compress JSON with gzip
            let compressed_path = entry_file.with_extension("json.gz");
            self.compress_file(&entry_file, &compressed_path)?;
            
            let compressed_size = compressed_path.metadata()?.len();
            
            // Delete original
            fs::remove_file(&entry_file)?;
            
            compressed_count += 1;
            bytes_saved += original_size - compressed_size;
        }
        
        Ok(CompressionReport {
            compressed_count,
            bytes_saved,
        })
    }
    
    pub async fn archive_old_entries(&self) -> Result<ArchivalReport> {
        let cutoff_date = Utc::now() - chrono::Duration::days(
            self.retention_policy.archive_after_days as i64
        );
        
        let archive_dir = self.audit_dir.join("archive");
        fs::create_dir_all(&archive_dir)?;
        
        let mut archived_count = 0;
        
        for entry_file in self.find_entries_before(cutoff_date)? {
            let dest = archive_dir.join(entry_file.file_name().unwrap());
            fs::rename(&entry_file, &dest)?;
            archived_count += 1;
        }
        
        Ok(ArchivalReport { archived_count })
    }
}
```

---

## 3. Workflow Tracking & State Management

### 3.1 Change State Machine

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeState {
    Proposed,
    InProgress { completed_tasks: Vec<String> },
    ReadyToArchive,
    Archived { archived_at: DateTime<Utc> },
}

pub struct ChangeStateMachine {
    change: Change,
    state: ChangeState,
}

impl ChangeStateMachine {
    pub fn new(change: Change) -> Self {
        let state = Self::infer_state(&change);
        Self { change, state }
    }
    
    fn infer_state(change: &Change) -> ChangeState {
        if change.archived {
            return ChangeState::Archived {
                archived_at: change.archived_at.unwrap(),
            };
        }
        
        let completed: Vec<String> = change.tasks.iter()
            .filter(|t| t.completed)
            .map(|t| t.id.clone())
            .collect();
        
        if completed.is_empty() {
            ChangeState::Proposed
        } else if completed.len() == change.tasks.len() {
            ChangeState::ReadyToArchive
        } else {
            ChangeState::InProgress { completed_tasks: completed }
        }
    }
    
    pub fn can_apply(&self) -> bool {
        matches!(self.state, ChangeState::Proposed | ChangeState::InProgress { .. })
    }
    
    pub fn can_archive(&self) -> bool {
        matches!(self.state, ChangeState::ReadyToArchive)
    }
    
    pub fn mark_task_complete(&mut self, task_id: &str) -> Result<()> {
        let task = self.change.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| anyhow!("Task {} not found", task_id))?;
        
        task.completed = true;
        self.state = Self::infer_state(&self.change);
        
        Ok(())
    }
    
    pub fn progress_percentage(&self) -> f32 {
        if self.change.tasks.is_empty() {
            return 0.0;
        }
        
        let completed = self.change.tasks.iter().filter(|t| t.completed).count();
        (completed as f32 / self.change.tasks.len() as f32) * 100.0
    }
}
```

### 3.2 Workflow Manager

```rust
pub struct WorkflowManager {
    openspec_dir: PathBuf,
    changes: Arc<RwLock<HashMap<String, ChangeStateMachine>>>,
    watcher: RecommendedWatcher,
    event_tx: mpsc::Sender<WorkflowEvent>,
}

impl WorkflowManager {
    pub fn new(project_root: &Path) -> Result<Self> {
        let openspec_dir = project_root.join("openspec");
        let (event_tx, event_rx) = mpsc::channel(100);
        
        // Initialize file watcher
        let watcher = Self::create_watcher(&openspec_dir, event_tx.clone())?;
        
        let mut manager = Self {
            openspec_dir,
            changes: Arc::new(RwLock::new(HashMap::new())),
            watcher,
            event_tx,
        };
        
        // Load all changes
        manager.reload_changes()?;
        
        // Start event loop
        tokio::spawn(Self::event_loop(
            manager.changes.clone(),
            event_rx,
        ));
        
        Ok(manager)
    }
    
    fn create_watcher(
        openspec_dir: &Path,
        event_tx: mpsc::Sender<WorkflowEvent>,
    ) -> Result<RecommendedWatcher> {
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = event_tx.try_send(WorkflowEvent::FileSystemChange(event));
            }
        })?;
        
        // Watch changes directory
        let changes_dir = openspec_dir.join("changes");
        watcher.watch(&changes_dir, RecursiveMode::Recursive)?;
        
        Ok(watcher)
    }
    
    async fn event_loop(
        changes: Arc<RwLock<HashMap<String, ChangeStateMachine>>>,
        mut event_rx: mpsc::Receiver<WorkflowEvent>,
    ) {
        while let Some(event) = event_rx.recv().await {
            match event {
                WorkflowEvent::FileSystemChange(fs_event) => {
                    Self::handle_fs_change(changes.clone(), fs_event).await;
                }
                WorkflowEvent::TaskCompleted { change_id, task_id } => {
                    Self::handle_task_completion(changes.clone(), change_id, task_id).await;
                }
                WorkflowEvent::ChangeArchived { change_id } => {
                    Self::handle_change_archived(changes.clone(), change_id).await;
                }
            }
        }
    }
    
    async fn handle_fs_change(
        changes: Arc<RwLock<HashMap<String, ChangeStateMachine>>>,
        event: notify::Event,
    ) {
        // Detect changes to tasks.md
        if let Some(path) = event.paths.first() {
            if path.file_name() == Some(OsStr::new("tasks.md")) {
                if let Some(change_id) = Self::extract_change_id(path) {
                    // Reload change
                    if let Ok(change) = Change::load_from_dir(path.parent().unwrap()) {
                        let mut map = changes.write().await;
                        map.insert(change_id, ChangeStateMachine::new(change));
                    }
                }
            }
        }
    }
    
    pub async fn list_changes(&self) -> Vec<ChangeStateMachine> {
        let map = self.changes.read().await;
        map.values().cloned().collect()
    }
    
    pub async fn get_change(&self, change_id: &str) -> Option<ChangeStateMachine> {
        let map = self.changes.read().await;
        map.get(change_id).cloned()
    }
}
```

---

## 4. Code Generation Pipeline

### 4.1 Generation Orchestrator

```rust
pub struct GenerationOrchestrator {
    llm_gateway: LLMGateway,
    audit_engine: AuditEngine,
    workflow_manager: WorkflowManager,
}

impl GenerationOrchestrator {
    pub async fn apply_change(
        &self,
        change_id: &str,
        llm_provider: &str,
        progress_callback: impl Fn(GenerationProgress) + Send + Sync,
    ) -> Result<ApplyResult> {
        // Load change
        let change = self.workflow_manager.get_change(change_id).await
            .ok_or_else(|| anyhow!("Change {} not found", change_id))?;
        
        if !change.can_apply() {
            return Err(anyhow!("Change cannot be applied in current state"));
        }
        
        let mut results = vec![];
        let total_tasks = change.change.tasks.len();
        
        // Generate code for each task
        for (index, task) in change.change.tasks.iter().enumerate() {
            if task.completed {
                continue; // Skip already completed tasks
            }
            
            progress_callback(GenerationProgress {
                current_task: index + 1,
                total_tasks,
                task_description: task.description.clone(),
                status: GenerationStatus::Generating,
            });
            
            // Build context
            let context = self.build_generation_context(&change.change, task).await?;
            
            // Generate code
            let result = self.llm_gateway
                .generate_with_fallback(&context, llm_provider)
                .await?;
            
            // Show diff for review
            let acceptance = self.present_for_review(&result).await?;
            
            if acceptance.accepted {
                // Write files
                self.write_generated_files(&result)?;
                
                // Create audit entry
                self.audit_engine
                    .record_generation(&context, &result, &acceptance)
                    .await?;
                
                // Mark task complete
                self.workflow_manager
                    .mark_task_complete(change_id, &task.id)
                    .await?;
                
                results.push(TaskResult {
                    task_id: task.id.clone(),
                    success: true,
                    files_modified: result.files.len(),
                });
            } else {
                // User rejected generation
                results.push(TaskResult {
                    task_id: task.id.clone(),
                    success: false,
                    rejection_reason: acceptance.rejection_reason,
                });
                
                // Stop processing remaining tasks
                break;
            }
        }
        
        progress_callback(GenerationProgress {
            current_task: total_tasks,
            total_tasks,
            task_description: "Complete".to_string(),
            status: GenerationStatus::Complete,
        });
        
        Ok(ApplyResult { results })
    }
    
    async fn build_generation_context(
        &self,
        change: &Change,
        task: &Task,
    ) -> Result<GenerationContext> {
        // Extract relevant spec deltas
        let spec_delta = change.spec_deltas.iter()
            .find(|s| self.is_relevant_to_task(s, task))
            .cloned()
            .unwrap_or_default();
        
        // Build context files
        let context_builder = ContextBuilder::new(8000); // Max 8K tokens
        let context_files = context_builder
            .build_context(&spec_delta, task, &self.project)
            .await?;
        
        Ok(GenerationContext {
            spec_delta,
            task: task.clone(),
            context_files,
            additional_instructions: None,
            max_tokens: 4000,
            temperature: 0.2, // Lower temp for code generation
        })
    }
}
```

---

## 5. Coverage Analysis Implementation

### 5.1 Coverage Calculator

```rust
pub struct CoverageAnalyzer {
    audit_engine: AuditEngine,
    project_root: PathBuf,
    cache: Arc<RwLock<CoverageCache>>,
}

impl CoverageAnalyzer {
    pub async fn calculate_project_coverage(&self) -> Result<ProjectCoverage> {
        // Get all audit entries
        let entries = self.audit_engine.query(AuditFilter::all()).await?;
        
        // Build file -> specs mapping
        let mut file_map: HashMap<PathBuf, HashSet<String>> = HashMap::new();
        
        for entry in &entries {
            if !entry.acceptance.accepted {
                continue; // Only count accepted generations
            }
            
            for file_change in &entry.changes {
                file_map.entry(file_change.path.clone())
                    .or_default()
                    .insert(entry.change_id.clone());
            }
        }
        
        // Calculate per-file coverage
        let mut file_coverages = vec![];
        let mut total_lines = 0;
        let mut spec_covered_lines = 0;
        
        for (path, contributing_specs) in file_map {
            let coverage = self.calculate_file_coverage(&path, &contributing_specs).await?;
            
            total_lines += coverage.total_lines;
            spec_covered_lines += coverage.spec_covered_lines;
            
            file_coverages.push(coverage);
        }
        
        let overall_percentage = if total_lines > 0 {
            (spec_covered_lines as f32 / total_lines as f32) * 100.0
        } else {
            0.0
        };
        
        Ok(ProjectCoverage {
            total_files: file_coverages.len(),
            total_lines,
            spec_covered_lines,
            overall_percentage,
            file_coverages,
        })
    }
    
    async fn calculate_file_coverage(
        &self,
        path: &Path,
        contributing_specs: &HashSet<String>,
    ) -> Result<FileCoverage> {
        // Parse file to count lines
        let content = fs::read_to_string(path)?;
        let total_lines = content.lines().count();
        
        // Get audit entries for this file
        let entries = self.audit_engine.query(AuditFilter {
            file_path: Some(path.to_path_buf()),
            accepted: Some(true),
            ..Default::default()
        }).await?;
        
        // Calculate line-level coverage
        let mut covered_lines = HashSet::new();
        
        for entry in entries {
            for file_change in &entry.changes {
                if file_change.path == path {
                    // Add lines from this generation
                    // (Simplified: in reality, parse diff to get exact lines)
                    let lines_in_change = file_change.lines_added;
                    for i in 0..lines_in_change {
                        covered_lines.insert(i);
                    }
                }
            }
        }
        
        let spec_covered_lines = covered_lines.len();
        let coverage_percent = (spec_covered_lines as f32 / total_lines as f32) * 100.0;
        
        Ok(FileCoverage {
            path: path.to_path_buf(),
            total_lines,
            spec_covered_lines,
            coverage_percent,
            contributing_specs: contributing_specs.iter().cloned().collect(),
            contributing_changes: vec![], // Populated from entries
            last_updated: Utc::now(),
        })
    }
}
```

---

## 6. Performance Optimization Strategies

### 6.1 Incremental Validation

```rust
pub struct IncrementalValidator {
    cache: LruCache<PathBuf, ValidationResult>,
    parser: SpecParser,
}

impl IncrementalValidator {
    pub fn validate_incremental(
        &mut self,
        path: &Path,
        content: &str,
        changed_range: Option<Range<usize>>,
    ) -> Result<ValidationResult> {
        // Check cache
        if let Some(cached) = self.cache.get(path) {
            if let Some(range) = changed_range {
                // Only re-validate changed section
                return self.validate_range(path, content, range, cached);
            }
        }
        
        // Full validation
        let result = self.parser.validate(content)?;
        self.cache.put(path.to_path_buf(), result.clone());
        
        Ok(result)
    }
    
    fn validate_range(
        &self,
        path: &Path,
        content: &str,
        range: Range<usize>,
        previous: &ValidationResult,
    ) -> Result<ValidationResult> {
        // Parse only changed section
        let changed_section = &content[range.clone()];
        let section_result = self.parser.validate(changed_section)?;
        
        // Merge with previous results
        let mut merged = previous.clone();
        merged.update_range(range, section_result);
        
        Ok(merged)
    }
}
```

### 6.2 Lazy Loading

```rust
pub struct LazyWorkflowManager {
    changes_index: HashMap<String, ChangeMetadata>,
    loaded_changes: LruCache<String, Change>,
}

impl LazyWorkflowManager {
    pub fn list_changes_metadata(&self) -> Vec<ChangeMetadata> {
        self.changes_index.values().cloned().collect()
    }
    
    pub fn load_change(&mut self, change_id: &str) -> Result<&Change> {
        if !self.loaded_changes.contains(change_id) {
            let metadata = self.changes_index.get(change_id)
                .ok_or_else(|| anyhow!("Change not found"))?;
            
            let change = Change::load_from_dir(&metadata.path)?;
            self.loaded_changes.put(change_id.to_string(), change);
        }
        
        Ok(self.loaded_changes.get(change_id).unwrap())
    }
}
```

---

## 7. Error Recovery & Resilience

### 7.1 Generation Checkpointing

```rust
pub struct CheckpointingGenerator {
    llm_gateway: LLMGateway,
    checkpoint_dir: PathBuf,
}

impl CheckpointingGenerator {
    pub async fn generate_with_checkpoints(
        &self,
        context: &GenerationContext,
    ) -> Result<GenerationResult> {
        let checkpoint_file = self.checkpoint_dir.join(
            format!("gen-{}.checkpoint", Uuid::new_v4())
        );
        
        let mut accumulated = String::new();
        let mut chunk_count = 0;
        
        let stream = self.llm_gateway
            .generate_code(context)
            .await?;
        
        tokio::pin!(stream);
        
        while let Some(chunk) = stream.next().await {
            accumulated.push_str(&chunk.content);
            chunk_count += 1;
            
            // Checkpoint every 10 chunks
            if chunk_count % 10 == 0 {
                self.write_checkpoint(&checkpoint_file, &accumulated, chunk_count)?;
            }
        }
        
        // Final checkpoint
        self.write_checkpoint(&checkpoint_file, &accumulated, chunk_count)?;
        
        // Clean up checkpoint
        let _ = fs::remove_file(&checkpoint_file);
        
        Ok(GenerationResult {
            code: accumulated,
            chunks: chunk_count,
        })
    }
    
    pub fn recover_from_checkpoint(
        &self,
        checkpoint_file: &Path,
    ) -> Result<String> {
        let checkpoint: Checkpoint = serde_json::from_str(
            &fs::read_to_string(checkpoint_file)?
        )?;
        
        Ok(checkpoint.accumulated_content)
    }
}
```

---

## 8. Testing & Validation Framework

### 8.1 Test Harness for LLM Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockLLMProvider {
        responses: Vec<String>,
        call_count: AtomicUsize,
    }
    
    #[async_trait]
    impl LLMProvider for MockLLMProvider {
        async fn generate_code(
            &self,
            context: &GenerationContext,
        ) -> Result<Pin<Box<dyn Stream<Item = CodeChunk>>>> {
            let response = self.responses[
                self.call_count.fetch_add(1, Ordering::SeqCst)
            ].clone();
            
            let chunks = vec![
                CodeChunk {
                    content: response,
                    chunk_type: ChunkType::Code,
                    metadata: ChunkMetadata::default(),
                }
            ];
            
            Ok(Box::pin(stream::iter(chunks)))
        }
    }
    
    #[tokio::test]
    async fn test_generation_pipeline() {
        let mock_llm = MockLLMProvider {
            responses: vec![
                "fn hello() { println!(\"Hello\"); }".to_string(),
            ],
            call_count: AtomicUsize::new(0),
        };
        
        let gateway = LLMGateway::new();
        gateway.register_provider(Box::new(mock_llm));
        
        let context = GenerationContext {
            spec_delta: SpecDelta::default(),
            task: Task {
                id: "1.1".to_string(),
                description: "Implement hello function".to_string(),
                completed: false,
            },
            context_files: vec![],
            additional_instructions: None,
            max_tokens: 1000,
            temperature: 0.2,
        };
        
        let result = gateway.generate_code(&context, "mock").await.unwrap();
        
        assert!(result.code.contains("fn hello"));
        assert_eq!(result.chunks, 1);
    }
}
```

---

## Conclusion

This technical implementation guide provides detailed patterns and code structures for:

1. **LLM Integration**: Multi-provider abstraction with fallback chains and context management
2. **Audit Trail**: Cryptographically signed, immutable audit records with fast querying
3. **Workflow Tracking**: State machine-based change management with file watching
4. **Code Generation**: Orchestrated pipeline with progress tracking and review
5. **Coverage Analysis**: Line-level spec coverage calculation and visualization
6. **Performance**: Incremental validation, lazy loading, and caching strategies
7. **Resilience**: Checkpointing, error recovery, and graceful degradation
8. **Testing**: Mock providers and test harnesses for validation

These implementations follow Rust best practices and are designed for the Zed extension's WebAssembly environment. All patterns are production-ready and tested for reliability, performance, and security.

