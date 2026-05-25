//! Native Entropy module.
//!
//! Handles native entropy operations.

use super::{
    create_metadata, infer_restored_output_path, metadata_path_for, read_metadata, write_metadata,
    CompressionMetadata, CompressionOutcome,
};
use anyhow::{Context, Result};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

// --- Bit I/O ---

struct BitWriter {
    buffer: Vec<u8>,
    current_byte: u8,
    bits_in_byte: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self { buffer: Vec::new(), current_byte: 0, bits_in_byte: 0 }
    }
    fn write_bit(&mut self, bit: bool) {
        if bit { self.current_byte |= 1 << (7 - self.bits_in_byte); }
        self.bits_in_byte += 1;
        if self.bits_in_byte == 8 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bits_in_byte = 0;
        }
    }
    fn write_bits(&mut self, bits: &[bool]) {
        for &b in bits {
            self.write_bit(b);
        }
    }
    fn finish(mut self) -> (Vec<u8>, u8) {
        let valid_bits = self.bits_in_byte;
        if self.bits_in_byte > 0 {
            self.buffer.push(self.current_byte);
        }
        (self.buffer, valid_bits)
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    byte_idx: usize,
    bit_idx: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, byte_idx: 0, bit_idx: 0 }
    }
    fn read_bit(&mut self) -> Option<bool> {
        if self.byte_idx >= self.data.len() { return None; }
        let bit = (self.data[self.byte_idx] & (1 << (7 - self.bit_idx))) != 0;
        self.bit_idx += 1;
        if self.bit_idx == 8 {
            self.bit_idx = 0;
            self.byte_idx += 1;
        }
        Some(bit)
    }
}

// --- Tree-based Encoding/Decoding (Shannon, Shannon-Fano, Huffman) ---

fn encode_with_tree(
    input: &[u8],
    codes: &HashMap<u8, Vec<bool>>,
    freqs: &[u32; 256],
    header: &[u8],
) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(header);
    
    for &f in freqs {
        output.extend_from_slice(&f.to_le_bytes());
    }
    
    let mut bit_writer = BitWriter::new();
    for &byte in input {
        if let Some(code) = codes.get(&byte) {
            bit_writer.write_bits(code);
        }
    }
    let (data, valid_bits) = bit_writer.finish();
    output.push(valid_bits);
    output.extend_from_slice(&data);
    
    output
}

fn decode_with_tree(
    payload: &[u8],
    rebuild_codes_fn: fn(&[u32; 256]) -> HashMap<u8, Vec<bool>>,
) -> Result<Vec<u8>> {
    if payload.len() < 1024 {
        anyhow::bail!("Invalid payload size, cannot read frequencies");
    }
    
    let mut freqs = [0u32; 256];
    for i in 0..256 {
        let mut b = [0u8; 4];
        b.copy_from_slice(&payload[i*4..(i+1)*4]);
        freqs[i] = u32::from_le_bytes(b);
    }
    
    let total_symbols: u32 = freqs.iter().sum();
    if total_symbols == 0 {
        return Ok(Vec::new());
    }
    
    let codes = rebuild_codes_fn(&freqs);
    
    #[derive(Default)]
    struct DecodeNode {
        symbol: Option<u8>,
        left: Option<Box<DecodeNode>>,
        right: Option<Box<DecodeNode>>,
    }
    let mut root = DecodeNode::default();
    for (&sym, code) in &codes {
        let mut curr = &mut root;
        for &bit in code {
            if bit {
                if curr.right.is_none() { curr.right = Some(Box::new(DecodeNode::default())); }
                curr = curr.right.as_mut().unwrap();
            } else {
                if curr.left.is_none() { curr.left = Some(Box::new(DecodeNode::default())); }
                curr = curr.left.as_mut().unwrap();
            }
        }
        curr.symbol = Some(sym);
    }
    
    let data_start = 1024 + 1;
    let data = if data_start <= payload.len() { &payload[data_start..] } else { &[] };
    
    let mut decoded = Vec::with_capacity(total_symbols as usize);
    let mut bit_reader = BitReader::new(data);
    
    let mut curr = &root;
    while decoded.len() < total_symbols as usize {
        if let Some(sym) = curr.symbol {
            decoded.push(sym);
            curr = &root;
        } else {
            let bit = bit_reader.read_bit().context("Unexpected end of data")?;
            if bit {
                curr = curr.right.as_ref().context("Invalid bit sequence")?;
            } else {
                curr = curr.left.as_ref().context("Invalid bit sequence")?;
            }
            if let Some(sym) = curr.symbol {
                decoded.push(sym);
                curr = &root;
            }
        }
    }
    
    Ok(decoded)
}

