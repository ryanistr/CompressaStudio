/**
 * @file utils.ts
 * @description Shared utility functions for formatting and basic operations.
 */

/**
 * Formats a size in bytes to a human-readable string.
 * @param bytes The size in bytes.
 * @returns Formatted size string (e.g., "1.5 MB").
 */
export function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

/**
 * Formats a duration in milliseconds to a human-readable string.
 * @param ms Duration in milliseconds.
 * @returns Formatted duration string (e.g., "1.2s", "450ms").
 */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`
  return `${(ms / 1000).toFixed(2)}s`
}

/**
 * Gets the current timestamp in locale time string format.
 * @returns The current time as a string.
 */
export function timestampNow(): string {
  return new Date().toLocaleTimeString()
}

/**
 * Converts an unknown error object to a readable error message string.
 * @param error The error object to convert.
 * @returns A string representation of the error.
 */
export function toMessage(error: unknown): string {
  if (typeof error === 'string') {
    return error
  }
  if (error instanceof Error) {
    return error.message
  }
  return 'Unknown error'
}
