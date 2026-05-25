//! Generic Compressor module.
//!
//! Handles generic compressor operations.

use super::{
    create_metadata, infer_restored_output_path, metadata_path_for, read_metadata, write_metadata,
    CompressionMetadata, CompressionOutcome, CompressionRequest, GenericAlgorithm,
};
use crate::compression::{native_dict, native_entropy, native_rle};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub const FAST_LEVEL: i32 = 1;
pub const BALANCED_LEVEL: i32 = 6;
pub const MAXIMUM_LEVEL: i32 = 19;

/// Compress Generic.
pub fn compress_generic(
    input_path: &Path,
    request: &CompressionRequest,
) -> Result<CompressionOutcome> {
    let algorithm = request.generic_algorithm.unwrap_or(GenericAlgorithm::Zstd);

    match algorithm {
        GenericAlgorithm::Zstd => {
            compress_zstd(input_path, request.zstd_level.unwrap_or(BALANCED_LEVEL))
        }
        GenericAlgorithm::Rle => native_rle::compress_rle(input_path),
        GenericAlgorithm::Shannon => native_entropy::compress_shannon(input_path),
        GenericAlgorithm::ShannonFano => native_entropy::compress_shannon_fano(input_path),
        GenericAlgorithm::Huffman => native_entropy::compress_huffman(input_path),
        GenericAlgorithm::Lz77 => native_dict::compress_lz77(input_path),
        GenericAlgorithm::Lz78 => native_dict::compress_lz78(input_path),
        GenericAlgorithm::Lzw => native_dict::compress_lzw(input_path),
        GenericAlgorithm::Arithmetic => native_entropy::compress_arithmetic(input_path),
    }
}

/// Compress Zstd.
pub fn compress_zstd(input_path: &Path, level: i32) -> Result<CompressionOutcome> {
    let output_path = PathBuf::from(format!("{}.zst", input_path.to_string_lossy()));
    let mut reader = BufReader::new(
        File::open(input_path)
            .with_context(|| format!("Unable to read input file: {}", input_path.display()))?,
    );
    let writer = BufWriter::new(
        File::create(&output_path)
            .with_context(|| format!("Unable to create output file: {}", output_path.display()))?,
    );
    let mut encoder = zstd::stream::Encoder::new(writer, clamp_level(level))
        .context("Failed to initialize zstd encoder")?;

    std::io::copy(&mut reader, &mut encoder).context("zstd compression failed")?;
    let mut writer = encoder.finish().context("Unable to finalize zstd file")?;
    writer.flush().context("Unable to flush zstd output")?;

    let metadata = create_metadata(input_path, "Zstandard dictionary/statistical compression")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Generic Lossless Compression (Zstd)".to_string(),
        lossless: true,
        message: format!(
            "Generic lossless compression completed using zstd level {}.",
            clamp_level(level)
        ),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

/// Decompress Zstd.
pub fn decompress_zstd(
    input_path: &Path,
) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let reader =
        BufReader::new(File::open(input_path).with_context(|| {
            format!("Unable to read compressed file: {}", input_path.display())
        })?);
    let mut decoder =
        zstd::stream::Decoder::new(reader).context("Failed to initialize zstd decoder")?;

    let output_path = infer_zstd_output_path(input_path);
    let mut writer = BufWriter::new(File::create(&output_path).with_context(|| {
        format!(
            "Unable to create decompressed file: {}",
            output_path.display()
        )
    })?);
    std::io::copy(&mut decoder, &mut writer).context("zstd decompression failed")?;
    writer
        .flush()
        .context("Unable to flush decompressed output")?;

    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);

    Ok((
        output_path,
        "Generic Lossless Compression (Zstd)".to_string(),
        metadata,
    ))
}

fn infer_zstd_output_path(input_path: &Path) -> PathBuf {
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let base_name = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or("file");

    let direct = parent.join(base_name);
    if !direct.exists() {
        return direct;
    }

    infer_restored_output_path(input_path)
}

fn clamp_level(level: i32) -> i32 {
    level.clamp(FAST_LEVEL, MAXIMUM_LEVEL)
}

#[cfg(test)]
mod tests {
    use super::{compress_zstd, decompress_zstd, BALANCED_LEVEL};
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn zstd_roundtrip_restores_original_bytes() -> Result<()> {
        let fixture = unique_path("compressa_generic_fixture.txt");
        fs::write(&fixture, b"aaaaabbbbbccccccddddddaaaaabbbbb")?;

        let result = compress_zstd(&fixture, BALANCED_LEVEL)?;
        let compressed = PathBuf::from(result.output_path);
        let (restored, _, _) = decompress_zstd(&compressed)?;

        let original = fs::read(&fixture)?;
        let restored_bytes = fs::read(&restored)?;

        if fixture.exists() {
            fs::remove_file(&fixture)?;
        }
        if compressed.exists() {
            fs::remove_file(&compressed)?;
        }
        let metadata = PathBuf::from(format!("{}.meta.json", compressed.to_string_lossy()));
        if metadata.exists() {
            fs::remove_file(metadata)?;
        }
        if restored.exists() {
            fs::remove_file(restored)?;
        }

        assert_eq!(original, restored_bytes);
        Ok(())
    }

    fn unique_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}_{name}"))
    }
}
