use super::{unique_output_path, CompressionOutcome, CompressionRequest, QualityPreset};
use crate::tool_detection::detect_tools;
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

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
        QualityPreset::HighQuality => ("21", "slow", "160k"),
        QualityPreset::Balanced => ("28", "medium", "128k"),
        QualityPreset::SmallSize => ("32", "medium", "96k"),
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
