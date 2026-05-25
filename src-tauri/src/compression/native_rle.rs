use super::{
    create_metadata, infer_restored_output_path, metadata_path_for, read_metadata, write_metadata,
    CompressionMetadata, CompressionOutcome,
};
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const RLE_HEADER: &[u8] = b"COMPRESSA_RLE_V1";

pub fn compress_rle(input_path: &Path) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).with_context(|| {
        format!(
            "Unable to read input file for RLE: {}",
            input_path.display()
        )
    })?;
    let encoded = encode_rle(&input);
    let output_path = PathBuf::from(format!("{}.rle", input_path.to_string_lossy()));
    let mut output = File::create(&output_path)
        .with_context(|| format!("Unable to create RLE output: {}", output_path.display()))?;

    output.write_all(RLE_HEADER)?;
    output.write_all(&encoded)?;
    output.flush()?;

    let metadata = create_metadata(input_path, "Educational Run-Length Encoding (RLE)")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Generic Lossless Compression (Educational RLE)".to_string(),
        lossless: true,
        message: "Educational RLE compression completed. Best suited for repetitive byte patterns."
            .to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

pub fn decompress_rle(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let mut source = File::open(input_path)
        .with_context(|| format!("Unable to open RLE file: {}", input_path.display()))?;
    let mut raw = Vec::new();
    source.read_to_end(&mut raw)?;

    let payload = raw
        .strip_prefix(RLE_HEADER)
        .context("Invalid RLE file header.")?;
    let decoded = decode_rle(payload)?;
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).with_context(|| {
        format!(
            "Unable to write restored RLE file: {}",
            output_path.display()
        )
    })?;

    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);

    Ok((
        output_path,
        "Generic Lossless Compression (Educational RLE)".to_string(),
        metadata,
    ))
}

fn encode_rle(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut encoded = Vec::with_capacity(input.len());
    let mut current = input[0];
    let mut count: u8 = 1;

    for &byte in &input[1..] {
        if byte == current && count < u8::MAX {
            count += 1;
        } else {
            encoded.push(count);
            encoded.push(current);
            current = byte;
            count = 1;
        }
    }

    encoded.push(count);
    encoded.push(current);
    encoded
}

fn decode_rle(input: &[u8]) -> Result<Vec<u8>> {
    let mut decoded = Vec::new();
    let mut chunks = input.chunks_exact(2);

    for chunk in &mut chunks {
        let count = chunk[0];
        let value = chunk[1];
        decoded.extend(std::iter::repeat_n(value, usize::from(count)));
    }

    if !chunks.remainder().is_empty() {
        anyhow::bail!("Malformed RLE payload.");
    }

    Ok(decoded)
}
