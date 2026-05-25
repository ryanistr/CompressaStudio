import { GlassCard } from './GlassCard'
import type { StatsSnapshot } from '../types'

interface StatsPanelProps {
  stats: StatsSnapshot
}

export function StatsPanel({ stats }: StatsPanelProps) {
  return (
    <GlassCard
      id="statistics"
      title="Statistics"
      subtitle="Compression ratio = (output_size / original_size) x 100, space saved = ((original_size - output_size) / original_size) x 100."
      className="stats-panel"
    >
      <div className="panel-content">
        <StatRow label="Method" value={stats.method ?? '-'} mono={false} />
        <StatRow
          label="Lossless"
          value={stats.lossless === undefined ? '-' : stats.lossless ? 'Yes' : 'No'}
          mono={false}
        />
        <StatRow label="Original Size" value={formatSize(stats.originalSize)} mono={false} />
        <StatRow label="Output Size" value={formatSize(stats.outputSize)} mono={false} />
        <StatRow label="Saved Size" value={formatSignedSize(stats.savedSize)} mono={false} />
        <StatRow label="Compression Ratio" value={formatPercent(stats.compressionRatio)} mono={false} />
        <StatRow label="Space Saved" value={formatPercent(stats.spaceSaved)} mono={false} />
        <StatRow
          label="Elapsed Time"
          value={typeof stats.elapsedMs === 'number' ? `${stats.elapsedMs} ms` : '-'}
          mono={false}
        />
        <StatRow label="Output Path" value={stats.outputPath ?? '-'} mono />
        <StatRow label="Source SHA-256" value={stats.sourceSha256 ?? '-'} mono />
        <StatRow label="Output SHA-256" value={stats.outputSha256 ?? '-'} mono />
        <StatRow label="Expected SHA-256" value={stats.expectedSha256 ?? '-'} mono />
        <StatRow
          label="Integrity Check"
          value={
            stats.integrityMatch === undefined
              ? '-'
              : stats.integrityMatch
                ? 'Checksum match'
                : 'Checksum mismatch'
          }
          mono={false}
        />
      </div>
    </GlassCard>
  )
}

function StatRow({ label, value, mono = false }: { label: string; value: string; mono?: boolean }) {
  return (
    <div className="data-row">
      <span className="data-label">{label}</span>
      <span className={`data-value ${mono ? 'monospace' : ''}`.trim()}>{value}</span>
    </div>
  )
}

function formatSize(value: number | undefined): string {
  if (typeof value !== 'number') {
    return '-'
  }
  if (value < 1024) {
    return `${value} B`
  }
  const units = ['KB', 'MB', 'GB', 'TB']
  let current = value / 1024
  let unitIndex = 0
  while (current >= 1024 && unitIndex < units.length - 1) {
    current /= 1024
    unitIndex += 1
  }
  return `${current.toFixed(2)} ${units[unitIndex]}`
}

function formatPercent(value: number | undefined): string {
  return typeof value === 'number' ? `${value.toFixed(2)}%` : '-'
}

function formatSignedSize(value: number | undefined): string {
  if (typeof value !== 'number') {
    return '-'
  }
  return `${value >= 0 ? '' : '-'}${formatSize(Math.abs(value))}`
}
