pub mod init;
pub mod proposal;
pub mod apply;
pub mod archive;
pub mod list;
pub mod audit;
pub mod validate;
pub mod coverage;

use zed_extension_api as zed;
use anyhow::Result;
use std::path::PathBuf;

use crate::utils::config::ExtensionConfig;

/// Command handler for all OpenSpec operations
pub struct CommandHandler {
    config: ExtensionConfig,
}

impl CommandHandler {
    pub fn new(config: ExtensionConfig) -> Self {
        Self { config }
    }

    /// Handle incoming commands from Zed
    pub fn handle_command(
        &self,
        command: &str,
        args: Vec<String>,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        let workspace_path = PathBuf::from(worktree.root_path());

        match command {
            "openspec:init" => {
                init::handle_init(&workspace_path)
                    .map_err(|e| e.to_string())
            }

            "openspec:new-proposal" => {
                let name = args.get(0)
                    .ok_or("Proposal name required")?
                    .clone();
                proposal::handle_new_proposal(&workspace_path, &name)
                    .map_err(|e| e.to_string())
            }

            "openspec:apply-change" => {
                let change_id = args.get(0)
                    .ok_or("Change ID required")?
                    .clone();
                let llm_provider = args.get(1)
                    .map(|s| s.as_str())
                    .unwrap_or(&self.config.llm.default_provider);

                apply::handle_apply_change(&workspace_path, &change_id, llm_provider, &self.config)
                    .map_err(|e| e.to_string())
            }

            "openspec:archive-change" => {
                let change_id = args.get(0)
                    .ok_or("Change ID required")?
                    .clone();
                archive::handle_archive_change(&workspace_path, &change_id)
                    .map_err(|e| e.to_string())
            }

            "openspec:list-changes" => {
                list::handle_list_changes(&workspace_path)
                    .map_err(|e| e.to_string())
            }

            "openspec:view-audit" => {
                let filter = args.get(0).map(|s| s.as_str());
                audit::handle_view_audit(&workspace_path, filter)
                    .map_err(|e| e.to_string())
            }

            "openspec:validate-file" => {
                let file_path = args.get(0)
                    .ok_or("File path required")?
                    .clone();
                validate::handle_validate_file(&workspace_path, &file_path)
                    .map_err(|e| e.to_string())
            }

            "openspec:show-coverage" => {
                coverage::handle_show_coverage(&workspace_path, &self.config)
                    .map_err(|e| e.to_string())
            }

            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}
