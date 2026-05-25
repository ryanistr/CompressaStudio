pub mod generic_compressor;
pub mod image_compressor;
pub mod native_dict;
pub mod native_entropy;
pub mod native_jpeg;
pub mod native_rle;
pub mod pdf_compressor;
pub mod video_compressor;
use crate::checksum::sha256_file;
use crate::file_detection::{detect_file, normalized_extension, FileCategory};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub const METADATA_SUFFIX: &str = ".meta.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QualityPreset {
    HighQuality,
    Balanced,
    SmallSize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PdfPreset {
    Screen,
    Ebook,
    Print,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GenericAlgorithm {
    Zstd,
    Rle,
    Shannon,
    ShannonFano,
    Huffman,
    Lz77,
    Lz78,
    Lzw,
    Arithmetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionRequest {
    pub quality_preset: Option<QualityPreset>,
    pub pdf_preset: Option<PdfPreset>,
    pub generic_algorithm: Option<GenericAlgorithm>,
    pub resize_percent: Option<u32>,
    pub zstd_level: Option<i32>,
    pub use_native_jpeg: Option<bool>,
    pub override_category: Option<FileCategory>,
}

impl Default for CompressionRequest {
    fn default() -> Self {
        Self {
            quality_preset: Some(QualityPreset::Balanced),
            pdf_preset: Some(PdfPreset::Ebook),
            generic_algorithm: Some(GenericAlgorithm::Zstd),
            resize_percent: None,
            zstd_level: Some(generic_compressor::BALANCED_LEVEL),
            use_native_jpeg: Some(false),
            override_category: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionOutcome {
    pub output_path: String,
    pub method: String,
    pub lossless: bool,
    pub message: String,
    pub metadata_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResult {
    pub operation: String,
    pub category: FileCategory,
    pub method: String,
    pub lossless: bool,
    pub input_path: String,
    pub output_path: String,
    pub original_size: u64,
    pub output_size: u64,
    pub saved_size: i64,
    pub compression_ratio: f64,
    pub space_saved: f64,
    pub elapsed_ms: u64,
    pub source_sha256: String,
    pub output_sha256: String,
    pub expected_sha256: Option<String>,
    pub integrity_match: Option<bool>,
    pub metadata_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionMetadata {
    pub original_file_name: String,
    pub original_extension: String,
    pub original_sha256: String,
    pub original_size: u64,
    pub algorithm: String,
}

pub fn compress(input_path: &Path, request: CompressionRequest) -> Result<OperationResult> {
    validate_input_file(input_path)?;

    let detection = detect_file(input_path);
    let mut final_category = detection.category;

    if let Some(override_cat) = request.override_category {
        if let Some(inferred) = detection.inferred_category {
            // Magic bytes exist. We block if the user overrides a definitively detected media type
            // to a DIFFERENT definitive media type (Image, Video, Pdf) to prevent processing failure/corruption.
            if inferred != override_cat && override_cat != FileCategory::Generic {
                return Err(anyhow!(
                    "Content Mismatch: You selected {:?} Compression, but the file contents are actually {:?}. Compression aborted to prevent corruption.",
                    override_cat, inferred
                ));
            }
        }
        final_category = override_cat;
    }

    let original_size = fs::metadata(input_path)?.len();
    let source_sha256 = sha256_file(input_path)?;
    let started_at = Instant::now();

    let outcome = match final_category {
        FileCategory::Image => image_compressor::compress_image(input_path, &request)?,
        FileCategory::Video => video_compressor::compress_video(input_path, &request)?,
        FileCategory::Pdf => pdf_compressor::compress_pdf(input_path, &request)?,
        FileCategory::Generic => generic_compressor::compress_generic(input_path, &request)?,
    };

    let output_path = PathBuf::from(&outcome.output_path);
    ensure_output_exists(&output_path)?;

    let elapsed_ms = started_at.elapsed().as_millis() as u64;
    let output_size = fs::metadata(&output_path)?.len();
    let output_sha256 = sha256_file(&output_path)?;

    Ok(OperationResult {
        operation: "compress".to_string(),
        category: final_category,
        method: outcome.method,
        lossless: outcome.lossless,
        input_path: input_path.to_string_lossy().to_string(),
        output_path: output_path.to_string_lossy().to_string(),
        original_size,
        output_size,
        saved_size: original_size as i64 - output_size as i64,
        compression_ratio: calculate_compression_ratio(output_size, original_size),
        space_saved: calculate_space_saved(output_size, original_size),
        elapsed_ms,
        source_sha256,
        output_sha256,
        expected_sha256: None,
        integrity_match: None,
        metadata_path: outcome.metadata_path,
        message: outcome.message,
    })
}

pub fn decompress(input_path: &Path) -> Result<OperationResult> {
    validate_input_file(input_path)?;
    let extension = normalized_extension(input_path);
    let original_size = fs::metadata(input_path)?.len();
    let source_sha256 = sha256_file(input_path)?;
    let started_at = Instant::now();

    let (output_path, method, metadata) = match extension.as_str() {
        "zst" => generic_compressor::decompress_zstd(input_path)?,
        "rle" => native_rle::decompress_rle(input_path)?,
        "shnc" => native_entropy::decompress_shannon(input_path)?,
        "sfc" => native_entropy::decompress_shannon_fano(input_path)?,
        "huff" => native_entropy::decompress_huffman(input_path)?,
        "lz77" => native_dict::decompress_lz77(input_path)?,
        "lz78" => native_dict::decompress_lz78(input_path)?,
        "lzw" => native_dict::decompress_lzw(input_path)?,
        "arith" => native_entropy::decompress_arithmetic(input_path)?,
        _ => {
            return Err(anyhow!(
                "Decompression is only supported for generic compressed outputs (.zst, .rle, .shnc, .sfc, .huff, .lz77, .lz78, .lzw, .arith)."
            ));
        }
    };

    ensure_output_exists(&output_path)?;

    let elapsed_ms = started_at.elapsed().as_millis() as u64;
    let output_size = fs::metadata(&output_path)?.len();
    let output_sha256 = sha256_file(&output_path)?;
    let expected_sha256 = metadata.as_ref().map(|item| item.original_sha256.clone());
    let integrity_match = expected_sha256
        .as_ref()
        .map(|expected| expected.eq_ignore_ascii_case(&output_sha256));

    let message = match integrity_match {
        Some(true) => format!("{method} decompression completed and checksum verified."),
        Some(false) => {
            format!("{method} decompression completed but checksum verification failed.")
        }
        None => format!("{method} decompression completed. No checksum metadata was found."),
    };

    Ok(OperationResult {
        operation: "decompress".to_string(),
        category: FileCategory::Generic,
        method,
        lossless: true,
        input_path: input_path.to_string_lossy().to_string(),
        output_path: output_path.to_string_lossy().to_string(),
        original_size,
        output_size,
        saved_size: original_size as i64 - output_size as i64,
        compression_ratio: calculate_compression_ratio(output_size, original_size),
        space_saved: calculate_space_saved(output_size, original_size),
        elapsed_ms,
        source_sha256,
        output_sha256,
        expected_sha256,
        integrity_match,
        metadata_path: metadata
            .map(|_| metadata_path_for(input_path).to_string_lossy().to_string()),
        message,
    })
}

pub fn validate_input_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("File not found: {}", path.display()));
    }

    if !path.is_file() {
        return Err(anyhow!("Path is not a file: {}", path.display()));
    }

    Ok(())
}

pub fn ensure_output_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!(
            "Compression reported success but no output file was created: {}",
            path.display()
        ));
    }

    if !path.is_file() {
        return Err(anyhow!("Output path is not a file: {}", path.display()));
    }

    Ok(())
}

pub fn calculate_compression_ratio(output_size: u64, original_size: u64) -> f64 {
    if original_size == 0 {
        return 0.0;
    }

    output_size as f64 / original_size as f64 * 100.0
}

pub fn calculate_space_saved(output_size: u64, original_size: u64) -> f64 {
    if original_size == 0 {
        return 0.0;
    }

    (original_size as f64 - output_size as f64) / original_size as f64 * 100.0
}

pub fn metadata_path_for(output_path: &Path) -> PathBuf {
    PathBuf::from(format!(
        "{}{}",
        output_path.to_string_lossy(),
        METADATA_SUFFIX
    ))
}

pub fn write_metadata(path: &Path, metadata: &CompressionMetadata) -> Result<()> {
    let content = serde_json::to_vec_pretty(metadata).context("Failed to serialize metadata")?;
    fs::write(path, content)
        .with_context(|| format!("Failed to write metadata file: {}", path.display()))?;
    Ok(())
}

pub fn read_metadata(path: &Path) -> Option<CompressionMetadata> {
    let raw = fs::read(path).ok()?;
    serde_json::from_slice::<CompressionMetadata>(&raw).ok()
}

pub fn create_metadata(input_path: &Path, algorithm: &str) -> Result<CompressionMetadata> {
    Ok(CompressionMetadata {
        original_file_name: input_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string(),
        original_extension: normalized_extension(input_path),
        original_sha256: sha256_file(input_path)?,
        original_size: fs::metadata(input_path)?.len(),
        algorithm: algorithm.to_string(),
    })
}

pub fn infer_restored_output_path(input_path: &Path) -> PathBuf {
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let base_name = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or("file");

    let candidate = parent.join(base_name);
    if !candidate.exists() {
        return candidate;
    }

    let mut counter = 1_u32;
    loop {
        let numbered = parent.join(format!("{base_name}_restored_{counter}"));
        if !numbered.exists() {
            return numbered;
        }
        counter += 1;
    }
}

pub fn unique_output_path(parent: &Path, stem: &str, extension: &str) -> PathBuf {
    let normalized_stem = if stem.is_empty() { "compressed" } else { stem };
    let extension = extension.trim_start_matches('.');
    let first = parent.join(format!("{normalized_stem}.{extension}"));

    if !first.exists() {
        return first;
    }

    let mut counter = 1_u32;
    loop {
        let candidate = parent.join(format!("{normalized_stem}_{counter}.{extension}"));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}
