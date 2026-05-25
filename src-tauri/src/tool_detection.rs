use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub command: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAvailability {
    pub ffmpeg: ToolStatus,
    pub ghostscript: ToolStatus,
}

pub fn detect_tools() -> ToolAvailability {
    ToolAvailability {
        ffmpeg: detect_tool("ffmpeg", ["-version"], ffmpeg_help()),
        ghostscript: detect_tool("gs", ["--version"], ghostscript_help()),
    }
}

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
