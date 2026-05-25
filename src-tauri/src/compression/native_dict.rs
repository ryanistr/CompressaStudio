use super::{
    create_metadata, infer_restored_output_path, metadata_path_for, read_metadata, write_metadata,
    CompressionMetadata, CompressionOutcome,
};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const LZ77_HEADER: &[u8] = b"COMPRESSA_LZ77_V1";
const LZ78_HEADER: &[u8] = b"COMPRESSA_LZ78_V1";
const LZW_HEADER: &[u8] = b"COMPRESSA_LZW__V1";

// --- LZ77 ---

pub fn compress_lz77(input_path: &Path) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).with_context(|| {
        format!("Unable to read input file for LZ77: {}", input_path.display())
    })?;
    let encoded = encode_lz77(&input);
    let output_path = PathBuf::from(format!("{}.lz77", input_path.to_string_lossy()));
    let mut output = File::create(&output_path)
        .with_context(|| format!("Unable to create LZ77 output: {}", output_path.display()))?;

    output.write_all(LZ77_HEADER)?;
    output.write_all(&(input.len() as u64).to_be_bytes())?;
    output.write_all(&encoded)?;
    output.flush()?;

    let metadata = create_metadata(input_path, "Educational LZ77")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Dictionary Compression (Educational LZ77)".to_string(),
        lossless: true,
        message: "Educational LZ77 compression completed.".to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

pub fn decompress_lz77(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let mut source = File::open(input_path)
        .with_context(|| format!("Unable to open LZ77 file: {}", input_path.display()))?;
    let mut raw = Vec::new();
    source.read_to_end(&mut raw)?;

    let payload = raw.strip_prefix(LZ77_HEADER).context("Invalid LZ77 file header.")?;
    if payload.len() < 8 {
        anyhow::bail!("LZ77 payload too small to contain original length.");
    }
    let original_len = u64::from_be_bytes(payload[0..8].try_into().unwrap()) as usize;
    let data = &payload[8..];

    let decoded = decode_lz77(data, original_len)?;
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).with_context(|| {
        format!("Unable to write restored LZ77 file: {}", output_path.display())
    })?;

    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);

    Ok((
        output_path,
        "Dictionary Compression (Educational LZ77)".to_string(),
        metadata,
    ))
}

fn encode_lz77(input: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut pos = 0;
    
    let mut head = vec![usize::MAX; 65536];
    let mut prev = vec![usize::MAX; 65536];
    
    fn hash(bytes: &[u8]) -> usize {
        let mut h = 0usize;
        h = h.wrapping_add(bytes[0] as usize);
        h = h.wrapping_shl(5) ^ bytes[1] as usize;
        h = h.wrapping_shl(5) ^ bytes[2] as usize;
        h & 0xFFFF
    }
    
    while pos < input.len() {
        let mut best_len = 0;
        let mut best_offset = 0;
        let lookahead_len = std::cmp::min(input.len() - pos, 255);
        
        if lookahead_len >= 3 {
            let h = hash(&input[pos..pos+3]);
            let mut curr = head[h];
            let mut limit = 100;
            while curr != usize::MAX && pos.saturating_sub(curr) <= 65535 && limit > 0 {
                if curr >= pos { break; }
                let mut match_len = 0;
                while match_len < lookahead_len && input[curr + match_len] == input[pos + match_len] {
                    match_len += 1;
                }
                if match_len > best_len {
                    best_len = match_len;
                    best_offset = pos - curr;
                    if best_len == lookahead_len { break; }
                }
                curr = prev[curr % 65536];
                limit -= 1;
            }
        }
        
        let next_byte = if pos + best_len < input.len() {
            input[pos + best_len]
        } else {
            0
        };
        
        encoded.extend((best_offset as u16).to_be_bytes());
        encoded.push(best_len as u8);
        encoded.push(next_byte);
        
        let advance = best_len + 1;
        for i in 0..advance {
            let p = pos + i;
            if p + 2 < input.len() {
                let h = hash(&input[p..p+3]);
                prev[p % 65536] = head[h];
                head[h] = p;
            }
        }
        
        pos += advance;
    }
    encoded
}

