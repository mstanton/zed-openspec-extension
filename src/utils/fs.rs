use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};

/// File system utility functions

/// Check if a directory exists
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Read file content as string
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))
}

/// Write content to file
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {:?}", path))
}

/// Create directory and all parent directories
pub fn create_dir_all(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {:?}", path))
}

/// List all subdirectories in a directory
pub fn list_subdirectories(path: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

/// List all files in a directory (non-recursive)
pub fn list_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

/// Get file size in bytes
pub fn file_size(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Write file
        write_file(&file_path, "Hello, world!").unwrap();

        // Check existence
        assert!(file_exists(&file_path));

        // Read file
        let content = read_file(&file_path).unwrap();
        assert_eq!(content, "Hello, world!");

        // Check size
        let size = file_size(&file_path).unwrap();
        assert_eq!(size, 13);
    }

    #[test]
    fn test_directory_operations() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");

        // Create directory
        create_dir_all(&sub_dir).unwrap();

        // Check existence
        assert!(dir_exists(&sub_dir));

        // List subdirectories
        let dirs = list_subdirectories(temp_dir.path()).unwrap();
        assert_eq!(dirs.len(), 1);
    }
}
