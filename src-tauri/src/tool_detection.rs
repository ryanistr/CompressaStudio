//! External tool availability detection.
//!
//! Probes the system PATH for required CLI tools (ffmpeg, Ghostscript) and
//! reports their availability and version info to the frontend.

use serde::{Deserialize, Serialize};
use std::process::Command;

/// Status of a single external tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    /// Display name of the tool.
    pub name: String,
    /// Whether the tool was found and executed successfully.
    pub available: bool,
    /// CLI command used to invoke the tool.
    pub command: String,
    /// Version string or help message if the tool is missing.
    pub detail: String,
}

/// Availability status for all external tools used by the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAvailability {
    /// Status of the ffmpeg video encoder.
    pub ffmpeg: ToolStatus,
    /// Status of the Ghostscript PDF processor.
    pub ghostscript: ToolStatus,
}

/// Probes the system for ffmpeg and Ghostscript availability.
///
/// Returns a [`ToolAvailability`] snapshot with version details or
/// installation guidance for each tool.
pub fn detect_tools() -> ToolAvailability {
    ToolAvailability {
        ffmpeg: detect_tool("ffmpeg", ["-version"], ffmpeg_help()),
        ghostscript: detect_tool("gs", ["--version"], ghostscript_help()),
    }
}

/// Attempts to run `command` with `args` and constructs a [`ToolStatus`].
///
/// On success, extracts the first line of stdout/stderr as the version detail.
/// On failure, populates the detail with `missing_help`.
fn detect_tool<const N: usize>(
    command: &str,
    args: [&str; N],
    missing_help: &'static str,
) -> ToolStatus {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let detail = stdout
                .lines()
                .next()
                .or_else(|| stderr.lines().next())
                .unwrap_or("Installed")
                .trim()
                .to_string();

            ToolStatus {
                name: command.to_string(),
                available: true,
                command: command.to_string(),
                detail,
            }
        }
        Ok(output) => ToolStatus {
            name: command.to_string(),
            available: false,
            command: command.to_string(),
            detail: format!(
                "{missing_help} Command exited with status {}.",
                output.status
            ),
        },
        Err(_) => ToolStatus {
            name: command.to_string(),
            available: false,
            command: command.to_string(),
            detail: missing_help.to_string(),
        },
    }
}

fn ffmpeg_help() -> &'static str {
    "ffmpeg was not found. Install ffmpeg and ensure it is available on PATH."
}

fn ghostscript_help() -> &'static str {
    "Ghostscript was not found. Install Ghostscript (`gs`) and ensure it is available on PATH."
}
