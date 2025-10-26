use anyhow::{Result, Context};
use std::path::Path;
use std::fs;

/// Handle `openspec:list-changes` command
/// Lists all OpenSpec changes
pub fn handle_list_changes(workspace_path: &Path) -> Result<String> {
    eprintln!("[OpenSpec] Listing changes");

    let changes_dir = workspace_path.join("openspec").join("changes");

    if !changes_dir.exists() {
        return Ok("No changes found. OpenSpec may not be initialized or no proposals created yet.".to_string());
    }

    let mut changes = Vec::new();

    for entry in fs::read_dir(&changes_dir).context("Failed to read changes directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Check if proposal.md exists to confirm it's a valid change
                let proposal_file = path.join("proposal.md");
                if proposal_file.exists() {
                    changes.push(name.to_string());
                }
            }
        }
    }

    if changes.is_empty() {
        Ok("No changes found. Create one with 'openspec:new-proposal'".to_string())
    } else {
        changes.sort();
        Ok(format!(
            "OpenSpec Changes ({}):\n\n{}\n\nUse 'openspec:apply-change <name>' to generate code for a change.",
            changes.len(),
            changes.iter()
                .enumerate()
                .map(|(i, name)| format!("{}. {}", i + 1, name))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }
}
