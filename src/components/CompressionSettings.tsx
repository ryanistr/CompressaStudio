import { CustomSelect } from './CustomSelect'
import { FileTypeBadge } from './FileTypeBadge'
import { GlassCard } from './GlassCard'
import { Stepper } from './Stepper'
import type {
  CompressionRequest,
  FileCategory,
  GenericAlgorithm,
  PdfPreset,
  QualityPreset,
  ToolAvailability,
} from '../types'

interface CompressionSettingsProps {
  category: FileCategory
  request: CompressionRequest
  tools: ToolAvailability | null
  disabled: boolean
  onChange: (next: CompressionRequest) => void
}

const qualityOptions: Array<{ value: QualityPreset; label: string }> = [
  { value: 'highQuality', label: 'High quality' },
  { value: 'balanced', label: 'Balanced' },
  { value: 'smallSize', label: 'Small size' },
]

const pdfOptions: Array<{ value: PdfPreset; label: string }> = [
  { value: 'screen', label: 'Screen' },
  { value: 'ebook', label: 'Ebook' },
  { value: 'print', label: 'Print' },
]

const genericAlgorithms: Array<{ value: GenericAlgorithm; label: string }> = [
  { value: 'zstd', label: 'Zstd dictionary/statistical coding' },
  { value: 'rle', label: 'Run-Length Encoding (RLE)' },
  { value: 'shannon', label: 'Shannon Coding' },
  { value: 'shannonFano', label: 'Shannon-Fano Coding' },
  { value: 'huffman', label: 'Huffman Coding' },
  { value: 'lz77', label: 'Lempel-Ziv 77 (LZ77)' },
  { value: 'lz78', label: 'Lempel-Ziv 78 (LZ78)' },
  { value: 'lzw', label: 'Lempel-Ziv-Welch (LZW)' },
  { value: 'arithmetic', label: 'Arithmetic Coding (Optimal Entropy)' },
]
/**
 * Displays user-configurable settings based on the selected file category.
 */
export function CompressionSettings({
  category,
  request,
  tools,
  disabled,
  onChange,
}: CompressionSettingsProps) {
  return (
    <GlassCard
      id="compression-mode"
      title="Compression Mode"
      subtitle="Compression mode is selected automatically from the detected file type."
      className="settings-panel"
      aside={<FileTypeBadge category={category} />}
    >
      <div className="settings-split">
        <div className="panel-content">
          {category === 'pdf' ? (
            <CustomSelect
              id="preset-control"
              label="Preset"
              value={request.pdfPreset ?? 'ebook'}
              disabled={disabled}
              options={pdfOptions}
              onChange={(pdfPreset) => onChange({ ...request, pdfPreset })}
            />
          ) : (
            <CustomSelect
              id="preset-control"
              label="Preset"
              value={request.qualityPreset ?? 'balanced'}
              disabled={disabled}
              options={qualityOptions}
              onChange={(qualityPreset) => onChange({ ...request, qualityPreset })}
            />
          )}
        </div>

        <div className="panel-content">
          <span className="field-label field-label-static">Advanced Settings</span>
          {category === 'image' && (
            <>
              <Stepper
                id="resize-control"
                label="Resize Percentage"
                min={10}
                max={100}
                step={5}
                suffix="%"
                value={request.resizePercent ?? 100}
                disabled={disabled}
                onChange={(resizePercent) => onChange({ ...request, resizePercent })}
              />
              <CustomSelect
                id="image-encoder"
                label="JPEG Encoder"
                value={request.useNativeJpeg ? 'native' : 'standard'}
                disabled={disabled}
                options={[
                  { value: 'standard', label: 'Standard (image crate)' },
                  { value: 'native', label: 'Native (From-Scratch Rust)' },
                ]}
                onChange={(val) => onChange({ ...request, useNativeJpeg: val === 'native' })}
              />
            </>
          )}

          {category === 'video' && (
            <ToolNotice
              title="ffmpeg availability"
              available={tools?.ffmpeg.available ?? false}
              detail={tools?.ffmpeg.detail ?? 'Checking ffmpeg...'}
            />
          )}

          {category === 'pdf' && (
            <ToolNotice
              title="Ghostscript availability"
              available={tools?.ghostscript.available ?? false}
              detail={tools?.ghostscript.detail ?? 'Checking Ghostscript...'}
            />
          )}

          {category === 'generic' && (
            <>
              <CustomSelect
                id="generic-algorithm"
                label="Algorithm Choice"
                value={request.genericAlgorithm ?? 'zstd'}
                disabled={disabled}
                options={genericAlgorithms}
                onChange={(genericAlgorithm) =>
                  onChange({
                    ...request,
                    genericAlgorithm,
                  })
                }
              />

              {(request.genericAlgorithm ?? 'zstd') === 'zstd' && (
                <Stepper
                  id="zstd-level"
                  label="Zstd Level"
                  min={1}
                  max={19}
                  value={request.zstdLevel ?? 6}
                  disabled={disabled}
                  onChange={(zstdLevel) => onChange({ ...request, zstdLevel })}
                />
              )}
            </>
          )}
        </div>
      </div>
    </GlassCard>
  )
}

function ToolNotice({
  title,
  available,
  detail,
}: {
  title: string
  available: boolean
  detail: string
}) {
  return (
    <div className={`tool-pill ${available ? 'tool-ok' : 'tool-missing'}`}>
      <strong>{title}</strong>
      <span>{available ? 'Available' : 'Missing'}</span>
      <small>{detail}</small>
    </div>
  )
}
