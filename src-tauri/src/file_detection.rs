//! File type detection by extension and content (magic bytes).
//!
//! Determines the [`FileCategory`] of a file through a two-pass strategy:
//! first by inspecting magic bytes via the `infer` crate, then falling back
//! to extension-based classification.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Broad classification of a file's content type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FileCategory {
    /// Raster image (JPEG, PNG, WebP).
    Image,
    /// Video container (MP4, MOV, MKV, AVI).
    Video,
    /// Portable Document Format.
    Pdf,
    /// Any other file type processed with generic compression.
    Generic,
}

/// Result of detecting a file's type and compression eligibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetection {
    /// Lowercase file extension (empty string if none).
    pub extension: String,
    /// Resolved category used for compression dispatch.
    pub category: FileCategory,
    /// Category inferred from magic bytes, if available.
    pub inferred_category: Option<FileCategory>,
    /// Whether the file extension is accepted as compression input.
    pub is_supported_input: bool,
    /// Whether the file can be decompressed by a known algorithm.
    pub is_decompressible: bool,
}

/// Detects the file type at `path` using magic bytes and extension matching.
///
/// Returns a [`FileDetection`] containing the resolved category, extension,
/// and flags indicating whether the file can be compressed or decompressed.
pub fn detect_file(path: &Path) -> FileDetection {
    let extension = normalized_extension(path);
    let mut inferred_category = None;
    
    if let Ok(Some(kind)) = infer::get_from_path(path) {
        let mime = kind.mime_type();
        if mime.starts_with("image/") {
            inferred_category = Some(FileCategory::Image);
        } else if mime.starts_with("video/") {
            inferred_category = Some(FileCategory::Video);
        } else if mime == "application/pdf" {
            inferred_category = Some(FileCategory::Pdf);
        }
    }

    let category = inferred_category.unwrap_or_else(|| category_from_extension(&extension));

    let is_supported_input = matches!(
        extension.as_str(),
        "jpg"
            | "jpeg"
            | "png"
            | "webp"
            | "mp4"
            | "mov"
            | "mkv"
            | "avi"
            | "pdf"
            | "txt"
            | "csv"
            | "json"
            | "xml"
            | "log"
            | "docx"
            | "zst"
            | "rle"
    ) || category == FileCategory::Generic;

    let is_decompressible = matches!(extension.as_str(), "zst" | "rle" | "shnc" | "sfc" | "huff" | "lz77" | "lz78" | "lzw" | "arith");

    FileDetection {
        extension,
        category,
        inferred_category,
        is_supported_input,
        is_decompressible,
    }
}

/// Extracts and lowercases the file extension from `path`.
///
/// Returns an empty string if the path has no extension.
pub fn normalized_extension(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default()
}

/// Maps a file extension to its [`FileCategory`].
fn category_from_extension(extension: &str) -> FileCategory {
    match extension {
        "jpg" | "jpeg" | "png" | "webp" => FileCategory::Image,
        "mp4" | "mov" | "mkv" | "avi" => FileCategory::Video,
        "pdf" => FileCategory::Pdf,
        _ => FileCategory::Generic,
    }
}
