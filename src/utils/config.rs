use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extension configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub llm: LLMConfig,
    pub validation: ValidationConfig,
    pub audit: AuditConfig,
    pub coverage: CoverageConfig,
    pub workflow: WorkflowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub default_provider: String,
    pub providers: HashMap<String, ProviderConfig>,
    pub fallback_chain: Vec<String>,
    pub generation_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub model: String,
    pub api_key_env: Option<String>,
    pub max_tokens: usize,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub debounce_ms: u64,
    pub rules: ValidationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub require_scenarios: bool,
    pub require_shall_must: bool,
    pub max_spec_size_kb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub signature_required: bool,
    pub export_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageConfig {
    pub exclude_patterns: Vec<String>,
    pub minimum_coverage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub auto_archive_on_complete: bool,
    pub require_all_tasks_complete: bool,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        let mut providers = HashMap::new();

        providers.insert(
            "claude".to_string(),
            ProviderConfig {
                model: "claude-sonnet-4-20250514".to_string(),
                api_key_env: Some("ANTHROPIC_API_KEY".to_string()),
                max_tokens: 8000,
                endpoint: None,
            },
        );

        providers.insert(
            "gpt-4".to_string(),
            ProviderConfig {
                model: "gpt-4-turbo".to_string(),
                api_key_env: Some("OPENAI_API_KEY".to_string()),
                max_tokens: 8000,
                endpoint: None,
            },
        );

        providers.insert(
            "ollama".to_string(),
            ProviderConfig {
                model: "codellama".to_string(),
                api_key_env: None,
                max_tokens: 4000,
                endpoint: Some("http://localhost:11434".to_string()),
            },
        );

        Self {
            llm: LLMConfig {
                default_provider: "claude".to_string(),
                providers,
                fallback_chain: vec!["claude".to_string(), "gpt-4".to_string()],
                generation_timeout_seconds: 120,
            },
            validation: ValidationConfig {
                enabled: true,
                debounce_ms: 500,
                rules: ValidationRules {
                    require_scenarios: true,
                    require_shall_must: true,
                    max_spec_size_kb: 1024,
                },
            },
            audit: AuditConfig {
                enabled: true,
                retention_days: 730,
                signature_required: true,
                export_format: "json".to_string(),
            },
            coverage: CoverageConfig {
                exclude_patterns: vec![
                    "**/test/**".to_string(),
                    "**/*.test.*".to_string(),
                    "**/node_modules/**".to_string(),
                ],
                minimum_coverage_percent: 60.0,
            },
            workflow: WorkflowConfig {
                auto_archive_on_complete: false,
                require_all_tasks_complete: true,
            },
        }
    }
}

impl ExtensionConfig {
    /// Load configuration from file or return default
    pub fn load_or_default() -> Self {
        // For Phase 1, just return defaults
        // In future phases, read from .openspec-config.json
        Self::default()
    }

    /// Save configuration to file
    pub fn save(&self, _path: &std::path::Path) -> anyhow::Result<()> {
        // Placeholder for future implementation
        Ok(())
    }
}
