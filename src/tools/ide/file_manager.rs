//! # File Manager
//!
//! Handles file system operations for the IDE tool.

use std::fs;
use std::path::Path;
use std::io::{self, Read};

/// Directory entry information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub size: u64,
}

/// File operation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileResult {
    pub success: bool,
    pub message: String,
    pub path: String,
}

/// File Manager for IDE operations
pub struct FileManager;

impl FileManager {
    /// Create a new file manager
    pub fn new() -> Self {
        Self
    }

    /// Read file contents
    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }

        if !path.is_file() {
            return Err(format!("Path is not a file: {}", path.display()));
        }

        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        Ok(contents)
    }

    /// Write file contents
    pub fn write_file(&self, path: &str, contents: &str) -> Result<(), String> {
        let path = Path::new(path);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }

        fs::write(path, contents)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    /// List directory contents
    pub fn list_directory(&self, path: &str) -> Result<Vec<DirectoryEntry>, String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(format!("Directory not found: {}", path.display()));
        }

        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", path.display()));
        }

        let mut entries = Vec::new();

        let dir_entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in dir_entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let metadata = entry.metadata()
                .map_err(|e| format!("Failed to read metadata: {}", e))?;

            let name = entry.file_name().to_string_lossy().to_string();
            let entry_path = entry.path().to_string_lossy().to_string();

            entries.push(DirectoryEntry {
                name,
                path: entry_path,
                is_dir: metadata.is_dir(),
                is_file: metadata.is_file(),
                size: metadata.len(),
            });
        }

        // Sort: directories first, then files, alphabetically
        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        Ok(entries)
    }

    /// Create a directory
    pub fn create_directory(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path);

        if path.exists() {
            return Err(format!("Path already exists: {}", path.display()));
        }

        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        Ok(())
    }

    /// Delete a file
    pub fn delete_file(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }

        if path.is_dir() {
            fs::remove_dir_all(path)
                .map_err(|e| format!("Failed to delete directory: {}", e))?;
        } else {
            fs::remove_file(path)
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }

        Ok(())
    }

    /// Check if path exists
    pub fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Get file metadata
    pub fn metadata(&self, path: &str) -> Result<fs::Metadata, String> {
        let path = Path::new(path);
        fs::metadata(path)
            .map_err(|e| format!("Failed to get metadata: {}", e))
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}
