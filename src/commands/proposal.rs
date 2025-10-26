use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;

/// Handle `openspec:new-proposal` command
/// Creates a new OpenSpec change proposal
pub fn handle_new_proposal(workspace_path: &Path, name: &str) -> Result<String> {
    eprintln!("[OpenSpec] Creating new proposal: {}", name);

    // Validate proposal name
    if !is_valid_proposal_name(name) {
        return Err(anyhow::anyhow!(
            "Invalid proposal name. Use only alphanumeric characters, hyphens, and underscores. Max 64 characters."
        ));
    }

    // Check if OpenSpec is initialized
    let openspec_dir = workspace_path.join("openspec");
    if !openspec_dir.exists() {
        return Err(anyhow::anyhow!(
            "OpenSpec not initialized. Run 'openspec:init' first."
        ));
    }

    // Run openspec proposal command
    let output = Command::new("openspec")
        .arg("proposal")
        .arg(name)
        .current_dir(workspace_path)
        .output()
        .context("Failed to execute openspec proposal")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!(
            "Proposal '{}' created successfully!\n\n{}\n\nNext steps:\n\
            1. Edit openspec/changes/{}/proposal.md\n\
            2. Add specs to openspec/changes/{}/specs/\n\
            3. Define tasks in openspec/changes/{}/tasks.md\n\
            4. Run 'openspec:apply-change' to generate code",
            name, stdout, name, name, name
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "Failed to create proposal:\n{}",
            stderr
        ))
    }
}

fn is_valid_proposal_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }

    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_proposal_names() {
        assert!(is_valid_proposal_name("add-authentication"));
        assert!(is_valid_proposal_name("feature_123"));
        assert!(is_valid_proposal_name("fix-bug-42"));
    }

    #[test]
    fn test_invalid_proposal_names() {
        assert!(!is_valid_proposal_name(""));
        assert!(!is_valid_proposal_name("with spaces"));
        assert!(!is_valid_proposal_name("with/slash"));
        assert!(!is_valid_proposal_name(&"a".repeat(65)));
    }
}
