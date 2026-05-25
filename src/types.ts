export type AppStatus = 'idle' | 'compressing' | 'decompressing' | 'success' | 'error'

export type FileCategory = 'image' | 'video' | 'pdf' | 'generic'
export type QualityPreset = 'highQuality' | 'balanced' | 'smallSize'
export type PdfPreset = 'screen' | 'ebook' | 'print'
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
export type LogLevel = 'info' | 'success' | 'error'

export interface ToolStatus {
  name: string
  available: boolean
  command: string
  detail: string
}

export interface ToolAvailability {
  ffmpeg: ToolStatus
  ghostscript: ToolStatus
}

export interface FileInfo {
  path: string
  fileName: string
  extension: string
  category: FileCategory
  isDecompressible: boolean
  sizeBytes: number
  sha256: string
  tools: ToolAvailability
}

export interface CompressionRequest {
  qualityPreset?: QualityPreset
  pdfPreset?: PdfPreset
  genericAlgorithm?: GenericAlgorithm
  resizePercent?: number | null
  zstdLevel?: number | null
  useNativeJpeg?: boolean | null
  overrideCategory?: FileCategory | null
}

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

export interface LogEntry {
  id: string
  timestamp: string
  level: LogLevel
  message: string
}
