use anyhow::{Result, Context};
use std::path::Path;

use crate::utils::config::ExtensionConfig;

/// Handle `openspec:apply-change` command
/// Generates code for a change using LLM
pub fn handle_apply_change(
    workspace_path: &Path,
    change_id: &str,
    llm_provider: &str,
    config: &ExtensionConfig,
) -> Result<String> {
    eprintln!("[OpenSpec] Applying change: {} with provider: {}", change_id, llm_provider);

    // Verify change exists
    let change_dir = workspace_path.join("openspec").join("changes").join(change_id);
    if !change_dir.exists() {
        return Err(anyhow::anyhow!(
            "Change '{}' not found. Available changes can be viewed with 'openspec:list-changes'",
            change_id
        ));
    }

    // This is a placeholder for Phase 1
    // Full LLM integration will be implemented in Phase 3
    Ok(format!(
        "Code generation for change '{}' is not yet implemented.\n\n\
        This feature will be available in Phase 3 of development.\n\n\
        For now, you can:\n\
        1. Manually implement the tasks in openspec/changes/{}/tasks.md\n\
        2. Mark tasks as complete in tasks.md\n\
        3. Run 'openspec:archive-change' when all tasks are done\n\n\
        LLM Provider configured: {}\n\
        Configuration: {:?}",
        change_id,
        change_id,
        llm_provider,
        config.llm
    ))
}