fn decode_lz77(input: &[u8], original_len: usize) -> Result<Vec<u8>> {
    let mut decoded = Vec::with_capacity(original_len);
    let mut chunks = input.chunks_exact(4);
    for chunk in &mut chunks {
        let offset = u16::from_be_bytes([chunk[0], chunk[1]]) as usize;
        let len = chunk[2] as usize;
        let next_byte = chunk[3];
        
        if len > 0 {
            if offset > decoded.len() || offset == 0 {
                anyhow::bail!("Invalid LZ77 offset");
            }
            let start = decoded.len() - offset;
            for i in 0..len {
                decoded.push(decoded[start + i]);
            }
        }
        decoded.push(next_byte);
    }
    decoded.truncate(original_len);
    Ok(decoded)
}

// --- LZ78 ---

pub fn compress_lz78(input_path: &Path) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).context("Unable to read input file for LZ78")?;
    let encoded = encode_lz78(&input);
    let output_path = PathBuf::from(format!("{}.lz78", input_path.to_string_lossy()));
    let mut output = File::create(&output_path).context("Unable to create LZ78 output")?;

    output.write_all(LZ78_HEADER)?;
    output.write_all(&(input.len() as u64).to_be_bytes())?;
    output.write_all(&encoded)?;
    output.flush()?;

    let metadata = create_metadata(input_path, "Educational LZ78")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Dictionary Compression (Educational LZ78)".to_string(),
        lossless: true,
        message: "Educational LZ78 compression completed.".to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

pub fn decompress_lz78(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let mut source = File::open(input_path).context("Unable to open LZ78 file")?;
    let mut raw = Vec::new();
    source.read_to_end(&mut raw)?;

    let payload = raw.strip_prefix(LZ78_HEADER).context("Invalid LZ78 file header.")?;
    if payload.len() < 8 {
        anyhow::bail!("LZ78 payload too small.");
    }
    let original_len = u64::from_be_bytes(payload[0..8].try_into().unwrap()) as usize;
    let data = &payload[8..];

    let decoded = decode_lz78(data, original_len)?;
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).context("Unable to write restored LZ78 file")?;

    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);

    Ok((
        output_path,
        "Dictionary Compression (Educational LZ78)".to_string(),
        metadata,
    ))
}

fn encode_lz78(input: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut dict: HashMap<(u32, u8), u32> = HashMap::new();
    let mut dict_idx = 1u32;
    let mut current_prefix = 0u32;

    for &byte in input {
        match dict.get(&(current_prefix, byte)) {
            Some(&next_idx) => {
                current_prefix = next_idx;
            }
            None => {
                encoded.extend(current_prefix.to_be_bytes());
                encoded.push(byte);
                
                if dict_idx < u32::MAX {
                    dict.insert((current_prefix, byte), dict_idx);
                    dict_idx += 1;
                }
                current_prefix = 0;
            }
        }
    }
    if current_prefix != 0 {
        encoded.extend(current_prefix.to_be_bytes());
        encoded.push(0);
    }
    encoded
}

fn decode_lz78(input: &[u8], original_len: usize) -> Result<Vec<u8>> {
    let mut decoded = Vec::with_capacity(original_len);
    let mut dict: Vec<Vec<u8>> = Vec::new();
    dict.push(Vec::new()); 
    
    let mut chunks = input.chunks_exact(5);
    for chunk in &mut chunks {
        let prefix_idx = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
        let byte = chunk[4];
        
        let prefix_idx_usize = prefix_idx as usize;
        if prefix_idx_usize >= dict.len() {
            anyhow::bail!("Invalid LZ78 dictionary index");
        }
        
        let mut entry = dict[prefix_idx_usize].clone();
        entry.push(byte);
        decoded.extend(&entry);
        
        dict.push(entry);
    }
    decoded.truncate(original_len);
    Ok(decoded)
}

