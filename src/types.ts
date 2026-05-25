/**
 * Represents the current status of the application workflow.
 */
export type AppStatus = 'idle' | 'compressing' | 'decompressing' | 'success' | 'error'

/**
 * The high-level file classification determining the compression route.
 */
export type FileCategory = 'image' | 'video' | 'pdf' | 'generic'

/**
 * Quality preset options for image compression.
 */
export type QualityPreset = 'highQuality' | 'balanced' | 'smallSize'

/**
 * Presets mapping to Ghostscript PDF downsampling targets.
 */
export type PdfPreset = 'screen' | 'ebook' | 'print'

/**
 * The specific algorithm chosen when using generic compression.
 */
export type GenericAlgorithm =
  | 'zstd'
  | 'rle'
  | 'shannon'
  | 'shannonFano'
  | 'huffman'
  | 'lz77'
  | 'lz78'
  | 'lzw'
  | 'arithmetic'

/**
 * Severity level for log entries.
 */
export type LogLevel = 'info' | 'success' | 'error'

/**
 * Describes the availability and status of an external tool (e.g., ffmpeg).
 */
export interface ToolStatus {
  /** The name of the tool (e.g., 'ffmpeg') */
  name: string
  /** Whether the tool is accessible and runnable */
  available: boolean
  /** The actual command used to test availability */
  command: string
  /** The version string or error message from the tool */
  detail: string
}

/**
 * Container for the availability of all required external tools.
 */
export interface ToolAvailability {
  ffmpeg: ToolStatus
  ghostscript: ToolStatus
}

/**
 * Metadata and classification info for a file selected by the user.
 */
export interface FileInfo {
  /** Absolute path to the file */
  path: string
  /** The basename of the file */
  fileName: string
  /** Normalized lowercase extension (or empty string) */
  extension: string
  /** The detected category determining how it will be compressed */
  category: FileCategory
  /** Whether the file is recognized as an output created by this app */
  isDecompressible: boolean
  /** Size of the file in bytes */
  sizeBytes: number
  /** Hex string of the SHA-256 hash */
  sha256: string
  /** Tool availability detected at the time this file was inspected */
  tools: ToolAvailability
}

/**
 * User-configurable settings for a compression operation.
 */
export interface CompressionRequest {
  qualityPreset?: QualityPreset
  pdfPreset?: PdfPreset
  genericAlgorithm?: GenericAlgorithm
  resizePercent?: number | null
  zstdLevel?: number | null
  useNativeJpeg?: boolean | null
  overrideCategory?: FileCategory | null
}

/**
 * The full result returned by the backend after an operation finishes.
 */
export interface OperationResult {
  operation: 'compress' | 'decompress'
  category: FileCategory
  method: string
  lossless: boolean
  inputPath: string
  outputPath: string
  originalSize: number
  outputSize: number
  savedSize: number
  compressionRatio: number
  spaceSaved: number
  elapsedMs: number
  sourceSha256: string
  outputSha256: string
  expectedSha256?: string | null
  integrityMatch?: boolean | null
  metadataPath?: string | null
  message: string
}

/**
 * A partial snapshot of an operation result used by the UI stats panel.
 */
export interface StatsSnapshot {
  method?: string
  lossless?: boolean
  originalSize?: number
  outputSize?: number
  savedSize?: number
  compressionRatio?: number
  spaceSaved?: number
  elapsedMs?: number
  sourceSha256?: string
  outputSha256?: string
  expectedSha256?: string
  integrityMatch?: boolean
  outputPath?: string
}

/**
 * A single entry in the application log.
 */
export interface LogEntry {
  id: string
  timestamp: string
  level: LogLevel
  message: string
}
