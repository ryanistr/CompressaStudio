//! Video Compressor module.
//!
//! Handles video compressor operations.

use super::{unique_output_path, CompressionOutcome, CompressionRequest, QualityPreset};
use crate::tool_detection::detect_tools;
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

const CRF_HIGH_QUALITY: &str = "21";
const CRF_BALANCED: &str = "28";
const CRF_SMALL_SIZE: &str = "32";

const SPEED_SLOW: &str = "slow";
const SPEED_MEDIUM: &str = "medium";

const BITRATE_HIGH: &str = "160k";
const BITRATE_BALANCED: &str = "128k";
const BITRATE_SMALL: &str = "96k";

/// Compress Video.
pub fn compress_video(
    input_path: &Path,
    request: &CompressionRequest,
) -> Result<CompressionOutcome> {
    let tools = detect_tools();
    if !tools.ffmpeg.available {
        return Err(anyhow!(tools.ffmpeg.detail));
    }

    let preset = request.quality_preset.unwrap_or(QualityPreset::Balanced);
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("video");
    let output_path = unique_output_path(parent, &format!("{stem}_compressed"), "mp4");

    let (crf, speed, audio_bitrate) = match preset {
        QualityPreset::HighQuality => (CRF_HIGH_QUALITY, SPEED_SLOW, BITRATE_HIGH),
        QualityPreset::Balanced => (CRF_BALANCED, SPEED_MEDIUM, BITRATE_BALANCED),
        QualityPreset::SmallSize => (CRF_SMALL_SIZE, SPEED_MEDIUM, BITRATE_SMALL),
    };

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_path)
        .args([
            "-c:v",
            "libx264",
            "-preset",
            speed,
            "-crf",
            crf,
            "-c:a",
            "aac",
            "-b:a",
            audio_bitrate,
            "-movflags",
            "+faststart",
        ])
        .arg(&output_path)
        .status()
        .context("Failed to start ffmpeg")?;

    if !status.success() {
        return Err(anyhow!(
            "ffmpeg failed to compress the video. Ensure the input codec is supported."
        ));
    }

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "Video Compression (H.264 via ffmpeg)".to_string(),
        lossless: false,
        message: format!(
            "Video compression completed with {:?} preset using ffmpeg.",
            preset
        ),
        metadata_path: None,
    })
}
