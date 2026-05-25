use super::{unique_output_path, CompressionOutcome, CompressionRequest, QualityPreset};
use anyhow::{Context, Result};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use webp::Encoder;

pub fn compress_image(
    input_path: &Path,
    request: &CompressionRequest,
) -> Result<CompressionOutcome> {
    if request.use_native_jpeg.unwrap_or(false) {
        return crate::compression::native_jpeg::compress_native_jpeg(input_path, request);
    }

    let preset = request.quality_preset.unwrap_or(QualityPreset::Balanced);
    let resize_percent = request.resize_percent.unwrap_or(100).clamp(10, 100);
    let image = image::open(input_path)
        .with_context(|| format!("Unable to decode image: {}", input_path.display()))?;
    let image = maybe_resize(image, resize_percent);
    let has_alpha = matches!(
        image.color(),
        image::ColorType::La8
            | image::ColorType::La16
            | image::ColorType::Rgba8
            | image::ColorType::Rgba16
            | image::ColorType::Rgba32F
    );
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("image");
    let output_format = choose_output_format(has_alpha, preset);
    let output_path = unique_output_path(
        parent,
        &format!("{stem}_compressed"),
        output_format.extension(),
    );

    match output_format {
        OutputFormat::Jpeg { quality } => write_jpeg(&image, &output_path, quality)?,
        OutputFormat::Webp { quality } => write_webp(&image, &output_path, quality)?,
    }

    Ok(CompressionOutcome {
        output_path: output_path.to_string_lossy().to_string(),
        method: format!(
            "Image Compression ({})",
            match output_format {
                OutputFormat::Jpeg { .. } => "JPEG lossy DCT/quantization",
                OutputFormat::Webp { .. } => "WebP lossy predictive transform",
            }
        ),
        lossless: false,
        message: format!(
            "Image compression completed with {:?} preset and resize {}%.",
            preset, resize_percent
        ),
        metadata_path: None,
    })
}

fn maybe_resize(image: DynamicImage, resize_percent: u32) -> DynamicImage {
    if resize_percent >= 100 {
        return image;
    }

    let (width, height) = image.dimensions();
    let new_width = (width.saturating_mul(resize_percent) / 100).max(1);
    let new_height = (height.saturating_mul(resize_percent) / 100).max(1);
    image.resize(new_width, new_height, FilterType::Lanczos3)
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Jpeg { quality: f32 },
    Webp { quality: f32 },
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Jpeg { .. } => "jpg",
            Self::Webp { .. } => "webp",
        }
    }
}

fn choose_output_format(has_alpha: bool, preset: QualityPreset) -> OutputFormat {
    if has_alpha {
        return OutputFormat::Webp {
            quality: match preset {
                QualityPreset::HighQuality => 88.0,
                QualityPreset::Balanced => 78.0,
                QualityPreset::SmallSize => 62.0,
            },
        };
    }

    match preset {
        QualityPreset::HighQuality => OutputFormat::Jpeg { quality: 88.0 },
        QualityPreset::Balanced => OutputFormat::Webp { quality: 76.0 },
        QualityPreset::SmallSize => OutputFormat::Webp { quality: 58.0 },
    }
}

fn write_jpeg(image: &DynamicImage, output_path: &Path, quality: f32) -> Result<()> {
    let writer = BufWriter::new(
        File::create(output_path)
            .with_context(|| format!("Unable to create JPEG output: {}", output_path.display()))?,
    );
    let rgb = image.to_rgb8();
    let mut encoder =
        JpegEncoder::new_with_quality(writer, quality.round().clamp(1.0, 100.0) as u8);
    encoder
        .encode_image(&DynamicImage::ImageRgb8(rgb))
        .context("JPEG encoding failed")?;
    Ok(())
}

fn write_webp(image: &DynamicImage, output_path: &Path, quality: f32) -> Result<()> {
    let rgba = image.to_rgba8();
    let encoder = Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
    let encoded = encoder.encode(quality);
    let bytes: &[u8] = &encoded;
    std::fs::write(output_path, bytes)
        .with_context(|| format!("Unable to write WebP output: {}", output_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::compress_image;
    use crate::compression::{CompressionRequest, QualityPreset};
    use anyhow::Result;
    use image::{ImageBuffer, Rgba};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn image_compression_creates_output_file() -> Result<()> {
        let input = unique_path("compressa_image_fixture.png");
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_fn(24, 24, |x, y| Rgba([x as u8 * 4, y as u8 * 4, 180, 255]));
        image.save(&input)?;

        let outcome = compress_image(
            &input,
            &CompressionRequest {
                quality_preset: Some(QualityPreset::Balanced),
                pdf_preset: None,
                generic_algorithm: None,
                resize_percent: Some(80),
                zstd_level: None,
            },
        )?;
        let output = PathBuf::from(outcome.output_path);

        assert!(output.exists());

        if input.exists() {
            fs::remove_file(input)?;
        }
        if output.exists() {
            fs::remove_file(output)?;
        }

        Ok(())
    }

    fn unique_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}_{name}"))
    }
}
