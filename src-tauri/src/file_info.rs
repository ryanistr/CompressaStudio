//! File metadata inspection for the frontend file-info panel.
//!
//! Gathers size, checksum, category, and tool availability into a single
//! [`FileInfo`] payload returned over IPC.

use crate::checksum::sha256_file;
use crate::file_detection::{detect_file, FileCategory};
use crate::tool_detection::{detect_tools, ToolAvailability};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Aggregated metadata about a user-selected file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// Absolute path to the file.
    pub path: String,
    /// Base file name including extension.
    pub file_name: String,
    /// Lowercase file extension.
    pub extension: String,
    /// Detected content category.
    pub category: FileCategory,
    /// Whether the file can be decompressed by a known algorithm.
    pub is_decompressible: bool,
    /// File size in bytes.
    pub size_bytes: u64,
    /// Hex-encoded SHA-256 digest.
    pub sha256: String,
    /// External tool availability snapshot.
    pub tools: ToolAvailability,
}

/// Inspects the file at `path` and returns a populated [`FileInfo`].
///
/// Validates that `path` exists and is a regular file, then reads metadata,
/// detects the file category, computes a SHA-256 checksum, and probes for
/// external tool availability.
///
/// # Errors
///
/// Returns an error if the path is invalid, metadata cannot be read, or
/// checksum computation fails.
pub fn inspect_file(path: &Path) -> Result<FileInfo> {
    if !path.exists() {
        return Err(anyhow!("File does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(anyhow!("Selected path is not a file: {}", path.display()));
    }

    let metadata = fs::metadata(path)
        .with_context(|| format!("Unable to read metadata for {}", path.display()))?;
    let detection = detect_file(path);
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Unable to determine file name for {}", path.display()))?
        .to_string();

    Ok(FileInfo {
        path: path.to_string_lossy().to_string(),
        file_name,
        extension: detection.extension,
        category: detection.category,
        is_decompressible: detection.is_decompressible,
        size_bytes: metadata.len(),
        sha256: sha256_file(path)?,
        tools: detect_tools(),
    })
}