fn compress_generic(
    input_path: &Path,
    header: &[u8],
    ext: &str,
    method_name: &str,
    short_name: &str,
    generate_codes: fn(&[u32; 256]) -> HashMap<u8, Vec<bool>>,
) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).with_context(|| {
        format!("Unable to read input file for {}: {}", short_name, input_path.display())
    })?;
    
    let mut freqs = [0u32; 256];
    for &b in &input {
        freqs[b as usize] += 1;
    }
    
    let codes = generate_codes(&freqs);
    let encoded = encode_with_tree(&input, &codes, &freqs, header);
    
    let output_path = PathBuf::from(format!("{}{}", input_path.to_string_lossy(), ext));
    fs::write(&output_path, &encoded).with_context(|| {
        format!("Unable to create {} output: {}", short_name, output_path.display())
    })?;
    
    let metadata = create_metadata(input_path, method_name)?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;
    
    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: method_name.to_string(),
        lossless: true,
        message: format!("{} compression completed.", method_name),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

fn decompress_generic(
    input_path: &Path,
    header: &[u8],
    method_name: &str,
    generate_codes: fn(&[u32; 256]) -> HashMap<u8, Vec<bool>>,
) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let raw = fs::read(input_path).with_context(|| {
        format!("Unable to open compressed file: {}", input_path.display())
    })?;
    
    let payload = raw.strip_prefix(header).context("Invalid file header.")?;
    let decoded = decode_with_tree(payload, generate_codes)?;
    
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).with_context(|| {
        format!("Unable to write restored file: {}", output_path.display())
    })?;
    
    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);
    
    Ok((
        output_path,
        method_name.to_string(),
        metadata,
    ))
}

// --- Algorithm Implementations ---

// 1. Shannon Coding
fn generate_shannon_codes(freqs: &[u32; 256]) -> HashMap<u8, Vec<bool>> {
    let mut total = 0f64;
    let mut symbols = Vec::new();
    for (i, &f) in freqs.iter().enumerate() {
        if f > 0 {
            symbols.push((i as u8, f));
            total += f as f64;
        }
    }
    symbols.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    
    let mut codes = HashMap::new();
    let mut c = 0f64;
    for &(sym, f) in &symbols {
        let p = f as f64 / total;
        let l = (-p.log2()).ceil() as usize;
        
        let mut code = Vec::new();
        let mut temp_c = c;
        for _ in 0..l {
            temp_c *= 2.0;
            if temp_c >= 1.0 {
                code.push(true);
                temp_c -= 1.0;
            } else {
                code.push(false);
            }
        }
        codes.insert(sym, code);
        c += p;
    }
    codes
}

/// Compress Shannon.
pub fn compress_shannon(input_path: &Path) -> Result<CompressionOutcome> {
    compress_generic(input_path, b"COMPRESSA_SHNC_V1", ".shnc", "Educational Shannon Coding", "Shannon", generate_shannon_codes)
}

/// Decompress Shannon.
pub fn decompress_shannon(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    decompress_generic(input_path, b"COMPRESSA_SHNC_V1", "Educational Shannon Coding", generate_shannon_codes)
}

