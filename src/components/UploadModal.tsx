import { useState } from 'react'
import { open } from '@tauri-apps/plugin-dialog'
import { GlassCard } from './GlassCard'
import type { FileCategory, FileInfo } from '../types'

interface UploadModalProps {
  mode: FileCategory | 'auto'
  onClose: () => void
  onFileConfirmed: (path: string, info: FileInfo) => void
  fetchFileInfo: (path: string) => Promise<FileInfo | null>
}

export function UploadModal({ mode, onClose, onFileConfirmed, fetchFileInfo }: UploadModalProps) {
  const [isDragging, setIsDragging] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)
  const [mismatch, setMismatch] = useState<{ detected: FileCategory, path: string, info: FileInfo } | null>(null)

  const processPath = async (path: string) => {
    setIsProcessing(true)
    const info = await fetchFileInfo(path)
    setIsProcessing(false)
    if (!info) return

    if (mode !== 'auto' && info.category !== mode && info.category !== 'generic') {
      // Mismatch detected (ignoring generic fallback mismatches)
      setMismatch({ detected: info.category, path, info })
    } else {
      // It's fine, proceed
      onFileConfirmed(path, info)
    }
  }

  const handleBrowse = async () => {
    let filters = undefined
    if (mode === 'image') filters = [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
    else if (mode === 'video') filters = [{ name: 'Videos', extensions: ['mp4', 'mkv', 'avi', 'mov'] }]
    else if (mode === 'pdf') filters = [{ name: 'PDF', extensions: ['pdf'] }]

    const result = await open({
      title: 'Choose a file to compress',
      multiple: false,
      directory: false,
      filters
    })

    if (result && !Array.isArray(result)) {
      await processPath(result)
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
  }

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
    const file = e.dataTransfer.files[0] as File & { path?: string }
    if (file && file.path) {
      await processPath(file.path)
    }
  }

  return (
    <div style={{
      position: 'fixed', inset: 0, zIndex: 9999, 
      display: 'flex', alignItems: 'center', justifyContent: 'center',
      background: 'rgba(0,0,0,0.4)', backdropFilter: 'blur(4px)'
    }}>
      <div style={{ width: '100%', maxWidth: '500px', animation: 'scaleUp 0.2s ease-out' }}>
        <GlassCard title={`Upload ${mode === 'auto' ? 'File' : mode.charAt(0).toUpperCase() + mode.slice(1)}`} subtitle="Drag and drop your file or browse.">
          {mismatch ? (
            <div style={{ textAlign: 'center', padding: '20px' }}>
              <h3 style={{ color: 'var(--coral)', marginBottom: '12px' }}>Format Mismatch Detected</h3>
              <p style={{ marginBottom: '24px', fontSize: '14px', color: 'var(--ink)' }}>
                You selected <strong>{mode}</strong> mode, but dropped a <strong>{mismatch.detected}</strong> file.
                <br /><br />
                We will automatically adjust and compress this as a <strong>{mismatch.detected}</strong>.
              </p>
              <div style={{ display: 'flex', gap: '12px', justifyContent: 'center' }}>
                <button className="workflow-link" style={{ background: 'rgba(var(--white-rgb), 0.6)' }} onClick={() => setMismatch(null)}>
                  Cancel & Reselect
                </button>
                <button className="workflow-link" style={{ background: 'var(--accent)', color: 'white' }} onClick={() => onFileConfirmed(mismatch.path, mismatch.info)}>
                  Proceed as {mismatch.detected}
                </button>
              </div>
            </div>
          ) : (
            <div 
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              onDrop={handleDrop}
              style={{
                border: `2px dashed ${isDragging ? 'var(--accent)' : 'rgba(72, 104, 142, 0.3)'}`,
                borderRadius: '12px',
                padding: '40px 20px',
                textAlign: 'center',
                background: isDragging ? 'rgba(var(--blue-rgb), 0.05)' : 'transparent',
                transition: 'all 0.2s ease'
              }}
            >
              {isProcessing ? (
                <div style={{ color: 'var(--accent)', fontWeight: 'bold' }}>Processing file...</div>
              ) : (
                <>
                  <p style={{ marginBottom: '16px', color: 'var(--ink-soft)' }}>
                    {isDragging ? 'Drop it here!' : `Drag and drop your ${mode === 'auto' ? 'file' : mode} here`}
                  </p>
                  <p style={{ marginBottom: '24px', fontSize: '12px', color: 'var(--muted)' }}>
                    Or use the file browser
                  </p>
                  <div style={{ display: 'flex', gap: '12px', justifyContent: 'center' }}>
                    <button className="workflow-link" onClick={onClose}>Cancel</button>
                    <button className="workflow-link" style={{ background: 'var(--accent)', color: 'white' }} onClick={handleBrowse}>
                      Browse Files
                    </button>
                  </div>
                </>
              )}
            </div>
          )}
        </GlassCard>
      </div>
      <style>{`
        @keyframes scaleUp {
          from { transform: scale(0.95); opacity: 0; }
          to { transform: scale(1); opacity: 1; }
        }
      `}</style>
    </div>
  )
}
