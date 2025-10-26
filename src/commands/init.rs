use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;

/// Handle `openspec:init` command
/// Initializes OpenSpec in the current workspace
pub fn handle_init(workspace_path: &Path) -> Result<String> {
    eprintln!("[OpenSpec] Initializing OpenSpec in: {:?}", workspace_path);

    // Check if OpenSpec CLI is installed
    let version_check = Command::new("openspec")
        .arg("--version")
        .output();

    match version_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            eprintln!("[OpenSpec] Found OpenSpec CLI: {}", version.trim());
        }
        _ => {
            return Err(anyhow::anyhow!(
                "OpenSpec CLI not found. Install it with:\n\
                npm install -g @fission-ai/openspec@latest"
            ));
        }
    }

    // Run openspec init
    let output = Command::new("openspec")
        .arg("init")
        .current_dir(workspace_path)
        .output()
        .context("Failed to execute openspec init")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!(
            "OpenSpec initialized successfully!\n\n{}",
            stdout
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "OpenSpec init failed:\n{}",
            stderr
        ))
    }
}
