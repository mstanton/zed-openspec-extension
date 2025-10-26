use anyhow::{Result, Context};
use std::path::Path;
use std::fs;

/// Handle `openspec:validate-file` command
/// Validates a spec file
pub fn handle_validate_file(workspace_path: &Path, file_path: &str) -> Result<String> {
    eprintln!("[OpenSpec] Validating file: {}", file_path);

    let full_path = workspace_path.join(file_path);

    if !full_path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", file_path));
    }

    // Read file content
    let content = fs::read_to_string(&full_path)
        .context("Failed to read file")?;

    // Basic validation (placeholder for Phase 2 LSP validation)
    let mut warnings = Vec::new();

    // Check if file is in openspec directory
    if !file_path.starts_with("openspec/") {
        warnings.push("File is not in openspec/ directory".to_string());
    }

    // Check for basic spec structure
    if !content.contains("### Requirement:") && !content.contains("## ADDED") {
        warnings.push("No requirements or spec deltas found".to_string());
    }

    // Count requirements
    let requirement_count = content.matches("### Requirement:").count();
    let scenario_count = content.matches("#### Scenario:").count();

    if requirement_count > 0 && scenario_count == 0 {
        warnings.push("Requirements found but no scenarios defined".to_string());
    }

    if warnings.is_empty() {
        Ok(format!(
            "✓ Validation passed for: {}\n\n\
            Found:\n\
            - {} requirement(s)\n\
            - {} scenario(s)\n\n\
            Note: Full LSP-based validation will be available in Phase 2",
            file_path, requirement_count, scenario_count
        ))
    } else {
        Ok(format!(
            "⚠ Validation warnings for: {}\n\n{}\n\n\
            Found:\n\
            - {} requirement(s)\n\
            - {} scenario(s)\n\n\
            Note: Full LSP-based validation will be available in Phase 2",
            file_path,
            warnings.iter()
                .enumerate()
                .map(|(i, w)| format!("{}. {}", i + 1, w))
                .collect::<Vec<_>>()
                .join("\n"),
            requirement_count,
            scenario_count
        ))
    }
}
