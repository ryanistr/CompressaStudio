import { GlassCard } from './GlassCard'

interface ActionPanelProps {
  canCompress: boolean
  canDecompress: boolean
  isBusy: boolean
  onChooseFile: () => void
  onCompress: () => void
  onDecompress: () => void
  onClear: () => void
}

export function ActionPanel({
  canCompress,
  canDecompress,
  isBusy,
  onChooseFile,
  onCompress,
  onDecompress,
  onClear,
}: ActionPanelProps) {
  return (
    <GlassCard
      id="actions"
      title="Actions"
      subtitle="Compression runs in the matching backend module. Decompression is enabled only for supported lossless outputs."
      className="action-panel"
    >
      <div className="action-grid">
        <button type="button" className="btn btn-primary" disabled={isBusy} onClick={onChooseFile}>
          Choose File
        </button>
        <button type="button" className="btn" disabled={!canCompress} onClick={onCompress}>
          Compress
        </button>
        <button type="button" className="btn" disabled={!canDecompress} onClick={onDecompress}>
          Decompress
        </button>
        <button type="button" className="btn btn-muted" disabled={isBusy} onClick={onClear}>
          Clear
        </button>
      </div>
    </GlassCard>
  )
}
