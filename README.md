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
When a non-media file (like `.txt`, `.docx`, `.pptx`, `.xlsx`, `.bin`) is selected, the app switches into generic data compression mode. This path is lossless, so decompression restores the exact original bytes.
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

## PPTX Compression

PowerPoint files (`.pptx`) are supported through **Generic Lossless Compression**. A `.pptx` file is an Office Open XML package, which means it is already a ZIP-based container holding slide XML, media, fonts, themes, and relationships. Because of that, it is not handled like a PDF and it should not be recompressed through the PDF pipeline.

Recommended method for `.pptx`:

1. Use **Auto Detect** or **Generic** file selection.
2. Keep **Algorithm Choice** set to **Zstd dictionary/statistical coding**.
3. Use the default Zstd level `6` for the best practical balance of speed and size.
4. Increase Zstd toward `15-19` only when maximum size reduction matters more than runtime.

Expected result: many `.pptx` files will shrink only a little, and some may grow slightly, because the file is already compressed internally. Zstd is still the most practical choice because it can sometimes find repeated XML/package structure across the ZIP payload while staying fast. Arithmetic Coding is supported for `.pptx`, but it is mainly useful for demonstrating the entropy-coding novelty rather than producing the smallest real-world `.pptx` output.

## Arithmetic Coding Novelty

Arithmetic Coding is the app's main educational novelty. Instead of assigning each byte a separate prefix code like Huffman, it narrows a numeric interval according to symbol probabilities and represents the whole message as bits from that interval. This can get closer to the Shannon entropy limit because it is not forced to spend at least one full bit per symbol.

Accepted inputs:

- Normal generic/document/data files: `.txt`, `.csv`, `.json`, `.xml`, `.log`, `.doc`, `.docx`, `.ppt`, `.pptx`, `.xls`, `.xlsx`, `.bin`, `.dat`.
- Any other unknown regular file that the app routes to Generic Lossless Compression.
- Existing image, video, and PDF files are normally routed to their specialized pipelines. Use those specialized modes for practical compression. Only override them to Generic if you intentionally want byte-exact research compression instead of image/video/PDF optimization.

How to use Arithmetic Coding:

1. Select a generic/document/data file.
2. In **Compression Mode**, set **Algorithm Choice** to **Arithmetic Coding (Optimal Entropy)**.
3. Click **Run Compression**.
4. The output is written next to the input as `original_filename.ext.arith`.
5. To restore it, select the `.arith` file and click **Decompress**. The app reads the arithmetic payload and sidecar metadata, then verifies the restored SHA-256 when metadata is available.

Best real-world fit:

- Good demonstration cases: text-heavy logs, CSV exports, JSON records, XML, simple binary streams, and datasets with very uneven byte frequencies.
- Poor practical cases: already compressed files such as `.pptx`, `.docx`, `.xlsx`, `.zip`, `.jpg`, `.png`, `.mp4`, and many PDFs. These often have little remaining entropy redundancy and can expand because the app stores a 256-symbol frequency table plus format metadata.
- Real-world analogy: arithmetic coding is used inside mature codecs and model-based compressors when the probability model is strong. In this app, the implementation is a static byte-frequency model, so it is best used to study entropy coding behavior and verify exact round-trip decompression.

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