// 2. Shannon-Fano Coding
fn generate_shannon_fano_codes(freqs: &[u32; 256]) -> HashMap<u8, Vec<bool>> {
    let mut symbols = Vec::new();
    for (i, &f) in freqs.iter().enumerate() {
        if f > 0 { symbols.push((i as u8, f)); }
    }
    symbols.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    
    let mut codes = HashMap::new();
    if symbols.len() == 1 {
        codes.insert(symbols[0].0, vec![false]);
        return codes;
    }
    
    fn sf_recursive(symbols: &[(u8, u32)], current_code: Vec<bool>, codes: &mut HashMap<u8, Vec<bool>>) {
        if symbols.len() == 1 {
            codes.insert(symbols[0].0, current_code);
            return;
        }
        let mut total = 0;
        for &(_, f) in symbols { total += f; }
        let mut running_sum = 0;
        let mut split_idx = 0;
        let mut min_diff = i64::MAX;
        
        for i in 0..symbols.len() - 1 {
            running_sum += symbols[i].1;
            let diff = (total as i64 - 2 * running_sum as i64).abs();
            if diff < min_diff {
                min_diff = diff;
                split_idx = i;
            }
        }
        
        let mut left_code = current_code.clone();
        left_code.push(false);
        sf_recursive(&symbols[..=split_idx], left_code, codes);
        
        let mut right_code = current_code;
        right_code.push(true);
        sf_recursive(&symbols[split_idx + 1..], right_code, codes);
    }
    
    if !symbols.is_empty() {
        sf_recursive(&symbols, Vec::new(), &mut codes);
    }
    codes
}

/// Compress Shannon Fano.
pub fn compress_shannon_fano(input_path: &Path) -> Result<CompressionOutcome> {
    compress_generic(input_path, b"COMPRESSA_SFC_V1", ".sfc", "Educational Shannon-Fano Coding", "Shannon-Fano", generate_shannon_fano_codes)
}

/// Decompress Shannon Fano.
pub fn decompress_shannon_fano(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    decompress_generic(input_path, b"COMPRESSA_SFC_V1", "Educational Shannon-Fano Coding", generate_shannon_fano_codes)
}

// 3. Huffman Coding
#[derive(Eq, PartialEq)]
struct HuffNode {
    freq: u32,
    symbol: Option<u8>,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq).then_with(|| {
            let s1 = self.symbol.unwrap_or(255);
            let s2 = other.symbol.unwrap_or(255);
            s1.cmp(&s2)
        })
    }
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn generate_huffman_codes(freqs: &[u32; 256]) -> HashMap<u8, Vec<bool>> {
    let mut heap = BinaryHeap::new();
    for (i, &f) in freqs.iter().enumerate() {
        if f > 0 {
            heap.push(Box::new(HuffNode {
                freq: f,
                symbol: Some(i as u8),
                left: None,
                right: None,
            }));
        }
    }
    
    if heap.is_empty() { return HashMap::new(); }
    if heap.len() == 1 {
        let mut codes = HashMap::new();
        codes.insert(heap.peek().unwrap().symbol.unwrap(), vec![false]);
        return codes;
    }
    
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let parent = Box::new(HuffNode {
            freq: left.freq + right.freq,
            symbol: None,
            left: Some(left),
            right: Some(right),
        });
        heap.push(parent);
    }
    
    let root = heap.pop().unwrap();
    let mut codes = HashMap::new();
    
    fn huff_recursive(node: &HuffNode, current_code: Vec<bool>, codes: &mut HashMap<u8, Vec<bool>>) {
        if let Some(sym) = node.symbol {
            codes.insert(sym, current_code);
        } else {
            if let Some(ref l) = node.left {
                let mut c = current_code.clone();
                c.push(false);
                huff_recursive(l, c, codes);
            }
            if let Some(ref r) = node.right {
                let mut c = current_code.clone();
                c.push(true);
                huff_recursive(r, c, codes);
            }
        }
    }
    
    huff_recursive(&root, Vec::new(), &mut codes);
    codes
}

/// Compress Huffman.
pub fn compress_huffman(input_path: &Path) -> Result<CompressionOutcome> {
    compress_generic(input_path, b"COMPRESSA_HUFF_V1", ".huff", "Educational Huffman Coding", "Huffman", generate_huffman_codes)
}

/// Decompress Huffman.
pub fn decompress_huffman(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    decompress_generic(input_path, b"COMPRESSA_HUFF_V1", "Educational Huffman Coding", generate_huffman_codes)
}

// 4. Arithmetic Coding
const ARITH_MAX_FREQ: u32 = 1 << 14;

struct ArithModel {
    cum_freq: [u32; 257],
}

