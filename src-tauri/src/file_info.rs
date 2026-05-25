use crate::checksum::sha256_file;
use crate::file_detection::{detect_file, FileCategory};
use crate::tool_detection::{detect_tools, ToolAvailability};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub path: String,
    pub file_name: String,
    pub extension: String,
    pub category: FileCategory,
    pub is_decompressible: bool,
    pub size_bytes: u64,
    pub sha256: String,
    pub tools: ToolAvailability,
}

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
