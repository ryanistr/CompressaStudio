import { useState, type DragEvent } from 'react'
import { open } from '@tauri-apps/plugin-dialog'
import { GlassCard } from './GlassCard'
import type { FileCategory, FileInfo } from '../types'
import { isTauriRuntime } from '../tauriRuntime'

interface UploadModalProps {
  mode: FileCategory | 'auto'
  onClose: () => void
  onFileConfirmed: (path: string, info: FileInfo) => void
  fetchFileInfo: (path: string) => Promise<FileInfo | null>
}

export function UploadModal({ mode, onClose, onFileConfirmed, fetchFileInfo }: UploadModalProps) {
  const [isDragging, setIsDragging] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)
  const [runtimeMessage, setRuntimeMessage] = useState<string | null>(null)
  const [mismatch, setMismatch] = useState<{ detected: FileCategory, path: string, info: FileInfo } | null>(null)
  const title = `Upload ${mode === 'auto' ? 'File' : mode.charAt(0).toUpperCase() + mode.slice(1)}`

  const processPath = async (path: string) => {
    setRuntimeMessage(null)
    setIsProcessing(true)
    const info = await fetchFileInfo(path)
    setIsProcessing(false)
    if (!info) return

    if (mode !== 'auto' && info.category !== mode && info.category !== 'generic') {
      setMismatch({ detected: info.category, path, info })
    } else {
      onFileConfirmed(path, info)
    }
  }

  const handleBrowse = async () => {
    if (!isTauriRuntime()) {
      setRuntimeMessage('File browsing is available inside the desktop app runtime.')
      return
    }

    let filters = undefined
    if (mode === 'image') filters = [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
    else if (mode === 'video') filters = [{ name: 'Videos', extensions: ['mp4', 'mkv', 'avi', 'mov'] }]
    else if (mode === 'pdf') filters = [{ name: 'PDF', extensions: ['pdf'] }]

    const result = await open({
      title: 'Choose a file to compress',
      multiple: false,
      directory: false,
      filters,
    })

    if (result && !Array.isArray(result)) {
      await processPath(result)
    }
  }

  const handleDragOver = (e: DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }

  const handleDragLeave = (e: DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
  }

  const handleDrop = async (e: DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
    const file = e.dataTransfer.files[0] as File & { path?: string }
    if (file && file.path) {
      await processPath(file.path)
    } else {
      setRuntimeMessage('Drag-and-drop file paths are available inside the desktop app runtime.')
    }
  }

  return (
    <div className="upload-modal-backdrop">
      <div className="upload-modal-shell" role="dialog" aria-modal="true" aria-label={title}>
        <GlassCard
          title={title}
          subtitle="Drag and drop your file or browse."
          className="upload-modal-card"
        >
          {mismatch ? (
            <div className="upload-modal-content upload-mismatch">
              <h3>Format Mismatch Detected</h3>
              <p>
                You selected <strong>{mode}</strong> mode, but dropped a <strong>{mismatch.detected}</strong> file.
                <br /><br />
                We will automatically adjust and compress this as a <strong>{mismatch.detected}</strong>.
              </p>
              <div className="upload-modal-actions">
                <button type="button" className="btn btn-muted upload-action-button" onClick={() => setMismatch(null)}>
                  Cancel & Reselect
                </button>
                <button type="button" className="btn btn-primary upload-action-button" onClick={() => onFileConfirmed(mismatch.path, mismatch.info)}>
                  Proceed as {mismatch.detected}
                </button>
              </div>
            </div>
          ) : (
            <div
              className={`upload-dropzone ${isDragging ? 'upload-dropzone-active' : ''}`.trim()}
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              onDrop={handleDrop}
            >
              {isProcessing ? (
                <div className="upload-processing">Processing file...</div>
              ) : (
                <>
                  <p className="upload-drop-title">
                    {isDragging ? 'Drop it here!' : `Drag and drop your ${mode === 'auto' ? 'file' : mode} here`}
                  </p>
                  <p className="upload-drop-subtitle">
                    Or use the file browser
                  </p>
                  {runtimeMessage && <p className="upload-runtime-note">{runtimeMessage}</p>}
                  <div className="upload-modal-actions">
                    <button type="button" className="btn btn-muted upload-action-button" onClick={onClose}>Cancel</button>
                    <button type="button" className="btn btn-primary upload-action-button" onClick={handleBrowse}>
                      Browse Files
                    </button>
                  </div>
                </>
              )}
            </div>
          )}
        </GlassCard>
      </div>
    </div>
  )
}