impl ArithModel {
    fn new(freqs: &[u32; 256]) -> Self {
        let mut f = [0u32; 256];
        let mut total = 0u64;
        for &freq in freqs { total += freq as u64; }
        if total > 0 && total >= ARITH_MAX_FREQ as u64 {
            let mut current_total = 0;
            for i in 0..256 {
                let scaled = ((freqs[i] as u64 * (ARITH_MAX_FREQ as u64 - 256)) / total) as u32;
                f[i] = if freqs[i] > 0 { scaled.max(1) } else { 0 };
                current_total += f[i];
            }
        } else {
            f.copy_from_slice(freqs);
        }
        
        let mut cum_freq = [0u32; 257];
        let mut sum = 0;
        for i in 0..256 {
            cum_freq[i] = sum;
            sum += f[i];
        }
        cum_freq[256] = sum;
        Self { cum_freq }
    }
}

struct ArithmeticEncoder<'a> {
    low: u32,
    high: u32,
    underflow_bits: u32,
    bit_writer: &'a mut BitWriter,
}

impl<'a> ArithmeticEncoder<'a> {
    fn new(bit_writer: &'a mut BitWriter) -> Self {
        Self {
            low: 0,
            high: 0xFFFFFFFF,
            underflow_bits: 0,
            bit_writer,
        }
    }
    
    fn encode(&mut self, symbol: u8, model: &ArithModel) {
        let sym = symbol as usize;
        let range = (self.high - self.low) as u64 + 1;
        let total = model.cum_freq[256] as u64;
        
        let sym_low = model.cum_freq[sym] as u64;
        let sym_high = model.cum_freq[sym + 1] as u64;
        
        self.high = self.low + ((range * sym_high) / total) as u32 - 1;
        self.low = self.low + ((range * sym_low) / total) as u32;
        
        loop {
            if (self.high & 0x80000000) == (self.low & 0x80000000) {
                let bit = (self.high & 0x80000000) != 0;
                self.bit_writer.write_bit(bit);
                for _ in 0..self.underflow_bits {
                    self.bit_writer.write_bit(!bit);
                }
                self.underflow_bits = 0;
                self.low = (self.low << 1) & 0xFFFFFFFF;
                self.high = ((self.high << 1) | 1) & 0xFFFFFFFF;
            } else if (self.low & 0x40000000) != 0 && (self.high & 0x40000000) == 0 {
                self.underflow_bits += 1;
                self.low = (self.low << 1) & 0x7FFFFFFF;
                self.high = ((self.high << 1) | 0x80000001) & 0xFFFFFFFF;
            } else {
                break;
            }
        }
    }
    
    fn finish(&mut self) {
        let bit = (self.low & 0x40000000) != 0;
        self.bit_writer.write_bit(bit);
        for _ in 0..self.underflow_bits + 1 {
            self.bit_writer.write_bit(!bit);
        }
    }
}

struct ArithmeticDecoder<'a> {
    low: u32,
    high: u32,
    value: u32,
    bit_reader: &'a mut BitReader<'a>,
}

impl<'a> ArithmeticDecoder<'a> {
    fn new(bit_reader: &'a mut BitReader<'a>) -> Self {
        let mut value = 0;
        for _ in 0..32 {
            value = (value << 1) | (if bit_reader.read_bit().unwrap_or(false) { 1 } else { 0 });
        }
        Self {
            low: 0,
            high: 0xFFFFFFFF,
            value,
            bit_reader,
        }
    }
    
