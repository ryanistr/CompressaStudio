import { GlassCard } from './GlassCard'
import type { CompressionRequest, FileCategory } from '../types'

interface AlgorithmExplanationProps {
  category: FileCategory
  request: CompressionRequest
}

export function AlgorithmExplanation({ category, request }: AlgorithmExplanationProps) {
  const explanation = explanationFor(category, request)

  return (
    <GlassCard
      id="algorithm-explanation"
      title="Algorithm Explanation"
      subtitle="This panel mirrors the compression concepts emphasized in the Materi slides."
      className="explanation-panel"
    >
      <div className="explanation-copy">
        <p>{explanation.summary}</p>
        <ul className="explanation-list">
          {explanation.points.map((point) => (
            <li key={point}>{point}</li>
          ))}
        </ul>
      </div>
    </GlassCard>
  )
}

function explanationFor(category: FileCategory, request: CompressionRequest) {
  if (category === 'image') {
    return {
      summary:
        'Image compression in this app is lossy. It follows the Materi discussion on visual redundancy, transform coding, quantization, and quality trade-offs in JPEG/WebP-style image pipelines.',
      points: [
        'Redundancy in neighboring pixels is reduced because many image regions vary slowly.',
        'Lower quality presets keep fewer visually unimportant details, similar to quantization in DCT-based image compression.',
        'Optional resizing reduces pixel count directly, which lowers spatial data before encoding.',
        'Output remains an image file instead of wrapping the image inside a generic archive.',
      ],
    }
  }

  if (category === 'video') {
    return {
      summary:
        'Video compression here is lossy and codec-based. The routing follows the Materi explanation of spatial redundancy, temporal redundancy, motion prediction, DCT, quantization, and bitrate control.',
      points: [
        'Spatial redundancy is reduced within each frame using transform-based compression.',
        'Temporal redundancy is reduced between adjacent frames by predictive coding in the video codec.',
        'Preset changes alter CRF/bitrate pressure, which changes the size-versus-quality trade-off.',
        'Output is re-encoded to a playable MP4 rather than being packed into `.zst`.',
      ],
    }
  }

  if (category === 'pdf') {
    return {
      summary:
        'PDF compression is document-aware. The app uses Ghostscript to optimize embedded objects and images instead of performing a fake archive wrap.',
      points: [
        'Screen, Ebook, and Print presets apply different PDF optimization targets.',
        'Compression may reduce embedded image quality or simplify document resources depending on the preset.',
        'This is lossy in practice because PDF image assets may be downsampled or re-encoded.',
        'The result remains a PDF suitable for document workflows and demos.',
      ],
    }
  }

  if ((request.genericAlgorithm ?? 'zstd') === 'rle') {
    return {
      summary:
        'Educational RLE is a lossless algorithm from the Materi family. It encodes repeated byte runs as count-value pairs to demonstrate redundancy removal directly.',
      points: [
        'RLE works best when the source contains long repeated sequences.',
        'It is easy to explain in a demo because each run is replaced by a shorter symbolic representation.',
        'This module is educational and stable for byte streams, but it is not always the most efficient compressor.',
        'Decompression reconstructs the exact original bytes, so it is lossless.',
      ],
    }
  }

  return {
    summary:
      'Generic lossless compression uses zstd, which fits the Materi themes of entropy, dictionary/statistical coding, and exact reconstruction. The legacy `.zst` workflow is preserved here as the generic module.',
    points: [
      'Lossless means decompression reconstructs the exact original file.',
      'Dictionary/statistical coding exploits repeated patterns and local redundancy efficiently.',
      'Compression effectiveness depends on entropy: structured text usually compresses better than already-compressed media.',
      'This path is used for text, structured data, documents when practical, and unknown file types.',
    ],
  }
}
