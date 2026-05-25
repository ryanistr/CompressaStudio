//! Native Jpeg module.
//!
//! Handles native jpeg operations.

use super::{
    create_metadata, metadata_path_for, unique_output_path, write_metadata, CompressionOutcome,
    CompressionRequest, QualityPreset,
};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const BLOCK_SIZE: usize = 8;
const BLOCK_AREA: usize = 64;

const ZIGZAG: [usize; 64] = [
    0,  1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63
];

const LUMA_QUANT: [u32; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61,
    12, 12, 14, 19, 26, 58, 60, 55,
    14, 13, 16, 24, 40, 57, 69, 56,
    14, 17, 22, 29, 51, 87, 80, 62,
    18, 22, 37, 56, 68, 109, 103, 77,
    24, 35, 55, 64, 81, 104, 113, 92,
    49, 64, 78, 87, 103, 121, 120, 101,
    72, 92, 95, 98, 112, 100, 103, 99
];

const CHROMA_QUANT: [u32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99,
    18, 21, 26, 66, 99, 99, 99, 99,
    24, 26, 56, 99, 99, 99, 99, 99,
    47, 66, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99
];

const STD_DC_LUMA_BITS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
const STD_DC_LUMA_VALS: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

const STD_DC_CHROMA_BITS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
const STD_DC_CHROMA_VALS: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

const STD_AC_LUMA_BITS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 125];
const STD_AC_LUMA_VALS: [u8; 162] = [
    0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07,
    0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0,
    0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
    0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
    0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
    0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5,
    0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2,
    0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
    0xf9, 0xfa
];

const STD_AC_CHROMA_BITS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119];
const STD_AC_CHROMA_VALS: [u8; 162] = [
    0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71,
    0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0,
    0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26,
    0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
    0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
    0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5,
    0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3,
    0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
    0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
    0xf9, 0xfa
];

struct BitWriter<'a> {
    out: &'a mut Vec<u8>,
    buffer: u32,
    bits: u8,
}

impl<'a> BitWriter<'a> {
    fn write_bits(&mut self, val: u32, num_bits: u8) {
        if num_bits == 0 { return; }
        for i in (0..num_bits).rev() {
            let bit = (val >> i) & 1;
            self.buffer = (self.buffer << 1) | bit;
            self.bits += 1;
            if self.bits == 8 {
                let byte = self.buffer as u8;
                self.out.push(byte);
                if byte == 0xFF {
                    self.out.push(0x00);
                }
                self.buffer = 0;
                self.bits = 0;
            }
        }
    }

    fn flush(&mut self) {
        if self.bits > 0 {
            let padding = 8 - self.bits;
            self.write_bits((1 << padding) - 1, padding);
        }
    }
}

fn apply_quality(base_table: &[u32; 64], quality: u32) -> [u32; 64] {
    let mut out = [0; 64];
    let q = if quality == 0 { 1 } else if quality > 100 { 100 } else { quality };
    let scale = if q < 50 { 5000 / q } else { 200 - 2 * q };
    for i in 0..64 {
        let mut val = (base_table[i] * scale + 50) / 100;
        if val < 1 { val = 1; }
        if val > 255 { val = 255; }
        out[i] = val;
    }
    out
}

fn build_huffman_table(bits: &[u8; 16], vals: &[u8]) -> HashMap<u8, (u32, u8)> {
    let mut map = HashMap::new();
    let mut code = 0u32;
    let mut val_idx = 0;
    for len in 1..=16 {
        for _ in 0..bits[len - 1] {
            map.insert(vals[val_idx], (code, len as u8));
            val_idx += 1;
            code += 1;
        }
        code <<= 1;
    }
    map
}

fn value_to_bits(val: i32) -> (u8, u32) {
    if val == 0 {
        (0, 0)
    } else {
        let mut size = 0;
        let mut temp = val.abs();
        while temp > 0 {
            size += 1;
            temp >>= 1;
        }
        let bits = if val > 0 { val as u32 } else { (val + (1 << size) - 1) as u32 };
        (size, bits)
    }
}

fn forward_dct_quantize(block: &[f32; 64], quant_table: &[u32; 64], cos_table: &[[f32; 8]; 8]) -> [i32; 64] {
    let mut out = [0; BLOCK_AREA];
    for rank in 0..BLOCK_AREA {
        let idx = ZIGZAG[rank];
        let v = idx / BLOCK_SIZE;
        let u = idx % BLOCK_SIZE;
        
        let mut sum = 0.0;
        for y in 0..BLOCK_SIZE {
            for x in 0..BLOCK_SIZE {
                let cos_x = cos_table[x][u];
                let cos_y = cos_table[y][v];
                sum += block[y * BLOCK_SIZE + x] * cos_x * cos_y;
            }
        }
        let cu = if u == 0 { std::f32::consts::FRAC_1_SQRT_2 } else { 1.0 };
        let cv = if v == 0 { std::f32::consts::FRAC_1_SQRT_2 } else { 1.0 };
        let coeff = 0.25 * cu * cv * sum;
        
        let q = quant_table[idx] as f32;
        out[rank] = (coeff / q).round() as i32;
    }
    out
}

