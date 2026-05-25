# Compressa Studio

Compressa Studio is a comprehensive desktop application built with Rust and Tauri for all-in-one file compression. It intelligently categorizes files (Images, Videos, PDFs, Generic Data) and routes them through specialized compression pipelines.

## Capabilities

The app supports four major compression modes:

### 1. Image Compression
When an image (PNG, JPG, etc.) is selected, the app scales and compresses it.
- **Features**: Quality presets (High, Balanced, Small Size) and custom resizing (10%-100%).
- **Native JPEG Encoder**: Enable the native Rust JPEG encoder from the settings to use our custom from-scratch implementation (featuring color space conversion, DCT, Quantization, and Huffman coding). Otherwise, standard encoders will automatically choose between JPEG and WebP based on transparency.

### 2. Video Compression
Uses `ffmpeg` natively to re-encode and compress video files.
- **Features**: Automatically checks for FFmpeg availability on your system. Shrinks video size using state-of-the-art presets.

### 3. PDF Compression
Leverages `ghostscript` to optimize and compress PDF files.
- **Features**: Three tailored presets depending on your intended use-case (Screen, Ebook, Print).

### 4. Generic Lossless Compression
When a non-media file (like `.txt`, `.docx`, `.bin`) is selected, the app switches into generic data compression mode.
- **Available Algorithms**:
  - **Zstd**: Industry standard fast and efficient dictionary/statistical coding.
  - **Run-Length Encoding (RLE)**: Lossless compression good for repetitive bytes.
  - **Shannon Coding**: Entropy coding based on exact probabilities.
  - **Shannon-Fano Coding**: Top-down sub-optimal tree generation.
  - **Huffman Coding**: Optimal bottom-up binary tree entropy coding.
  - **Lempel-Ziv 77 (LZ77)**: Sliding window dictionary encoding.
  - **Lempel-Ziv 78 (LZ78)**: Explicit phrase dictionary construction.
  - **Lempel-Ziv-Welch (LZW)**: Dynamic ASCII dictionary encoding.
  - **Arithmetic Coding**: State-of-the-art mathematically optimal entropy compression.

## How to Use

1. **Select a File**: Click the central file selection box (or click on any compression mode tile) to browse and select the file you want to compress.
2. **Configure Settings**: Based on the file type, adjust the settings panel on the right (e.g., Quality Preset, Algorithm Choice).
3. **Compress**: Click the "Run Compression" button. The app will process the file natively in Rust and display real-time statistics (e.g., Space Saved, Compression Ratio).
4. **Decompress**: For lossless files (like `.zst`, `.huff`, `.lz77`, etc.), selecting the compressed file automatically switches the app into Decompression mode. Just click "Decompress File" to restore the original perfectly.
