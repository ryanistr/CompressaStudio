//! Tauri IPC command handlers exposed to the frontend.
//!
//! Each public function is registered via `tauri::generate_handler!` and
//! invoked from the webview through Tauri's command protocol.

use crate::compression::{self, CompressionRequest, OperationResult};
use crate::file_info::{self, FileInfo};
use crate::tool_detection::{self, ToolAvailability};
use std::path::PathBuf;

/// Inspects a file at `path` and returns its metadata, category, and SHA-256 checksum.
///
/// Runs blocking I/O on a dedicated thread to avoid stalling the async runtime.
///
/// # Errors
///
/// Returns a `String` describing the failure if the file cannot be read or inspected.
#[tauri::command]
pub async fn get_file_info(path: String) -> Result<FileInfo, String> {
    let task = tauri::async_runtime::spawn_blocking(move || {
        let path = PathBuf::from(path);
        file_info::inspect_file(&path)
    });

    match task.await {
        Ok(result) => result.map_err(format_error),
        Err(error) => Err(format!("Failed to complete file info task: {error}")),
    }
}

/// Detects which external tools (ffmpeg, Ghostscript) are available on the system PATH.
///
/// # Errors
///
/// Returns a `String` if the background task panics.
#[tauri::command]
pub async fn get_tool_availability() -> Result<ToolAvailability, String> {
    let task = tauri::async_runtime::spawn_blocking(tool_detection::detect_tools);

    match task.await {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("Failed to inspect tool availability: {error}")),
    }
}

/// Compresses the file at `path` according to the provided [`CompressionRequest`].
///
/// Dispatches to the appropriate compressor based on file category and request parameters.
///
/// # Errors
///
/// Returns a `String` describing the failure if compression fails at any stage.
#[tauri::command]
pub async fn compress_file(
    path: String,
    request: CompressionRequest,
) -> Result<OperationResult, String> {
    let task = tauri::async_runtime::spawn_blocking(move || {
        let path = PathBuf::from(path);
        compression::compress(&path, request)
    });

    match task.await {
        Ok(result) => result.map_err(format_error),
        Err(error) => Err(format!("Failed to complete compression task: {error}")),
    }
}

/// Decompresses a previously compressed file at `path`, restoring the original content.
///
/// The algorithm is inferred from the file extension (`.zst`, `.rle`, `.huff`, etc.).
///
/// # Errors
///
/// Returns a `String` describing the failure if decompression fails.
#[tauri::command]
pub async fn decompress_file(path: String) -> Result<OperationResult, String> {
    let task = tauri::async_runtime::spawn_blocking(move || {
        let path = PathBuf::from(path);
        compression::decompress(&path)
    });

    match task.await {
        Ok(result) => result.map_err(format_error),
        Err(error) => Err(format!("Failed to complete decompression task: {error}")),
    }
}

/// Formats an [`anyhow::Error`] into a display string, preserving the full causal chain.
fn format_error(error: anyhow::Error) -> String {
    format!("{error:#}")
}