fn encode_block(
    block: &[i32; 64],
    dc_pred: &mut i32,
    dc_huff: &HashMap<u8, (u32, u8)>,
    ac_huff: &HashMap<u8, (u32, u8)>,
    bit_writer: &mut BitWriter
) {
    let diff = (block[0] - *dc_pred).clamp(-2047, 2047);
    *dc_pred = block[0];
    let (dc_size, dc_bits) = value_to_bits(diff);
    let &(huff_code, huff_len) = dc_huff.get(&dc_size).unwrap();
    bit_writer.write_bits(huff_code, huff_len);
    bit_writer.write_bits(dc_bits, dc_size);

    let mut run = 0;
    for i in 1..64 {
        let coeff = block[i].clamp(-1023, 1023);
        if coeff == 0 {
            run += 1;
        } else {
            while run > 15 {
                let &(huff_code, huff_len) = ac_huff.get(&0xF0).unwrap();
                bit_writer.write_bits(huff_code, huff_len);
                run -= 16;
            }
            let (ac_size, ac_bits) = value_to_bits(coeff);
            let sym = (run << 4) | ac_size;
            let &(huff_code, huff_len) = ac_huff.get(&sym).unwrap();
            bit_writer.write_bits(huff_code, huff_len);
            bit_writer.write_bits(ac_bits, ac_size);
            run = 0;
        }
    }
    if run > 0 {
        let &(huff_code, huff_len) = ac_huff.get(&0x00).unwrap();
        bit_writer.write_bits(huff_code, huff_len);
    }
}

fn write_marker(out: &mut Vec<u8>, marker: u16) {
    out.push((marker >> 8) as u8);
    out.push((marker & 0xFF) as u8);
}

fn write_segment_len(out: &mut Vec<u8>, len: u16) {
    out.push((len >> 8) as u8);
    out.push((len & 0xFF) as u8);
}

fn write_dht(out: &mut Vec<u8>, class_id: u8, bits: &[u8; 16], vals: &[u8]) {
    write_marker(out, 0xFFC4);
    let len = 2 + 1 + 16 + vals.len() as u16;
    write_segment_len(out, len);
    out.push(class_id);
    out.extend_from_slice(bits);
    out.extend_from_slice(vals);
}

