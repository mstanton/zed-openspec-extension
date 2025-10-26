use zed_extension_api as zed;

mod commands;
mod utils;

use commands::CommandHandler;
use utils::config::ExtensionConfig;

/// Main extension struct
pub struct OpenSpecExtension {
    command_handler: CommandHandler,
    config: ExtensionConfig,
}

impl zed::Extension for OpenSpecExtension {
    fn new() -> Self {
        // Initialize extension
        eprintln!("[OpenSpec] Extension initializing...");

        let config = ExtensionConfig::load_or_default();
        let command_handler = CommandHandler::new(config.clone());

        eprintln!("[OpenSpec] Extension initialized successfully");

        Self {
            command_handler,
            config,
        }
    }

    fn command(
        &mut self,
        command: String,
        args: Vec<String>,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        eprintln!("[OpenSpec] Command received: {} with args: {:?}", command, args);

        // Handle command
        match self.command_handler.handle_command(&command, args, worktree) {
            Ok(result) => {
                eprintln!("[OpenSpec] Command succeeded: {}", command);
                Ok(result)
            }
            Err(e) => {
                eprintln!("[OpenSpec] Command failed: {} - Error: {}", command, e);
                Err(format!("OpenSpec Error: {}", e))
            }
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        eprintln!("[OpenSpec] Language server command requested for: {:?}", language_server_id);

        // LSP server will be implemented in Phase 2
        Err("LSP server not yet implemented".to_string())
    }
}

zed::register_extension!(OpenSpecExtension);
