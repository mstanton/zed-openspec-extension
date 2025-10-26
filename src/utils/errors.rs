use std::fmt;

/// Custom error types for the extension
#[derive(Debug)]
pub enum OpenSpecError {
    /// OpenSpec CLI not found or not installed
    CLINotFound,

    /// OpenSpec not initialized in workspace
    NotInitialized,

    /// Change not found
    ChangeNotFound(String),

    /// Invalid proposal name
    InvalidProposalName(String),

    /// LLM provider error
    LLMError(String),

    /// Validation error
    ValidationError(String),

    /// Audit error
    AuditError(String),

    /// IO error
    IOError(std::io::Error),

    /// Generic error
    Other(String),
}

impl fmt::Display for OpenSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CLINotFound => write!(
                f,
                "OpenSpec CLI not found. Install with: npm install -g @fission-ai/openspec@latest"
            ),
            Self::NotInitialized => write!(
                f,
                "OpenSpec not initialized. Run 'openspec:init' first"
            ),
            Self::ChangeNotFound(id) => write!(
                f,
                "Change '{}' not found",
                id
            ),
            Self::InvalidProposalName(name) => write!(
                f,
                "Invalid proposal name '{}'. Use alphanumeric, hyphens, underscores only (max 64 chars)",
                name
            ),
            Self::LLMError(msg) => write!(f, "LLM error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::AuditError(msg) => write!(f, "Audit error: {}", msg),
            Self::IOError(e) => write!(f, "IO error: {}", e),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for OpenSpecError {}

impl From<std::io::Error> for OpenSpecError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
