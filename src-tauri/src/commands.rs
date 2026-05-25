use crate::compression::{self, CompressionRequest, OperationResult};
use crate::file_info::{self, FileInfo};
use crate::tool_detection::{self, ToolAvailability};
use std::path::PathBuf;

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

#[tauri::command]
pub async fn get_tool_availability() -> Result<ToolAvailability, String> {
    let task = tauri::async_runtime::spawn_blocking(tool_detection::detect_tools);

    match task.await {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("Failed to inspect tool availability: {error}")),
    }
}

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

fn format_error(error: anyhow::Error) -> String {
    error.to_string()
}
