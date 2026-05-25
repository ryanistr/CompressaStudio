use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FileCategory {
    Image,
    Video,
    Pdf,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetection {
    pub extension: String,
    pub category: FileCategory,
    pub inferred_category: Option<FileCategory>,
    pub is_supported_input: bool,
    pub is_decompressible: bool,
}

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

pub fn normalized_extension(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default()
}

fn category_from_extension(extension: &str) -> FileCategory {
    match extension {
        "jpg" | "jpeg" | "png" | "webp" => FileCategory::Image,
        "mp4" | "mov" | "mkv" | "avi" => FileCategory::Video,
        "pdf" => FileCategory::Pdf,
        _ => FileCategory::Generic,
    }
}
