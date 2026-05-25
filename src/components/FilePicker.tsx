import { useState } from 'react'
import { FileTypeBadge } from './FileTypeBadge'
import { GlassCard } from './GlassCard'
import type { FileInfo, FileCategory } from '../types'

interface FilePickerProps {
  selectedFile: FileInfo | null
  selectedPath: string | null
  activeCategory?: FileCategory
  onOverrideCategory?: (category: FileCategory | null) => void
}

export function FilePicker({ selectedFile, selectedPath, activeCategory, onOverrideCategory }: FilePickerProps) {
  const [showOverride, setShowOverride] = useState(false)
  return (
    <GlassCard
      id="file-selection"
      title="File Selection"
      subtitle="The app detects extension and routes the file to the matching compression pipeline."
      className="file-panel"
      aside={selectedFile ? <FileTypeBadge category={selectedFile.category} /> : null}
    >
      {selectedFile ? (
        <div className="panel-content">
          <div className="data-row">
            <span className="data-label">Selected File</span>
            <span className="data-value">{selectedFile.fileName}</span>
          </div>
          <div className="data-row">
            <span className="data-label">Selected Path</span>
            <span className="data-value monospace">{selectedFile.path}</span>
          </div>
          <div className="data-row">
            <span className="data-label">Detected Extension</span>
            <span className="data-value">{selectedFile.extension || 'none'}</span>
          </div>
          <div className="data-row">
            <span className="data-label">Detected Category</span>
            <span className="data-value">{selectedFile.category}</span>
          </div>
          {activeCategory && onOverrideCategory && (
            <div className="data-row" style={{ alignItems: 'center' }}>
              <span className="data-label">Active Mode</span>
              <div className="data-value" style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                <span className="capitalize" style={{ fontWeight: 600 }}>{activeCategory}</span>
                {activeCategory === selectedFile.category ? (
                  <button 
                    className="button-link"
                    style={{ fontSize: '12px', color: '#2563eb', textDecoration: 'underline', background: 'none', border: 'none', cursor: 'pointer' }}
                    onClick={() => setShowOverride(!showOverride)}
                  >
                    Not the correct type?
                  </button>
                ) : (
                  <button 
                    className="button-link"
                    style={{ fontSize: '12px', color: '#64748b', textDecoration: 'underline', background: 'none', border: 'none', cursor: 'pointer' }}
                    onClick={() => {
                      onOverrideCategory(null)
                      setShowOverride(false)
                    }}
                  >
                    Reset to detected
                  </button>
                )}
              </div>
            </div>
          )}
          {showOverride && activeCategory === selectedFile.category && onOverrideCategory && (
            <div className="data-row" style={{ background: 'rgba(255,255,255,0.5)', padding: '12px', borderRadius: '8px', marginTop: '4px' }}>
              <span className="data-label">Override</span>
              <div className="data-value" style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                {['image', 'video', 'pdf', 'generic'].map(cat => (
                  <button 
                    key={cat}
                    onClick={() => {
                      onOverrideCategory(cat as FileCategory)
                      setShowOverride(false)
                    }}
                    style={{
                      padding: '4px 8px',
                      borderRadius: '4px',
                      border: '1px solid #cbd5e1',
                      background: '#fff',
                      cursor: 'pointer',
                      fontSize: '13px'
                    }}
                  >
                    {cat}
                  </button>
                ))}
              </div>
            </div>
          )}
          <div className="data-row">
            <span className="data-label">Original Size</span>
            <span className="data-value">{formatSize(selectedFile.sizeBytes)}</span>
          </div>
          <div className="data-row">
            <span className="data-label">SHA-256</span>
            <span className="data-value monospace">{selectedFile.sha256}</span>
          </div>
        </div>
      ) : (
        <div className="empty-state">
          <span>No file selected yet.</span>
          <span className="muted-text">Choose an image, video, PDF, document, or generic file.</span>
          {selectedPath && <span className="monospace">{selectedPath}</span>}
        </div>
      )}
    </GlassCard>
  )
}

function formatSize(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`
  }

  const units = ['KB', 'MB', 'GB', 'TB']
  let value = bytes / 1024
  let unitIndex = 0

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024
    unitIndex += 1
  }

  return `${value.toFixed(2)} ${units[unitIndex]}`
}