/// Compress Native Jpeg.
pub fn compress_native_jpeg(
    input_path: &Path,
    request: &CompressionRequest,
) -> Result<CompressionOutcome> {
    let img = image::open(input_path)
        .context("Failed to open image for native JPEG compression")?
        .into_rgb8();
    let (width, height) = img.dimensions();

    let quality = match request.quality_preset.unwrap_or(QualityPreset::Balanced) {
        QualityPreset::HighQuality => 90,
        QualityPreset::Balanced => 75,
        QualityPreset::SmallSize => 40,
    };

    let luma_q = apply_quality(&LUMA_QUANT, quality);
    let chroma_q = apply_quality(&CHROMA_QUANT, quality);

    let dc_luma_huff = build_huffman_table(&STD_DC_LUMA_BITS, &STD_DC_LUMA_VALS);
    let dc_chroma_huff = build_huffman_table(&STD_DC_CHROMA_BITS, &STD_DC_CHROMA_VALS);
    let ac_luma_huff = build_huffman_table(&STD_AC_LUMA_BITS, &STD_AC_LUMA_VALS);
    let ac_chroma_huff = build_huffman_table(&STD_AC_CHROMA_BITS, &STD_AC_CHROMA_VALS);

    let mut cos_table = [[0.0f32; 8]; 8];
    for x in 0..8 {
        for u in 0..8 {
            cos_table[x][u] = ((2.0 * x as f32 + 1.0) * u as f32 * std::f32::consts::PI / 16.0).cos();
        }
    }

    let mut out = Vec::new();

    // SOI
    write_marker(&mut out, 0xFFD8);

    // APP0
    write_marker(&mut out, 0xFFE0);
    write_segment_len(&mut out, 16);
    out.extend_from_slice(b"JFIF\0");
    out.push(1); // ver major
    out.push(1); // ver minor
    out.push(0); // units
    out.push(0); out.push(1); // X density
    out.push(0); out.push(1); // Y density
    out.push(0); // thumb w
    out.push(0); // thumb h

    // DQT Luma
    write_marker(&mut out, 0xFFDB);
    write_segment_len(&mut out, 2 + 1 + 64);
    out.push(0x00);
    for i in 0..64 { out.push(luma_q[ZIGZAG[i]] as u8); }

    // DQT Chroma
    write_marker(&mut out, 0xFFDB);
    write_segment_len(&mut out, 2 + 1 + 64);
    out.push(0x01);
    for i in 0..64 { out.push(chroma_q[ZIGZAG[i]] as u8); }

    // SOF0
    write_marker(&mut out, 0xFFC0);
    write_segment_len(&mut out, 8 + 3 * 3);
    out.push(8); // precision
    out.push((height >> 8) as u8); out.push((height & 0xFF) as u8);
    out.push((width >> 8) as u8); out.push((width & 0xFF) as u8);
    out.push(3); // components
    out.push(1); out.push(0x11); out.push(0); // Y: ID 1, 1x1, DQT 0
    out.push(2); out.push(0x11); out.push(1); // Cb: ID 2, 1x1, DQT 1
    out.push(3); out.push(0x11); out.push(1); // Cr: ID 3, 1x1, DQT 1

    // DHT DC Luma
    write_dht(&mut out, 0x00, &STD_DC_LUMA_BITS, &STD_DC_LUMA_VALS);
    // DHT DC Chroma
    write_dht(&mut out, 0x01, &STD_DC_CHROMA_BITS, &STD_DC_CHROMA_VALS);
    // DHT AC Luma
    write_dht(&mut out, 0x10, &STD_AC_LUMA_BITS, &STD_AC_LUMA_VALS);
    // DHT AC Chroma
    write_dht(&mut out, 0x11, &STD_AC_CHROMA_BITS, &STD_AC_CHROMA_VALS);

    // SOS
    write_marker(&mut out, 0xFFDA);
    write_segment_len(&mut out, 6 + 2 * 3);
    out.push(3);
    out.push(1); out.push(0x00); // Y
    out.push(2); out.push(0x11); // Cb
    out.push(3); out.push(0x11); // Cr
    out.push(0); // Ss
    out.push(63); // Se
    out.push(0x00); // Ah, Al

    // Data
    let mut bit_writer = BitWriter { out: &mut out, buffer: 0, bits: 0 };
    let mut dc_y = 0;
    let mut dc_cb = 0;
    let mut dc_cr = 0;

    let pad_width = (width + 7) / 8 * 8;
    let pad_height = (height + 7) / 8 * 8;

    for by in 0..(pad_height / 8) {
        for bx in 0..(pad_width / 8) {
            let mut block_y = [0.0; 64];
            let mut block_cb = [0.0; 64];
            let mut block_cr = [0.0; 64];

            for y in 0..8 {
                for x in 0..8 {
                    let px = (bx * 8 + x).min(width - 1);
                    let py = (by * 8 + y).min(height - 1);
                    let pixel = img.get_pixel(px, py);
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b = pixel[2] as f32;

                    block_y[y as usize * 8 + x as usize] = 0.299 * r + 0.587 * g + 0.114 * b - 128.0;
                    block_cb[y as usize * 8 + x as usize] = -0.1687 * r - 0.3313 * g + 0.5 * b;
                    block_cr[y as usize * 8 + x as usize] = 0.5 * r - 0.4187 * g - 0.0813 * b;
                }
            }

            let coeff_y = forward_dct_quantize(&block_y, &luma_q, &cos_table);
            let coeff_cb = forward_dct_quantize(&block_cb, &chroma_q, &cos_table);
            let coeff_cr = forward_dct_quantize(&block_cr, &chroma_q, &cos_table);

            encode_block(&coeff_y, &mut dc_y, &dc_luma_huff, &ac_luma_huff, &mut bit_writer);
            encode_block(&coeff_cb, &mut dc_cb, &dc_chroma_huff, &ac_chroma_huff, &mut bit_writer);
            encode_block(&coeff_cr, &mut dc_cr, &dc_chroma_huff, &ac_chroma_huff, &mut bit_writer);
        }
    }
    bit_writer.flush();

    // EOI
    write_marker(&mut out, 0xFFD9);

    let output_path = unique_output_path(
        input_path.parent().unwrap_or(Path::new("")),
        input_path.file_stem().unwrap_or_default().to_str().unwrap_or_default(),
        "jpg"
    );

    fs::write(&output_path, &out).context("Failed to write Native JPEG file")?;

    let metadata = create_metadata(input_path, "Native Scratch JPEG Encoder")?;
    let metadata_path = metadata_path_for(&output_path);
    write_metadata(&metadata_path, &metadata)?;

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Native From-Scratch JPEG".to_string(),
        lossless: false,
        message: "Image successfully encoded using from-scratch native JPEG pipeline".to_string(),
        metadata_path: Some(metadata_path.to_string_lossy().to_string()),
    })
}