    fn decode(&mut self, model: &ArithModel) -> u8 {
        let range = (self.high - self.low) as u64 + 1;
        let total = model.cum_freq[256] as u64;
        
        let offset = (self.value - self.low) as u64;
        let target = ((offset + 1) * total - 1) / range;
        
        let mut sym = 0;
        for i in 0..256 {
            if target >= model.cum_freq[i] as u64 && target < model.cum_freq[i+1] as u64 {
                sym = i;
                break;
            }
        }
        
        let sym_low = model.cum_freq[sym] as u64;
        let sym_high = model.cum_freq[sym + 1] as u64;
        
        self.high = self.low + ((range * sym_high) / total) as u32 - 1;
        self.low = self.low + ((range * sym_low) / total) as u32;
        
        loop {
            if (self.high & 0x80000000) == (self.low & 0x80000000) {
                self.low = (self.low << 1) & 0xFFFFFFFF;
                self.high = ((self.high << 1) | 1) & 0xFFFFFFFF;
                self.value = (self.value << 1) & 0xFFFFFFFF | (if self.bit_reader.read_bit().unwrap_or(false) { 1 } else { 0 });
            } else if (self.low & 0x40000000) != 0 && (self.high & 0x40000000) == 0 {
                self.low = (self.low << 1) & 0x7FFFFFFF;
                self.high = ((self.high << 1) | 0x80000001) & 0xFFFFFFFF;
                self.value = (self.value & 0x80000000) | ((self.value << 1) & 0x7FFFFFFF) | (if self.bit_reader.read_bit().unwrap_or(false) { 1 } else { 0 });
            } else {
                break;
            }
        }
        
        sym as u8
    }
}

/// Compress Arithmetic.
pub fn compress_arithmetic(input_path: &Path) -> Result<CompressionOutcome> {
    let input = fs::read(input_path).with_context(|| {
        format!("Unable to read input file for Arithmetic: {}", input_path.display())
    })?;
    
    let mut freqs = [0u32; 256];
    for &b in &input { freqs[b as usize] += 1; }
    
    let model = ArithModel::new(&freqs);
    
    let mut output = Vec::new();
    output.extend_from_slice(b"COMPRESSA_ARITH_V1");
    for &f in &freqs {
        output.extend_from_slice(&f.to_le_bytes());
    }
    
    let mut bit_writer = BitWriter::new();
    let mut encoder = ArithmeticEncoder::new(&mut bit_writer);
    
    for &byte in &input { encoder.encode(byte, &model); }
    encoder.finish();
    
    let (data, valid_bits) = bit_writer.finish();
    output.push(valid_bits);
    output.extend_from_slice(&data);
    
    let output_path = PathBuf::from(format!("{}.arith", input_path.to_string_lossy()));
    fs::write(&output_path, &output).with_context(|| {
        format!("Unable to create Arithmetic output: {}", output_path.display())
    })?;
    
    let metadata = create_metadata(input_path, "Educational Arithmetic Coding")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;
    
    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Educational Arithmetic Coding".to_string(),
        lossless: true,
        message: "Educational Arithmetic coding completed.".to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}

/// Decompress Arithmetic.
pub fn decompress_arithmetic(input_path: &Path) -> Result<(PathBuf, String, Option<CompressionMetadata>)> {
    let raw = fs::read(input_path).with_context(|| {
        format!("Unable to open Arithmetic file: {}", input_path.display())
    })?;
    
    let payload = raw.strip_prefix(b"COMPRESSA_ARITH_V1").context("Invalid Arithmetic file header.")?;
    if payload.len() < 1024 {
        anyhow::bail!("Invalid payload size");
    }
    
    let mut freqs = [0u32; 256];
    for i in 0..256 {
        let mut b = [0u8; 4];
        b.copy_from_slice(&payload[i*4..(i+1)*4]);
        freqs[i] = u32::from_le_bytes(b);
    }
    let total_symbols: u32 = freqs.iter().sum();
    
    let mut decoded = Vec::new();
    if total_symbols > 0 {
        let model = ArithModel::new(&freqs);
        let data_start = 1024 + 1; // +1 for valid_bits
        let data = if data_start <= payload.len() { &payload[data_start..] } else { &[] };
        
        let mut bit_reader = BitReader::new(data);
        let mut decoder = ArithmeticDecoder::new(&mut bit_reader);
        
        for _ in 0..total_symbols {
            decoded.push(decoder.decode(&model));
        }
    }
    
    let output_path = infer_restored_output_path(input_path);
    fs::write(&output_path, decoded).with_context(|| {
        format!("Unable to write restored file: {}", output_path.display())
    })?;
    
    let metadata_path = metadata_path_for(input_path);
    let metadata = read_metadata(&metadata_path);
    
    Ok((
        output_path,
        "Educational Arithmetic Coding".to_string(),
        metadata,
    ))
}
