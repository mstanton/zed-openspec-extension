use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;

/// Handle `openspec:archive-change` command
/// Archives a completed change
pub fn handle_archive_change(workspace_path: &Path, change_id: &str) -> Result<String> {
    eprintln!("[OpenSpec] Archiving change: {}", change_id);

    // Verify change exists
    let change_dir = workspace_path.join("openspec").join("changes").join(change_id);
    if !change_dir.exists() {
        return Err(anyhow::anyhow!(
            "Change '{}' not found",
            change_id
        ));
    }

    // TODO: Verify all tasks are complete before archiving
    // This will be implemented when workflow tracking is added

    // Run openspec archive command
    let output = Command::new("openspec")
        .arg("archive")
        .arg(change_id)
        .arg("--yes")
        .current_dir(workspace_path)
        .output()
        .context("Failed to execute openspec archive")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!(
            "Change '{}' archived successfully!\n\n{}",
            change_id, stdout
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "Failed to archive change:\n{}",
            stderr
        ))
    }
}
