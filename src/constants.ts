/**
 * @file constants.ts
 * @description Application-wide constants and configurations.
 */

import type { AppStatus, CompressionRequest, FileCategory } from './types'

export const STATUS_LABELS: Record<AppStatus, string> = {
  idle: 'Idle',
  compressing: 'Compressing',
  decompressing: 'Decompressing',
  success: 'Success',
  error: 'Error',
}

export const DEFAULT_REQUEST: CompressionRequest = {
  qualityPreset: 'balanced',
  pdfPreset: 'ebook',
  genericAlgorithm: 'zstd',
  resizePercent: 100,
  zstdLevel: 6,
}

export const FILE_CATEGORY_LABELS: Record<FileCategory, string> = {
  image: 'Image Compression',
  video: 'Video Compression',
  pdf: 'PDF Compression',
  generic: 'Generic Lossless Compression',
}

export const FILE_CATEGORIES: FileCategory[] = ['image', 'video', 'pdf', 'generic']

export interface AlgorithmExplanationData {
  summary: string
  points: string[]
}

export const ALGORITHM_EXPLANATIONS: Record<string, AlgorithmExplanationData> = {
  image: {
    summary:
      'Image compression in this app is lossy. It follows the Materi discussion on visual redundancy, transform coding, quantization, and quality trade-offs in JPEG/WebP-style image pipelines.',
    points: [
      'Redundancy in neighboring pixels is reduced because many image regions vary slowly.',
      'Lower quality presets keep fewer visually unimportant details, similar to quantization in DCT-based image compression.',
      'Optional resizing reduces pixel count directly, which lowers spatial data before encoding.',
      'Output remains an image file instead of wrapping the image inside a generic archive.',
    ],
  },
  video: {
    summary:
      'Video compression utilizes inter-frame and intra-frame redundancy. It drops invisible details and encodes motion rather than full frames.',
    points: [
      'Spatial redundancy (intra-frame) is reduced within single frames similar to image compression.',
      'Temporal redundancy (inter-frame) is exploited by tracking motion between frames.',
      'CRF (Constant Rate Factor) adjusts the quantization dynamically to maintain visual quality.',
      'Audio streams are also compressed using AAC.',
    ],
  },
  pdf: {
    summary:
      'PDF compression optimizes document structures, downsamples embedded images, and removes invisible metadata and unused objects.',
    points: [
      'Images inside the PDF are re-compressed and downsampled to screen or print resolutions.',
      'Fonts are subsetted to only include the characters actually used in the document.',
      'Redundant objects and unnecessary metadata are stripped from the file structure.',
    ],
  },
  generic: {
    summary:
      'Generic lossless compression uses zstd by default, finding repetitive byte patterns and applying entropy coding to reduce file size without losing any data.',
    points: [
      'Dictionary matching (LZ77-style) replaces repeated byte sequences with short references.',
      'Entropy coding (Huffman/FSE) encodes more frequent symbols with fewer bits.',
      'The exact original file is perfectly reconstructed during decompression.',
    ],
  },
  zstd: {
    summary:
      'Zstandard (zstd) is a fast lossless compression algorithm that combines dictionary-based L77 with Finite State Entropy (FSE) coding.',
    points: [
      'Provides excellent compression ratios comparable to Deflate but at much higher speeds.',
      'Uses a large search window for finding byte sequence repetitions.',
      'Employs FSE to encode literals and match lengths efficiently.',
    ],
  },
  rle: {
    summary:
      'Run-Length Encoding (RLE) is a simple lossless algorithm that replaces sequences of identical data values with a single value and a count.',
    points: [
      'Highly effective on data with long runs of identical bytes (e.g., simple graphics, blank space).',
      'Can sometimes expand the file size if the data contains very few or no runs.',
      'Serves as a fundamental building block in more complex compression schemes.',
    ],
  },
  shannon: {
    summary:
      'Shannon entropy measurement establishes the theoretical minimum number of bits needed to encode a symbol based on its probability.',
    points: [
      'Calculates the information content of each symbol: -log2(probability).',
      'Provides the lower bound for any lossless compression algorithm.',
      'Not a compression algorithm itself, but a theoretical limit.',
    ],
  },
  shannonFano: {
    summary:
      'Shannon-Fano coding is a top-down probability-based binary tree prefix code.',
    points: [
      'Sorts symbols by frequency and recursively divides them into two sets of roughly equal total probability.',
      'Assigns 0 and 1 to each set, creating a prefix-free code.',
      'Often slightly less optimal than Huffman coding but historically significant.',
    ],
  },
  huffman: {
    summary:
      'Huffman coding is a bottom-up optimal prefix-free variable-length coding method.',
    points: [
      'Builds a tree by iteratively combining the two least frequent symbols.',
      'Guarantees the most efficient prefix code for a given symbol probability distribution.',
      'Saves space by using shorter binary codes for more common symbols.',
    ],
  },
  lz77: {
    summary:
      'LZ77 uses a sliding window dictionary compression with back-references.',
    points: [
      'Instead of outputting repeated text, outputs a reference (distance, length) to previous occurrences.',
      'Maintains a "sliding window" of recently seen data.',
      'The foundation of algorithms like Deflate (used in ZIP, PNG) and zstd.',
    ],
  },
  lz78: {
    summary:
      'LZ78 is a dictionary-based compression algorithm that builds phrases incrementally.',
    points: [
      'Builds an explicit dictionary of phrases seen so far.',
      'Outputs (dictionary index, next character) pairs.',
      'Less constrained by a sliding window, but the dictionary can grow large.',
    ],
  },
  lzw: {
    summary:
      'LZW (Lempel-Ziv-Welch) extends LZ78 without needing explicit phrase output.',
    points: [
      'Pre-initializes the dictionary with all single characters.',
      'Outputs only dictionary indices; the decoder builds the exact same dictionary on the fly.',
      'Famously used in GIF images and the UNIX compress utility.',
    ],
  },
  arithmetic: {
    summary:
      'Arithmetic coding encodes an entire message as a single fractional number in the range [0,1).',
    points: [
      'Subdivides the [0,1) interval according to symbol probabilities.',
      'Achieves compression rates closer to Shannon entropy than Huffman coding because it avoids the 1-bit-per-symbol minimum constraint.',
      'Computationally more intensive than Huffman coding.',
    ],
  },
}
