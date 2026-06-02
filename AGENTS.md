# Compressa Studio — Full Project Documentation

> **Purpose**: This document serves as persistent AI context for future sessions. It captures the complete architecture, module structure, data flow, type system, and build/run procedures so that no re-discovery or re-analysis is needed.
>
> **Last updated**: 2026-06-02

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Technology Stack](#2-technology-stack)
3. [Repository Layout](#3-repository-layout)
4. [Build & Run](#4-build--run)
5. [Architecture Overview](#5-architecture-overview)
6. [Frontend (React + TypeScript)](#6-frontend-react--typescript)
   - 6.1 [Entry Points](#61-entry-points)
   - 6.2 [Type System (`types.ts`)](#62-type-system-typests)
   - 6.3 [Constants (`constants.ts`)](#63-constants-constantsts)
   - 6.4 [Utilities (`utils.ts`)](#64-utilities-utilsts)
   - 6.5 [Tauri Bridge (`tauriRuntime.ts`)](#65-tauri-bridge-tauriruntimets)
   - 6.6 [Root Component (`App.tsx`)](#66-root-component-apptsx)
   - 6.7 [Component Tree](#67-component-tree)
   - 6.8 [Component Reference](#68-component-reference)
   - 6.9 [CSS Design System (`styles.css`)](#69-css-design-system-stylescss)
7. [Backend (Rust + Tauri)](#7-backend-rust--tauri)
   - 7.1 [Crate Configuration](#71-crate-configuration)
   - 7.2 [Entry Point & Module Declarations](#72-entry-point--module-declarations)
   - 7.3 [IPC Commands (`commands.rs`)](#73-ipc-commands-commandsrs)
   - 7.4 [File Detection (`file_detection.rs`)](#74-file-detection-file_detectionrs)
   - 7.5 [File Info (`file_info.rs`)](#75-file-info-file_infors)
   - 7.6 [Checksum (`checksum.rs`)](#76-checksum-checksumrs)
   - 7.7 [Tool Detection (`tool_detection.rs`)](#77-tool-detection-tool_detectionrs)
   - 7.8 [Compression Module](#78-compression-module)
8. [IPC Contract (Frontend ↔ Backend)](#8-ipc-contract-frontend--backend)
9. [Compression Algorithms Reference](#9-compression-algorithms-reference)
10. [File Format & Extension Registry](#10-file-format--extension-registry)
11. [External Dependencies](#11-external-dependencies)
12. [Supplementary Files](#12-supplementary-files)
13. [Known Constraints & Design Decisions](#13-known-constraints--design-decisions)

---

## 1. Project Overview

**Compressa Studio** is a desktop application for all-in-one file compression. It classifies files into four categories (Image, Video, PDF, Generic) and routes them through specialized compression pipelines — all implemented natively in Rust with a React/TypeScript frontend rendered inside a Tauri webview.

**Key features**:
- **Image compression**: Lossy JPEG/WebP encoding with quality presets, resizing, and an optional from-scratch native Rust JPEG encoder (DCT, quantization, Huffman coding).
- **Video compression**: FFmpeg CRF re-encode (H.264/AAC).
- **PDF compression**: Ghostscript-based optimisation with Screen/Ebook/Print presets.
- **Generic lossless compression**: Nine algorithms — Zstd, RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, Arithmetic Coding — all implemented from scratch in Rust (except Zstd which uses the `zstd` crate).
- **Decompression**: Round-trip restoration for all generic lossless outputs with SHA-256 integrity verification via sidecar metadata files.

**Educational context**: This project is tied to a data compression course ("Kompresi Data"). The `Materi/` directory contains lecture slides (`.pptx`, `.pdf`). The `Conclusion.md` file is a course synthesis. The educational algorithms (Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, Arithmetic, RLE, native JPEG) are implemented from scratch for learning purposes.

---

## 2. Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Desktop Runtime | Tauri | 2.11.2 |
| Backend Language | Rust | Edition 2021, MSRV 1.77.2 |
| Frontend Framework | React | 19.2.6 |
| Frontend Language | TypeScript | ~6.0.2 |
| Bundler | Vite | 8.0.12 |
| Styling | Vanilla CSS (glassmorphism design system) | — |
| Package Manager | npm | — |
| Linting | ESLint + typescript-eslint + react-hooks + react-refresh | — |
| External Tools | FFmpeg (video), Ghostscript (PDF) | System-installed |

---

## 3. Repository Layout

```
kompresidata/
├── index.html                  # Vite HTML entry point
├── package.json                # npm config (name: compressa-studio)
├── vite.config.ts              # Vite + React plugin
├── tsconfig.json               # TS project references
├── tsconfig.app.json           # App TS config
├── tsconfig.node.json          # Node TS config
├── eslint.config.js            # ESLint flat config
├── run.sh                      # Dev launcher script
├── extract.py                  # Utility: extract text from Materi/ slides → Conclusion.md
├── README.md                   # User-facing documentation
├── Conclusion.md               # Course material synthesis
├── DOCUMENTATION.md            # ← This file
├── .gitignore
│
├── public/
│   ├── favicon.svg             # App icon (SVG)
│   └── icons.svg               # SVG sprite sheet
│
├── src/                        # Frontend source (React + TypeScript)
│   ├── main.tsx                # React DOM entry
│   ├── App.tsx                 # Root component (all state + orchestration)
│   ├── types.ts                # Shared type definitions
│   ├── constants.ts            # App-wide constants & algorithm explanations
│   ├── utils.ts                # Formatting utilities
│   ├── tauriRuntime.ts         # Tauri detection & IPC wrapper
│   ├── styles.css              # Complete CSS design system (~1900 lines)
│   ├── assets/
│   │   └── hero.png            # Hero image asset
│   └── components/
│       ├── ActionPanel.tsx      # Compress/Decompress/Clear buttons
│       ├── AlgorithmExplanation.tsx  # Educational algorithm info card
│       ├── CompressionSettings.tsx   # Category-adaptive settings panel
│       ├── CustomSelect.tsx     # Styled dropdown with portal + keyboard nav
│       ├── FilePicker.tsx       # File metadata display + category override
│       ├── FileTypeBadge.tsx    # Category badge pill
│       ├── GlassCard.tsx        # Glassmorphism container primitive
│       ├── LogDropdown.tsx      # Collapsible event log (topbar dropdown)
│       ├── StatsPanel.tsx       # Compression result statistics
│       ├── Stepper.tsx          # Numeric stepper input (resize %, zstd level)
│       └── UploadModal.tsx      # File selection modal (drag-drop + native dialog)
│
├── src-tauri/                  # Backend source (Rust + Tauri)
│   ├── Cargo.toml              # Rust crate config
│   ├── Cargo.lock
│   ├── build.rs                # Tauri build script
│   ├── tauri.conf.json         # Tauri app config (window, bundle, CSP)
│   ├── capabilities/
│   │   └── default.json        # Tauri permissions (core:default, dialog:default)
│   ├── icons/                  # App icons (PNG, ICO, ICNS)
│   └── src/
│       ├── main.rs             # Binary entry point
│       ├── lib.rs              # Crate root, Tauri builder, module declarations
│       ├── commands.rs         # IPC command handlers (bridge layer)
│       ├── file_detection.rs   # File category detection (extension + magic bytes)
│       ├── file_info.rs        # FileInfo struct assembly
│       ├── checksum.rs         # SHA-256 streaming hasher
│       ├── tool_detection.rs   # FFmpeg/Ghostscript availability probe
│       └── compression/
│           ├── mod.rs               # Compression router + OperationResult + metadata
│           ├── generic_compressor.rs # Zstd compress/decompress + algorithm dispatch
│           ├── image_compressor.rs   # Image encode (JPEG/WebP via image crate)
│           ├── native_jpeg.rs        # From-scratch JPEG encoder
│           ├── native_rle.rs         # From-scratch RLE encoder/decoder
│           ├── native_entropy.rs     # Shannon, Shannon-Fano, Huffman, Arithmetic
│           ├── native_dict.rs        # LZ77, LZ78, LZW
│           ├── video_compressor.rs   # FFmpeg CLI wrapper
│           └── pdf_compressor.rs     # Ghostscript CLI wrapper
│
├── Materi/                     # Lecture materials (gitignored, binary assets)
│   ├── *.pptx                  # Lecture slides
│   └── *.pdf                   # Lecture PDFs
│
└── dist/                       # Vite build output (gitignored)
```

---

## 4. Build & Run

### Prerequisites

- **Rust toolchain** (rustup, cargo) — edition 2021, MSRV 1.77.2
- **Node.js + npm**
- **Tauri CLI**: installed as devDependency (`@tauri-apps/cli`)
- **Optional**: FFmpeg (for video compression), Ghostscript (for PDF compression)

### Development

```bash
# Quick start (installs npm deps if missing, then runs Tauri dev)
./run.sh

# Or manually:
npm install
npm run tauri -- dev
```

Tauri dev mode starts Vite on `http://localhost:5173` and opens the native window with hot-reload.

### Production Build

```bash
npm run build          # TypeScript check + Vite production build
npm run tauri -- build  # Full Tauri bundle (includes Rust compilation)
```

### Linting

```bash
npm run lint
```

### Tauri Configuration

- **Window**: 1320×860, resizable, not fullscreen
- **Identifier**: `com.compressa.studio`
- **Capabilities**: `core:default`, `dialog:default`
- **Frontend dist**: `../dist` (relative to `src-tauri/`)
- **Dev URL**: `http://localhost:5173`

---

## 5. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Tauri Desktop Window                     │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              React Frontend (Vite + TypeScript)           │  │
│  │                                                           │  │
│  │  App.tsx ─── state management ─── component tree          │  │
│  │      │                                                    │  │
│  │      │  invokeTauri('command', args)                      │  │
│  │      ▼                                                    │  │
│  │  ─── IPC Bridge (tauriRuntime.ts) ───────────────────     │  │
│  └──────────────────────┬────────────────────────────────────┘  │
│                         │ Tauri invoke() / JSON serialization   │
│  ┌──────────────────────▼────────────────────────────────────┐  │
│  │              Rust Backend (src-tauri/)                     │  │
│  │                                                           │  │
│  │  commands.rs (IPC handlers)                               │  │
│  │      ├── get_file_info ──► file_info + file_detection     │  │
│  │      │                     + checksum + tool_detection     │  │
│  │      ├── compress_file ──► compression/mod.rs router      │  │
│  │      │   ├── Image ──► image_compressor / native_jpeg     │  │
│  │      │   ├── Video ──► video_compressor (ffmpeg CLI)      │  │
│  │      │   ├── PDF   ──► pdf_compressor (gs CLI)            │  │
│  │      │   └── Generic ► generic_compressor                 │  │
│  │      │       ├── Zstd (zstd crate)                        │  │
│  │      │       ├── RLE ──► native_rle                       │  │
│  │      │       ├── Shannon/ShannonFano/Huffman/Arith         │  │
│  │      │       │   └──► native_entropy                      │  │
│  │      │       └── LZ77/LZ78/LZW ──► native_dict           │  │
│  │      └── decompress_file ──► extension-based dispatch     │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Data flow**: User selects file → frontend calls `get_file_info` → backend inspects file (category, SHA-256, tool availability) → frontend displays metadata and renders category-specific settings → user configures and clicks Compress → frontend calls `compress_file` with path + request → backend routes to correct pipeline, produces output file + optional `.meta.json` sidecar → returns `OperationResult` → frontend displays stats. Decompression follows the same pattern in reverse.

---

## 6. Frontend (React + TypeScript)

### 6.1 Entry Points

**`index.html`**: Standard Vite HTML shell. Mounts `<div id="root">` and loads `src/main.tsx`.

**`src/main.tsx`**: Creates React root in StrictMode, renders `<App />`, imports `styles.css`.

### 6.2 Type System (`types.ts`)

All shared types between frontend modules. These mirror the Rust backend serialization.

| Type | Kind | Values / Fields |
|------|------|-----------------|
| `AppStatus` | Union | `'idle' \| 'compressing' \| 'decompressing' \| 'success' \| 'error'` |
| `FileCategory` | Union | `'image' \| 'video' \| 'pdf' \| 'generic'` |
| `QualityPreset` | Union | `'highQuality' \| 'balanced' \| 'smallSize'` |
| `PdfPreset` | Union | `'screen' \| 'ebook' \| 'print'` |
| `GenericAlgorithm` | Union | `'zstd' \| 'rle' \| 'shannon' \| 'shannonFano' \| 'huffman' \| 'lz77' \| 'lz78' \| 'lzw' \| 'arithmetic'` |
| `LogLevel` | Union | `'info' \| 'success' \| 'error'` |
| `ToolStatus` | Interface | `name, available, command, detail` |
| `ToolAvailability` | Interface | `ffmpeg: ToolStatus, ghostscript: ToolStatus` |
| `FileInfo` | Interface | `path, fileName, extension, category, isDecompressible, sizeBytes, sha256, tools` |
| `CompressionRequest` | Interface | `qualityPreset?, pdfPreset?, genericAlgorithm?, resizePercent?, zstdLevel?, useNativeJpeg?, overrideCategory?` |
| `OperationResult` | Interface | `operation, category, method, lossless, inputPath, outputPath, originalSize, outputSize, savedSize, compressionRatio, spaceSaved, elapsedMs, sourceSha256, outputSha256, expectedSha256?, integrityMatch?, metadataPath?, message` |
| `StatsSnapshot` | Interface | Partial subset of `OperationResult` for UI display |
| `LogEntry` | Interface | `id, timestamp, level, message` |

### 6.3 Constants (`constants.ts`)

| Constant | Purpose |
|----------|---------|
| `STATUS_LABELS` | Display labels per `AppStatus` |
| `DEFAULT_REQUEST` | Default `CompressionRequest` (balanced, ebook, zstd, 100%, level 6) |
| `FILE_CATEGORY_LABELS` | Human-readable labels per `FileCategory` |
| `FILE_CATEGORIES` | Ordered array `['image', 'video', 'pdf', 'generic']` |
| `BROWSER_TOOL_AVAILABILITY` | Fallback `ToolAvailability` when not in Tauri (both unavailable) |
| `ALGORITHM_EXPLANATIONS` | Record of 12 entries (image, video, pdf, generic, zstd, rle, shannon, shannonFano, huffman, lz77, lz78, lzw, arithmetic) — each with `summary: string` and `points: string[]` |

### 6.4 Utilities (`utils.ts`)

| Function | Signature | Purpose |
|----------|-----------|---------|
| `formatSize` | `(bytes: number) → string` | Human-readable size (e.g., "1.5 MB") |
| `formatDuration` | `(ms: number) → string` | Duration display ("450ms" or "1.20s") |
| `timestampNow` | `() → string` | Current time as locale string |
| `toMessage` | `(error: unknown) → string` | Converts any error to string |

### 6.5 Tauri Bridge (`tauriRuntime.ts`)

| Function | Purpose |
|----------|---------|
| `isTauriRuntime()` | Returns `true` if `window.__TAURI_INTERNALS__` exists |
| `invokeTauri<T>(command, args?)` | Type-safe wrapper around `@tauri-apps/api/core invoke()`. Throws if not in Tauri runtime. |

### 6.6 Root Component (`App.tsx`)

**State variables**:

| State | Type | Purpose |
|-------|------|---------|
| `selectedPath` | `string \| null` | Filesystem path of current file |
| `selectedFile` | `FileInfo \| null` | Backend-inspected metadata |
| `uploadModalMode` | `FileCategory \| 'auto' \| null` | Controls UploadModal visibility/mode |
| `request` | `CompressionRequest` | User-configurable compression settings |
| `status` | `AppStatus` | Current operation state |
| `statusText` | `string` | Human-readable status message |
| `stats` | `StatsSnapshot` | Post-operation result metrics |
| `tools` | `ToolAvailability \| null` | FFmpeg/Ghostscript availability |
| `logs` | `LogEntry[]` | Operation history (newest first) |

**Derived values**:
- `category` — resolved from `request.overrideCategory ?? selectedFile?.category ?? 'generic'`
- `isBusy` — `true` during compress/decompress
- `canCompress` / `canDecompress` — gate action buttons

**Key handler functions**:
- `pushLog(level, message)` — prepends timestamped log entry
- `refreshFileInfo(path)` — calls `get_file_info` IPC, updates `selectedFile` and `tools`
- `handleChooseFile(mode?)` — opens UploadModal
- `handleFileConfirmed(path, info)` — callback from UploadModal
- `handleCompress()` — invokes `compress_file` IPC, updates stats/status/logs
- `handleDecompress()` — invokes `decompress_file` IPC
- `handleClear()` — resets all state to defaults

**Layout** (top to bottom):
1. Top bar — brand + status badge + LogDropdown
2. Status strip — one-line colored status text
3. Quick Start section (only when no file selected) — 5 mode buttons (Auto, Image, Video, PDF, Generic)
4. Two-column dashboard grid:
   - Left: FilePicker → CompressionSettings → ActionPanel
   - Right: StatsPanel → AlgorithmExplanation
5. UploadModal (conditional overlay)

**`useEffect` on mount**: Detects Tauri runtime. If browser: sets `BROWSER_TOOL_AVAILABILITY` with warning. If Tauri: invokes `get_tool_availability`.

### 6.7 Component Tree

```
App
├── LogDropdown                      ← logs[]
├── FilePicker                       ← selectedFile, selectedPath, category
│   ├── GlassCard
│   └── FileTypeBadge
├── CompressionSettings              ← category, request, tools, disabled
│   ├── GlassCard
│   ├── FileTypeBadge
│   ├── CustomSelect                 ← preset/algorithm/encoder selection
│   ├── Stepper                      ← resize %, zstd level
│   └── ToolNotice (internal)
├── ActionPanel                      ← canCompress, canDecompress, isBusy
│   └── GlassCard
├── StatsPanel                       ← stats
│   └── GlassCard
├── AlgorithmExplanation             ← category, request
│   └── GlassCard
└── UploadModal                      ← mode, fetchFileInfo
    └── GlassCard
```

All state flows down from `App.tsx`. User actions call handlers in `App.tsx` which invoke Tauri IPC and update state.

### 6.8 Component Reference

#### `ActionPanel.tsx`
- **Props**: `canCompress, canDecompress, isBusy, onChooseFile, onCompress, onDecompress, onClear`
- **Renders**: Four buttons (Choose File, Compress, Decompress, Clear) inside GlassCard
- **Logic**: Buttons disabled based on `canCompress`/`canDecompress`/`isBusy`

#### `AlgorithmExplanation.tsx`
- **Props**: `category: FileCategory, request: CompressionRequest`
- **Renders**: GlassCard with summary paragraph + bullet list
- **Logic**: Resolves explanation key from `ALGORITHM_EXPLANATIONS` (generic uses `request.genericAlgorithm`, others use category)

#### `CompressionSettings.tsx`
- **Props**: `category, request, tools, disabled, onChange`
- **Renders** (category-dependent):
  - **Image**: Quality preset (CustomSelect) + Resize % (Stepper) + JPEG encoder selector (standard vs native Rust)
  - **Video**: ToolNotice for FFmpeg availability
  - **PDF**: PDF preset (CustomSelect) + ToolNotice for Ghostscript
  - **Generic**: Algorithm choice (CustomSelect) + conditional Zstd level (Stepper, only when zstd selected)

#### `CustomSelect.tsx`
- **Props**: `id?, label, value, options, disabled?, onChange`
- **Features**: Portal-rendered menu (`createPortal`), viewport-aware positioning (opens above if insufficient space below), keyboard nav (arrows, Enter, Escape), ARIA roles, repositions on scroll/resize

#### `FilePicker.tsx`
- **Props**: `selectedFile, selectedPath, activeCategory?, onChooseFile?, onOverrideCategory?`
- **When file selected**: Displays file name, path, extension, detected category, size, SHA-256. Provides category override toggle with buttons for each category.
- **When no file**: Empty state with dashed border, clickable to browse

#### `FileTypeBadge.tsx`
- **Props**: `category: FileCategory`
- **Renders**: `<span>` with category-specific CSS class and label

#### `GlassCard.tsx`
- **Props**: `id?, title, subtitle?, className?, aside?, children`
- **Renders**: `<section>` with glassmorphism styling, header (title + optional subtitle + aside slot), children

#### `LogDropdown.tsx`
- **Props**: `logs: LogEntry[]`
- **Features**: Topbar dropdown, portal-rendered, count badge, right-aligned, scroll containment (custom wheel handler), Escape/outside-click close, color-coded entries (info/success/error)

#### `StatsPanel.tsx`
- **Props**: `stats: StatsSnapshot`
- **Displays**: Method, Lossless flag, Original/Output/Saved sizes, Compression Ratio, Space Saved %, Elapsed time, Output path, Source/Output SHA-256, Expected SHA-256, Integrity (match/mismatch)

#### `Stepper.tsx`
- **Props**: `id, label, value, min, max, step?, suffix?, disabled?, onChange`
- **Renders**: Numeric input with +/- buttons, clamping, validates on blur
- **Used for**: Resize % (10–100, step 5) and Zstd level (1–19)

#### `UploadModal.tsx`
- **Props**: `mode: FileCategory | 'auto', onClose, onFileConfirmed, fetchFileInfo`
- **Features**: Backdrop overlay, drag-and-drop zone (reads `file.path` — Tauri webview specific), native file dialog via `@tauri-apps/plugin-dialog` with mode-specific filters, category mismatch handling ("Proceed as detected" or "Cancel & Reselect")

### 6.9 CSS Design System (`styles.css`)

~1900 lines of vanilla CSS implementing a glassmorphism design system.

**Design tokens** (`:root` custom properties):
- **Colors**: canvas, ink, muted, accent/blue, mint, amber, coral
- **Glass**: base/strong/frost/border/shadow opacity values
- **Spacing**: 8-step scale
- **Radii**: xs → pill
- **Blur levels**, easing curves, transition durations
- **Typography**: Inter (Google Fonts)

**Key CSS classes**:
| Class | Purpose |
|-------|---------|
| `.app-shell` | Root container |
| `.glass-panel` | Shared glassmorphism card (backdrop-filter blur + saturate, gradient borders, shadows) |
| `.glass-modal`, `.glass-drawer`, `.glass-menu` | Variant glass containers |
| `.dashboard` | 2-column CSS grid (1.2fr + 1fr), max-width 1580px |
| `.category-badge` | Color-coded pill per file category |
| `.btn`, `.btn-primary`, `.btn-muted` | Action buttons with hover lift + shine sweep animation |
| `.data-row` | Two-column label/value grid (170px + 1fr) |
| `.custom-select-*` | Custom dropdown styles |
| `.stepper-*` | Numeric stepper styles |

**Responsive breakpoints**: 1180px, 860px, 560px — progressively collapses to single column.

**Animation keyframes**: `dropdown-open`, `overlay-fade-in`, `log-row-in`, `modal-scale-in`, status pulse.

**Fallbacks**: `@supports not (backdrop-filter)` → solid white background. `prefers-reduced-motion` → disables all animations.

---

## 7. Backend (Rust + Tauri)

### 7.1 Crate Configuration

**`Cargo.toml`**:
- **Crate name**: `compressa_studio`
- **Library name**: `compressa_studio_lib`
- **Crate types**: `staticlib`, `cdylib`, `rlib`
- **Edition**: 2021, MSRV 1.77.2

**Dependencies**:

| Crate | Version | Purpose |
|-------|---------|---------|
| `tauri` | 2.11.2 | Desktop runtime |
| `tauri-plugin-dialog` | 2 | Native file dialogs |
| `serde` | 1.0 (derive) | Serialization |
| `serde_json` | 1.0 | JSON handling |
| `anyhow` | 1 | Error handling |
| `sha2` | 0.10 | SHA-256 hashing |
| `infer` | 0.15 | MIME type detection (magic bytes) |
| `zstd` | 0.13 | Zstandard compression |
| `image` | 0.25 (jpeg, png, webp) | Image decoding/encoding |
| `webp` | 0.3 | WebP encoding |

### 7.2 Entry Point & Module Declarations

**`main.rs`**: Minimal binary entry. Calls `compressa_studio_lib::run()`. Has `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` to hide console on Windows release.

**`lib.rs`**: Declares all modules. Configures Tauri builder:
- Plugin: `tauri_plugin_dialog`
- IPC handlers: `get_file_info`, `get_tool_availability`, `compress_file`, `decompress_file`

### 7.3 IPC Commands (`commands.rs`)

All commands use `spawn_blocking` to offload I/O from the async runtime. Errors are converted from `anyhow::Error` to `String` for frontend consumption.

| Command | Input | Output | Delegates to |
|---------|-------|--------|-------------|
| `get_file_info` | `path: String` | `FileInfo` | `file_info::inspect_file` |
| `get_tool_availability` | (none) | `ToolAvailability` | `tool_detection::detect_tools` |
| `compress_file` | `path: String, request: CompressionRequest` | `OperationResult` | `compression::compress` |
| `decompress_file` | `path: String` | `OperationResult` | `compression::decompress` |

### 7.4 File Detection (`file_detection.rs`)

**Two-pass detection**:
1. **Magic bytes** via `infer` crate — detects MIME type from file content
2. **Extension fallback** — maps known extensions to categories

**Enum `FileCategory`**: `Image`, `Video`, `Pdf`, `Generic` (serialized as camelCase)

**Struct `FileDetection`**: `extension, category, inferred_category, is_supported_input, is_decompressible`

**Extension mappings**:
- **Image**: `jpg, jpeg, png, webp`
- **Video**: `mp4, mov, mkv, avi`
- **PDF**: `pdf`
- **Decompressible**: `zst, rle, shnc, sfc, huff, lz77, lz78, lzw, arith`
- **Everything else**: Generic

### 7.5 File Info (`file_info.rs`)

**`inspect_file(path) → Result<FileInfo>`**:
1. Validates path exists and is a file
2. Calls `file_detection::detect_file` → category + extension + decompressible flag
3. Calls `checksum::sha256_file` → hex SHA-256
4. Calls `tool_detection::detect_tools` → FFmpeg/Ghostscript availability
5. Assembles `FileInfo` struct (serialized camelCase for frontend)

### 7.6 Checksum (`checksum.rs`)

**`sha256_file(path) → Result<String>`**: Streams file in 64 KiB chunks through `sha2::Sha256`. Returns lowercase hex digest. Used for:
- File info display
- Compression metadata (original hash)
- Decompression integrity verification

### 7.7 Tool Detection (`tool_detection.rs`)

**`detect_tools() → ToolAvailability`**: Probes `ffmpeg -version` and `gs --version` via `std::process::Command`.

Returns per-tool `ToolStatus`:
- `name`: tool name
- `available`: `bool` (command found + exit 0)
- `command`: actual command string
- `detail`: first line of stdout (version) or error message

### 7.8 Compression Module

Located in `src-tauri/src/compression/`. Contains 9 source files.

#### `mod.rs` — Compression Router (377 lines)

**Shared types**:

| Type | Fields |
|------|--------|
| `QualityPreset` | `HighQuality, Balanced, SmallSize` |
| `PdfPreset` | `Screen, Ebook, Print` |
| `GenericAlgorithm` | `Zstd, Rle, Shannon, ShannonFano, Huffman, Lz77, Lz78, Lzw, Arithmetic` |
| `CompressionRequest` | `quality_preset?, pdf_preset?, generic_algorithm?, resize_percent?, zstd_level?, use_native_jpeg?, override_category?` |
| `OperationResult` | `operation, category, method, lossless, input_path, output_path, original_size, output_size, saved_size, compression_ratio, space_saved, elapsed_ms, source_sha256, output_sha256, expected_sha256?, integrity_match?, metadata_path?, message` |
| `CompressionMetadata` | `original_file_name, original_extension, original_sha256, original_size, algorithm` |

**`compress(path, request) → Result<OperationResult>`**:
1. Detects category (with `override_category` support + magic byte safety check)
2. Computes source SHA-256
3. Dispatches to: `image_compressor`, `video_compressor`, `pdf_compressor`, or `generic_compressor`
4. Records timing, computes output hash/size/ratio/savings
5. Returns `OperationResult`

**`decompress(path) → Result<OperationResult>`**:
1. Validates extension is decompressible
2. Extension-based dispatch:
   - `.zst` → `generic_compressor`
   - `.rle` → `native_rle`
   - `.shnc` → `native_entropy::shannon`
   - `.sfc` → `native_entropy::shannon_fano`
   - `.huff` → `native_entropy::huffman`
   - `.lz77`, `.lz78`, `.lzw` → `native_dict`
   - `.arith` → `native_entropy::arithmetic`
3. Reads sidecar `.meta.json` for expected SHA-256
4. Verifies integrity (compares expected vs actual hash)
5. Returns `OperationResult`

**Utility functions**: `validate_input_file`, `ensure_output_exists`, `calculate_compression_ratio`, `calculate_space_saved`, `metadata_path_for`, `write_metadata`, `read_metadata`, `create_metadata`, `infer_restored_output_path`, `unique_output_path`.

#### `generic_compressor.rs` — Zstd + Algorithm Dispatch (174 lines)

- **`compress_generic(path, algo, level, sha256, size)`**: Dispatches to Zstd (handled here) or native algorithms
- **`compress_zstd(path, level)`**: Streaming encoder via `zstd` crate, levels 1–19 (clamped)
- **`decompress_zstd(path)`**: Streaming decoder
- Writes `.meta.json` sidecar for all algorithms

#### `image_compressor.rs` — Image Pipeline (198 lines)

- **`compress_image(path, request)`**: Main entry
- Decodes via `image` crate, optional resize (Lanczos3 filter, 10–100%)
- **Standard path** (`use_native_jpeg = false`):
  - Alpha → WebP encoding (`webp` crate)
  - No alpha + HighQuality → JPEG
  - No alpha + Balanced/SmallSize → WebP
- **Native path** (`use_native_jpeg = true`): Delegates to `native_jpeg`
- Quality constants: High=90, Balanced=75, Small=50 (JPEG), 80/65/40 (WebP)

#### `native_jpeg.rs` — From-Scratch JPEG Encoder (400 lines)

Full educational JPEG implementation:
1. **RGB → YCbCr** color space conversion
2. **8×8 block splitting** with edge replication for non-multiple-of-8 dimensions
3. **Forward DCT** with precomputed cosine table
4. **Quantization** using standard JFIF luma/chroma tables, scaled by quality factor (IJG formula: `scale = q < 50 ? 5000/q : 200-2q`)
5. **Zigzag reordering** (standard 64-element table)
6. **DC DPCM** + **AC RLE** encoding
7. **Huffman coding** with standard JPEG DC/AC luma/chroma tables
8. **JFIF byte stream**: SOI, APP0, DQT×2, SOF0 (4:4:4 baseline), DHT×4, SOS, entropy data, EOI
9. `0xFF` byte stuffing

Quality presets: HighQuality=90, Balanced=75, SmallSize=40.

#### `native_rle.rs` — Run-Length Encoding (117 lines)

- **Format**: `[count:u8][value:u8]` pairs, max run 255
- **Header**: Magic bytes `COMPRESSA_RLE_V1`
- **`compress_rle(path) → (output_path, metadata_path)`**
- **`decompress_rle(path) → output_path`**
- Validates even payload length on decompress

#### `native_entropy.rs` — Entropy Coders (673 lines)

Four educational entropy coding algorithms sharing infrastructure:

**Shared infrastructure**:
- `BitWriter` / `BitReader` — bit-level I/O
- `HuffNode` — binary tree node for prefix codes
- `encode_with_tree()` / `decode_with_tree()` — shared tree-based encode/decode
- All store 256×u32 frequency table (1024 bytes LE) + valid_bits byte + bitstream

**Shannon Coding**:
- `generate_shannon_codes()` — assigns codes based on cumulative probability and `-log2(p)` length
- Header: `COMPRESSA_SHNC_V1`
- Output extension: `.shnc`

**Shannon-Fano Coding**:
- `generate_shannon_fano_codes()` — top-down recursive binary partition minimizing frequency imbalance
- Header: `COMPRESSA_SFC_V1`
- Output extension: `.sfc`

**Huffman Coding**:
- `generate_huffman_codes()` — bottom-up `BinaryHeap` (min-heap via `Reverse`) tree construction
- Header: `COMPRESSA_HUFF_V1`
- Output extension: `.huff`

**Arithmetic Coding** (separate path):
- `ArithModel` — cumulative frequency table, `ARITH_MAX_FREQ = 16384`
- `ArithmeticEncoder` / `ArithmeticDecoder` — 32-bit range coder with underflow handling and renormalization
- Header: `COMPRESSA_ARITH_V1`
- Output extension: `.arith`
- Stores 256-entry frequency table in header

#### `native_dict.rs` — Dictionary Coders (399 lines)

Three educational dictionary-based algorithms:

**LZ77**:
- Hash-chained sliding window (65536 hash table, 100 chain limit, max match 255)
- Token format: `[offset:u16][length:u8][next_byte:u8]`
- Header: `COMPRESSA_LZ77_V1` + original length (u64 BE)
- Output extension: `.lz77`

**LZ78**:
- Trie-based dictionary with `(prefix_idx:u32, byte:u8)` entries
- Header: `COMPRESSA_LZ78_V1`
- Output extension: `.lz78`

**LZW**:
- Pre-initialized 256-entry single-byte dictionary
- 32-bit dictionary indices
- Header: `COMPRESSA_LZW__V1`
- Output extension: `.lzw`

#### `video_compressor.rs` — FFmpeg Wrapper (85 lines)

- **`compress_video(path)`**: Shells out to `ffmpeg`
- Guards on `detect_tools().ffmpeg.available`
- Codec: `libx264`, Audio: `aac`
- CRF by preset: HighQuality=21 (slow, 160k audio), Balanced=28 (medium, 128k), SmallSize=32 (medium, 96k)
- Output: `_compressed.mp4` with `+faststart`
- Lossy, no metadata sidecar

#### `pdf_compressor.rs` — Ghostscript Wrapper (63 lines)

- **`compress_pdf(path)`**: Shells out to `gs -sDEVICE=pdfwrite`
- Guards on `detect_tools().ghostscript.available`
- Preset → `-dPDFSETTINGS`: Screen=`/screen`, Ebook=`/ebook`, Print=`/printer`
- Output: `_compressed.pdf`
- Lossy, no metadata sidecar

---

## 8. IPC Contract (Frontend ↔ Backend)

All IPC uses Tauri's `invoke()` with JSON serialization. Field naming convention: **camelCase** on both sides (Rust uses `#[serde(rename_all = "camelCase")]`).

### `get_file_info`

```
Request:  { path: string }
Response: FileInfo {
  path, fileName, extension, category, isDecompressible,
  sizeBytes, sha256, tools: { ffmpeg: ToolStatus, ghostscript: ToolStatus }
}
```

### `get_tool_availability`

```
Request:  (none)
Response: ToolAvailability {
  ffmpeg:      { name, available, command, detail },
  ghostscript: { name, available, command, detail }
}
```

### `compress_file`

```
Request: {
  path: string,
  request: {
    qualityPreset?: "highQuality" | "balanced" | "smallSize",
    pdfPreset?: "screen" | "ebook" | "print",
    genericAlgorithm?: "zstd" | "rle" | "shannon" | "shannonFano" | "huffman" | "lz77" | "lz78" | "lzw" | "arithmetic",
    resizePercent?: number,
    zstdLevel?: number,
    useNativeJpeg?: boolean,
    overrideCategory?: "image" | "video" | "pdf" | "generic"
  }
}
Response: OperationResult { ... }  // see types.ts
```

### `decompress_file`

```
Request:  { path: string }
Response: OperationResult { ... }
```

---

## 9. Compression Algorithms Reference

| Algorithm | Type | Extension | Lossless | Implementation | Notes |
|-----------|------|-----------|----------|----------------|-------|
| **Zstd** | Dictionary + FSE | `.zst` | Yes | `zstd` crate | Levels 1–19, default 6 |
| **RLE** | Run-Length | `.rle` | Yes | From scratch | `[count:u8][value:u8]` pairs |
| **Shannon** | Entropy | `.shnc` | Yes | From scratch | `-log2(p)` code lengths |
| **Shannon-Fano** | Entropy | `.sfc` | Yes | From scratch | Top-down recursive partition |
| **Huffman** | Entropy | `.huff` | Yes | From scratch | Bottom-up min-heap tree |
| **LZ77** | Dictionary | `.lz77` | Yes | From scratch | Sliding window, hash chain |
| **LZ78** | Dictionary | `.lz78` | Yes | From scratch | Trie-based phrase dictionary |
| **LZW** | Dictionary | `.lzw` | Yes | From scratch | 256-entry init, 32-bit codes |
| **Arithmetic** | Entropy | `.arith` | Yes | From scratch | 32-bit range coder |
| **Image (JPEG/WebP)** | Transform | `.jpg`/`.webp` | No | `image`+`webp` crates | Quality presets |
| **Native JPEG** | Transform | `.jpg` | No | From scratch | Full DCT+quantization+Huffman |
| **Video (H.264)** | Inter/Intra-frame | `.mp4` | No | FFmpeg CLI | CRF-based |
| **PDF** | Document | `.pdf` | No | Ghostscript CLI | dPDFSETTINGS presets |

### Sidecar Metadata (`.meta.json`)

All generic lossless algorithms write a sidecar file alongside the compressed output:

```json
{
  "originalFileName": "example.txt",
  "originalExtension": "txt",
  "originalSha256": "a1b2c3...",
  "originalSize": 12345,
  "algorithm": "huffman"
}
```

Used during decompression for integrity verification (SHA-256 comparison).

### Custom File Format Headers

Each from-scratch algorithm writes a magic header for format identification:

| Algorithm | Magic Bytes |
|-----------|-------------|
| RLE | `COMPRESSA_RLE_V1` |
| Shannon | `COMPRESSA_SHNC_V1` |
| Shannon-Fano | `COMPRESSA_SFC_V1` |
| Huffman | `COMPRESSA_HUFF_V1` |
| LZ77 | `COMPRESSA_LZ77_V1` |
| LZ78 | `COMPRESSA_LZ78_V1` |
| LZW | `COMPRESSA_LZW__V1` |
| Arithmetic | `COMPRESSA_ARITH_V1` |

---

## 10. File Format & Extension Registry

### Supported Input Extensions

| Category | Extensions |
|----------|-----------|
| Image | `jpg, jpeg, png, gif, bmp, tiff, tif, webp, ico, svg` |
| Video | `mp4, mkv, avi, mov, wmv, flv, webm, m4v, ts, mpeg, mpg, 3gp` |
| PDF | `pdf` |
| Generic | Everything else (`.txt, .csv, .json, .xml, .log, .doc, .docx, .ppt, .pptx, .xls, .xlsx, .bin, .dat`, etc.) |

### Output Extensions

| Pipeline | Output |
|----------|--------|
| Image (standard) | `.jpg` or `.webp` (based on alpha + preset) |
| Image (native) | `.jpg` |
| Video | `_compressed.mp4` |
| PDF | `_compressed.pdf` |
| Zstd | `.zst` |
| RLE | `.rle` |
| Shannon | `.shnc` |
| Shannon-Fano | `.sfc` |
| Huffman | `.huff` |
| LZ77 | `.lz77` |
| LZ78 | `.lz78` |
| LZW | `.lzw` |
| Arithmetic | `.arith` |

### Decompression Output Naming

Strips the compression extension and appends `_restored_N` to avoid overwriting:
- `file.txt.zst` → `file.txt_restored_1`
- If `_restored_1` exists → `file.txt_restored_2`, etc.

---

## 11. External Dependencies

### System-Level (Optional)

| Tool | Used For | Detection | Fallback |
|------|----------|-----------|----------|
| **FFmpeg** | Video compression | `ffmpeg -version` | UI shows "Tool Missing", video compression disabled |
| **Ghostscript** | PDF compression | `gs --version` | UI shows "Tool Missing", PDF compression disabled |

### Rust Crates

| Crate | Purpose |
|-------|---------|
| `tauri` 2.11.2 | Desktop runtime, IPC, window management |
| `tauri-plugin-dialog` 2 | Native OS file open dialogs |
| `serde` 1.0 + `serde_json` 1.0 | JSON serialization for IPC |
| `anyhow` 1 | Error context and chaining |
| `sha2` 0.10 | SHA-256 integrity hashing |
| `infer` 0.15 | MIME detection from magic bytes |
| `zstd` 0.13 | Zstandard compression library |
| `image` 0.25 | Image decode/encode (JPEG, PNG, WebP) |
| `webp` 0.3 | WebP encoding |

### NPM Packages

| Package | Purpose |
|---------|---------|
| `react` 19.2.6 + `react-dom` | UI framework |
| `@tauri-apps/api` 2.11.0 | Tauri frontend IPC |
| `@tauri-apps/plugin-dialog` 2.7.1 | File dialog frontend API |
| `vite` 8.0.12 | Bundler/dev server |
| `@vitejs/plugin-react` 6.0.1 | React HMR |
| `typescript` ~6.0.2 | Type checking |
| `eslint` + plugins | Linting |

---

## 12. Supplementary Files

### `run.sh`
Convenience script. Checks for `node_modules/`, runs `npm install` if absent, then `npm run tauri -- dev`.

### `extract.py`
Python utility to extract text from lecture materials in `Materi/`:
- `.pptx` → via `markitdown` Python module
- `.pdf` → via `pdftotext` (poppler-utils)
- Output: `Conclusion.md`

### `Conclusion.md`
Synthesized course material covering: Information Theory (Shannon Entropy), Statistical/Variable-Length Encoding (Shannon, Shannon-Fano, Huffman), Dictionary Encoding (LZ77, LZ78, LZW), Image Compression (JPEG pipeline), Video Compression (MPEG, H.264/265, VP9, AV1).

### `Materi/`
Directory of lecture slide decks (`.pptx`) and PDFs. Gitignored. Contains test compression outputs from the application.

---

## 13. Known Constraints & Design Decisions

1. **Single-file state**: All application state lives in `App.tsx`. No external state management library. Suitable for the current scope.

2. **Synchronous blocking IPC**: All Rust commands use `spawn_blocking` to avoid blocking the Tauri async runtime during file I/O and compression.

3. **Category override**: Users can force any file into any compression pipeline (e.g., compress an image as generic data). The backend performs a safety check via magic bytes.

4. **Native JPEG encoder**: Educational implementation. Does not support progressive encoding, chroma subsampling (always 4:4:4), or optimized Huffman tables. Produces larger files than production encoders.

5. **Educational algorithms**: RLE, Shannon, Shannon-Fano, LZ77, LZ78, LZW are implemented for learning. They can expand data when applied to already-compressed or high-entropy files.

6. **Arithmetic coding overhead**: Stores a 256×u32 frequency table (1024 bytes) plus format metadata. For small files, this overhead can exceed the savings.

7. **No streaming UI progress**: Compression operations block until complete. The frontend shows only "Compressing…" / "Decompressing…" states, not progress percentages.

8. **Decompression scope**: Only generic lossless outputs can be decompressed. Image, video, and PDF outputs are one-way lossy transformations.

9. **Sidecar dependency**: Integrity verification requires the `.meta.json` sidecar file to be present alongside the compressed file. Decompression works without it but skips checksum validation.

10. **External tool dependency**: Video and PDF compression require system-installed FFmpeg and Ghostscript respectively. The app gracefully degrades (disables buttons, shows warnings) when these are absent.

11. **Window default size**: 1320×860 pixels, resizable. No minimum size constraint in config.

12. **CSP**: Set to `null` (no Content Security Policy restrictions) in `tauri.conf.json`.

13. **Portal rendering**: `CustomSelect` and `LogDropdown` use `createPortal` to render menus to `document.body` to avoid parent overflow clipping.
