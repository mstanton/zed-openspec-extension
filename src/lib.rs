use zed_extension_api as zed;

/// Main extension struct for OpenSpec integration
///
/// Phase 1 Note: Zed extensions currently support language servers and grammar.
/// Command palette integration requires slash commands (/) which will be added
/// in a future Zed update. For now, this extension provides the foundation
/// for OpenSpec integration that will be activated when command support is added.
pub struct OpenSpecExtension;

impl zed::Extension for OpenSpecExtension {
    /// Initialize the extension
    fn new() -> Self {
        eprintln!("[OpenSpec] Extension initialized");
        eprintln!("[OpenSpec] Note: Full command support coming in Phase 2");
        eprintln!("[OpenSpec] Current: Foundation and LSP preparation");
        Self
    }

    /// Provide language server configuration
    ///
    /// This will be used in Phase 2 to provide real-time spec validation
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // Phase 2 will implement LSP server for spec validation
        Err("OpenSpec LSP server not yet implemented - Coming in Phase 2".to_string())
    }
}

zed::register_extension!(OpenSpecExtension);