// --- LZW ---

pub fn compress_lzw(input_path: &Path) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).context("Unable to read input file for LZW")?;
    let encoded = encode_lzw(&input);
    let output_path = PathBuf::from(format!("{}.lzw", input_path.to_string_lossy()));
    let mut output = File::create(&output_path).context("Unable to create LZW output")?;

    output.write_all(LZW_HEADER)?;
    output.write_all(&(input.len() as u64).to_be_bytes())?;
    output.write_all(&encoded)?;
    output.flush()?;

    let metadata = create_metadata(input_path, "Educational LZW")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Dictionary Compression (Educational LZW)".to_string(),
        lossless: true,
        message: "Educational LZW compression completed.".to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

pub fn decompress_lzw(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let mut source = File::open(input_path).context("Unable to open LZW file")?;
    let mut raw = Vec::new();
    source.read_to_end(&mut raw)?;

    let payload = raw.strip_prefix(LZW_HEADER).context("Invalid LZW file header.")?;
    if payload.len() < 8 {
        anyhow::bail!("LZW payload too small.");
    }
    let original_len = u64::from_be_bytes(payload[0..8].try_into().unwrap()) as usize;
    let data = &payload[8..];

    let decoded = decode_lzw(data, original_len)?;
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).context("Unable to write restored LZW file")?;

    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);

    Ok((
        output_path,
        "Dictionary Compression (Educational LZW)".to_string(),
        metadata,
    ))
}

fn encode_lzw(input: &[u8]) -> Vec<u8> {
    if input.is_empty() { return Vec::new(); }
    let mut encoded = Vec::new();
    let mut dict: HashMap<(u32, u8), u32> = HashMap::new();
    let mut dict_idx = 256u32;
    
    let mut current_prefix = input[0] as u32;

    for &byte in &input[1..] {
        if let Some(&next_idx) = dict.get(&(current_prefix, byte)) {
            current_prefix = next_idx;
        } else {
            encoded.extend(current_prefix.to_be_bytes());
            if dict_idx < u32::MAX {
                dict.insert((current_prefix, byte), dict_idx);
                dict_idx += 1;
            }
            current_prefix = byte as u32;
        }
    }
    encoded.extend(current_prefix.to_be_bytes());
    encoded
}

fn decode_lzw(input: &[u8], original_len: usize) -> Result<Vec<u8>> {
    if input.is_empty() { return Ok(Vec::new()); }
    let mut decoded = Vec::with_capacity(original_len);
    
    let mut dict: Vec<Vec<u8>> = (0..=255).map(|i| vec![i as u8]).collect();
    let mut chunks = input.chunks_exact(4);
    
    if let Some(first_chunk) = chunks.next() {
        let mut old_code = u32::from_be_bytes(first_chunk[0..4].try_into().unwrap());
        let mut old_code_usize = old_code as usize;
        if old_code_usize >= dict.len() {
            anyhow::bail!("Invalid LZW code");
        }
        decoded.extend(&dict[old_code_usize]);
        
        for chunk in chunks {
            let new_code = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let new_code_usize = new_code as usize;
            
            let entry = if new_code_usize < dict.len() {
                dict[new_code_usize].clone()
            } else if new_code_usize == dict.len() {
                let mut entry = dict[old_code_usize].clone();
                entry.push(entry[0]);
                entry
            } else {
                anyhow::bail!("Invalid LZW dictionary sequence");
            };
            
            decoded.extend(&entry);
            
            let mut new_entry = dict[old_code_usize].clone();
            new_entry.push(entry[0]);
            dict.push(new_entry);
            
            old_code = new_code;
            old_code_usize = new_code_usize;
        }
    }
    decoded.truncate(original_len);
    Ok(decoded)
}
