use super::{unique_output_path, CompressionOutcome, CompressionRequest, PdfPreset};
use crate::tool_detection::detect_tools;
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

pub fn compress_pdf(input_path: &Path, request: &CompressionRequest) -> Result<CompressionOutcome> {
    let tools = detect_tools();
    if !tools.ghostscript.available {
        return Err(anyhow!(tools.ghostscript.detail));
    }

    let preset = request.pdf_preset.unwrap_or(PdfPreset::Ebook);
    let gs_profile = match preset {
        PdfPreset::Screen => "/screen",
        PdfPreset::Ebook => "/ebook",
        PdfPreset::Print => "/printer",
    };

    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("document");
    let output_path = unique_output_path(parent, &format!("{stem}_compressed"), "pdf");

    let status = Command::new("gs")
        .args([
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dNOPAUSE",
            "-dQUIET",
            "-dBATCH",
        ])
        .arg(format!("-dPDFSETTINGS={gs_profile}"))
        .arg(format!("-sOutputFile={}", output_path.to_string_lossy()))
        .arg(input_path)
        .status()
        .context("Failed to start Ghostscript")?;

    if !status.success() {
        return Err(anyhow!(
            "Ghostscript failed to compress the PDF. Check whether the PDF is readable and not protected."
        ));
    }

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: "PDF Compression (Ghostscript pdfwrite)".to_string(),
        lossless: false,
        message: format!(
            "PDF compression completed with {:?} preset using Ghostscript.",
            preset
        ),
        metadata_path: None,
    })
}
