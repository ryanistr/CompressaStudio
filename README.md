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
4. **Decompress**: Select a supported generic lossless output, then click **Decompress File**. The app restores the original byte stream when the selected file was produced by one of the supported lossless compressors.

## Decompression Workflow

Decompression is only available for generic lossless outputs created by Compressa Studio. Image, video, and PDF outputs are optimized media/document files and are not reversible through the Decompress button.

Supported decompression inputs:

- `.zst` - Zstandard dictionary/statistical compression.
- `.rle` - Educational Run-Length Encoding.
- `.shnc` - Educational Shannon Coding.
- `.sfc` - Educational Shannon-Fano Coding.
- `.huff` - Educational Huffman Coding.
- `.lz77` - Educational LZ77.
- `.lz78` - Educational LZ78.
- `.lzw` - Educational LZW.
- `.arith` - Educational Arithmetic Coding.

How it works:

1. Choose a supported compressed file. The app detects the extension and enables the Decompress action.
2. Click **Decompress File**. The backend routes the file to the matching decoder.
3. The restored file is written next to the compressed file. If the original output name already exists, the app creates a numbered restored filename.
4. When a sidecar metadata file exists, for example `example.txt.zst.meta.json`, the app reads the original SHA-256 and verifies the restored output.
5. The statistics panel reports the output path, checksum result, elapsed time, and restored size.

If no metadata sidecar is present, decompression can still run for supported formats, but checksum verification will be reported as unavailable.
