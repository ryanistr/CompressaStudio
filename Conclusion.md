# Data Compression, Image, and Video Encoding: Comprehensive Synthesis

## 1. Information Theory & Fundamentals
* **Information Definition:** The measure of uncertainty reduced when a message is received. Highly uncertain events contain maximum information.
* **Shannon Entropy:** Calculates the average amount of information produced by a stochastic source of data. Formula: `H(x) = -Σ P(x_i) log2 P(x_i)` (measured in bits).
* **Lossless Compression:** Reduces file size without any loss of data. The decompressed data is a bit-for-bit exact copy of the original. (Examples: PNG, GIF, ZIP).
* **Lossy Compression:** Eliminates redundant or imperceptible data to achieve significantly higher compression ratios. Exact original data cannot be reconstructed. (Examples: JPEG, MPEG, MP3).
* **Redundancy Reduction:** Core to compression; exploiting spatial (neighboring similarities) and temporal (sequential similarities) overlaps.

## 2. Statistical & Variable-Length Encoding
* **Uniquely Decodable Code:** A sequence of bits that can be unambiguously decoded into its original symbols without multiple interpretations.
* **Prefix Property:** No codeword is a prefix of another codeword. Facilitates instantaneous decoding (e.g., Huffman coding).
* **Shannon Coding:** Sub-optimal coding sorting symbols by probability descending. Codeword length `l_i = |-log2(p_i)|`.
* **Shannon-Fano Coding:** Sub-optimal, top-down approach. Sorts symbols by frequency, recursively dividing them into two groups with probabilities as close as possible, assigning '0' to one and '1' to the other.
* **Huffman Coding:** Optimal, bottom-up approach building a binary tree to ensure the most frequent symbols have the shortest codewords.

## 3. Dictionary-Based Encoding
Instead of encoding single symbols, sequences of symbols are replaced by a single token or index.
* **LZ77:** Uses a sliding window (Search buffer and Look-ahead buffer). Outputs a triplet: `<offset, length, next_character>`.
* **LZ78:** Builds a dictionary explicitly, adding phrases one symbol at a time. Outputs a tuple: `<dictionary_index, next_character>`.
* **LZW (Lempel-Ziv-Welch):** An improvement over LZ78. Pre-initializes the dictionary with all single characters (e.g., ASCII 0-255). Outputs only the dictionary index, allowing the decoder to build the exact same dictionary dynamically. Used in GIF and UNIX compress.

## 4. Image Compression (JPEG Pipeline)
Image compression targets spatial redundancy.
* **Color Space Conversion:** Converts RGB to YCbCr (or YUV). The human eye is more sensitive to luminance (Y) than chrominance (Cb, Cr). Chrominance components can be downsampled (e.g., 4:2:0 format) to save bandwidth.
* **Block Preparation:** Image is split into 8x8 macroblocks.
* **Discrete Cosine Transform (DCT):** Transforms spatial pixel data into the frequency domain. Low frequencies (coarse details) map to the top-left of the 8x8 matrix; high frequencies (fine details, noise) map to the bottom-right.
* **Quantization:** The primary source of lossy compression. Divides DCT coefficients by a quantization matrix and rounds them to the nearest integer. High-frequency coefficients are driven to zero.
* **Zig-Zag Scanning:** Reorders the 2D 8x8 matrix into a 1D vector (1x64), grouping the low-frequency non-zero coefficients at the beginning and long runs of zeros at the end.
* **DPCM & RLE:** Differential Pulse Code Modulation encodes the DC component (first value) using the difference from the previous block's DC. Run-Length Encoding compresses the AC components by counting consecutive zeros.
* **Entropy Coding:** The final sequence is compressed using Huffman or Arithmetic coding.
* **Operating Modes:** Sequential (top-to-bottom), Lossless (predictive), Progressive (successive approximation/spectral selection), Hierarchical (multiple resolutions).

## 5. Video Compression (MPEG & Advanced Codecs)
Video compression expands upon image compression by adding temporal redundancy reduction.
* **Spatial Compression (Intra-frame):** Compression within a single frame using DCT and quantization (similar to JPEG).
* **Temporal Compression (Inter-frame):** Compression between consecutive frames using motion estimation.
* **MPEG Frame Types:**
    * **I-Frames (Intra-coded):** Encoded independently. Lowest compression, serves as the reference point.
    * **P-Frames (Predictive):** Encoded with forward prediction using the previous I-Frame or P-Frame.
    * **B-Frames (Bi-directional):** Encoded using both forward and backward prediction from past and future I/P-Frames. Highest compression.
* **Group of Pictures (GOP):** The ordering structure of I, P, and B frames (e.g., `IBBBPBBBP`). A GOP must begin with an I-Frame.
* **Motion Vectors & Prediction Error:** Instead of transmitting full pixels for moving objects, the codec calculates a motion vector (displacement in X and Y). The remaining difference between the predicted block and the actual block is the prediction error, which is then DCT compressed.
* **Modern Standards:**
    * **H.264 / AVC (MPEG-4 Part 10):** Industry standard for streaming and recording.
    * **H.265 / HEVC:** Improved compression efficiency, suitable for 4K.
    * **VP9:** Google-developed alternative to H.265, heavily used in YouTube.
    * **AV1:** Open-source, royalty-free codec with superior compression efficiency designed for high-resolution web video.
