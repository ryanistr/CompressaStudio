import { useState } from 'react'
import { FileTypeBadge } from './FileTypeBadge'
import { GlassCard } from './GlassCard'
import type { FileInfo, FileCategory } from '../types'
import { formatSize } from '../utils'
import { FILE_CATEGORIES } from '../constants'

/**
 * Component for selecting and overriding the compression file type.
 */
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
            <div className="data-row row-centered">
              <span className="data-label">Active Mode</span>
              <div className="data-value override-mode-container">
                <span className="active-category-value">{activeCategory}</span>
                {activeCategory === selectedFile.category ? (
                  <button
                    type="button"
                    className="override-link-active"
                    onClick={() => setShowOverride(!showOverride)}
                  >
                    Not the correct type?
                  </button>
                ) : (
                  <button
                    type="button"
                    className="override-link-muted"
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
            <div className="data-row override-panel">
              <span className="data-label">Override</span>
              <div className="data-value override-options">
                {FILE_CATEGORIES.map(cat => (
                  <button
                    type="button"
                    key={cat}
                    onClick={() => {
                      if (FILE_CATEGORIES.includes(cat as FileCategory)) {
                        onOverrideCategory(cat as FileCategory)
                      }
                      setShowOverride(false)
                    }}
                    className="override-btn"
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
