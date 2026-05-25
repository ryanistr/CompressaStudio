import { invoke } from '@tauri-apps/api/core'

import { useEffect, useState } from 'react'
import { ActionPanel } from './components/ActionPanel'
import { AlgorithmExplanation } from './components/AlgorithmExplanation'
import { CompressionSettings } from './components/CompressionSettings'
import { FilePicker } from './components/FilePicker'
import { LogDropdown } from './components/LogDropdown'
import { UploadModal } from './components/UploadModal'
import { StatsPanel } from './components/StatsPanel'
import type {
  AppStatus,
  CompressionRequest,
  FileCategory,
  FileInfo,
  LogEntry,
  OperationResult,
  StatsSnapshot,
  ToolAvailability,
} from './types'
import { timestampNow, toMessage } from './utils'
import { STATUS_LABELS, DEFAULT_REQUEST } from './constants'
import type { ReactNode } from 'react'

const QUICK_START_ICONS: Record<string, ReactNode> = {
  auto: (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><path d="m9 15 2 2 4-4"/></svg>
  ),
  image: (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>
  ),
  video: (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m22 8-6 4 6 4V8Z"/><rect width="14" height="12" x="2" y="6" rx="2" ry="2"/></svg>
  ),
  pdf: (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><path d="M8 13h2a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2H8v9"/><path d="M15 17v-8h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-2"/></svg>
  ),
  generic: (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
  )
}

function App() {
  const [selectedPath, setSelectedPath] = useState<string | null>(null)
  const [selectedFile, setSelectedFile] = useState<FileInfo | null>(null)
  const [uploadModalMode, setUploadModalMode] = useState<FileCategory | 'auto' | null>(null)
  const [request, setRequest] = useState<CompressionRequest>(DEFAULT_REQUEST)
  const [status, setStatus] = useState<AppStatus>('idle')
  const [statusText, setStatusText] = useState('Ready for all-in-one compression.')
  const [stats, setStats] = useState<StatsSnapshot>({})
  const [tools, setTools] = useState<ToolAvailability | null>(null)
  const [logs, setLogs] = useState<LogEntry[]>([
    {
      id: crypto.randomUUID(),
      timestamp: timestampNow(),
      level: 'info',
      message: 'Compressa Studio initialized.',
    },
  ])

  const category: FileCategory = request.overrideCategory ?? selectedFile?.category ?? 'generic'
  const isBusy = status === 'compressing' || status === 'decompressing'
  const isCompressedArtifact = selectedFile?.isDecompressible ?? false
  const canCompress = Boolean(selectedPath) && !isBusy && !isCompressedArtifact
  const canDecompress = Boolean(selectedFile?.isDecompressible) && !isBusy

  function pushLog(level: LogEntry['level'], message: string) {
    setLogs((previous) => [
      {
        id: crypto.randomUUID(),
        timestamp: timestampNow(),
        level,
        message,
      },
      ...previous,
    ])
  }

  function handleError(error: unknown, context: string) {
    const msg = toMessage(error)
    setStatus('error')
    setStatusText(msg)
    pushLog('error', `${context}: ${msg}`)
  }

  useEffect(() => {
    async function loadInitialTools() {
      try {
        const result = await invoke<ToolAvailability>('get_tool_availability')
        setTools(result)
      } catch (error) {
        setLogs((previous) => [
          {
            id: crypto.randomUUID(),
            timestamp: timestampNow(),
            level: 'error',
            message: `Tool detection failed: ${toMessage(error)}`,
          },
          ...previous,
        ])
      }
    }

    void loadInitialTools()
  }, [])

  async function refreshFileInfo(path: string): Promise<FileInfo | null> {
    try {
      const info = await invoke<FileInfo>('get_file_info', { path })
      setSelectedFile(info)
      setTools(info.tools)
      return info
    } catch (error) {
      const messageText = toMessage(error)
      pushLog('error', `Failed to read file info: ${messageText}`)
      return null
    }
  }

  function handleChooseFile(mode?: FileCategory | 'auto') {
    setUploadModalMode(mode ?? 'auto')
  }

  function handleFileConfirmed(path: string, info: FileInfo) {
    setSelectedPath(path)
    setStatus('idle')
    setStatusText('File selected.')
    setUploadModalMode(null)

    setSelectedFile(info)
    setTools(info.tools)
    
    setStats({
      originalSize: info.sizeBytes,
      sourceSha256: info.sha256,
    })
    pushLog('success', `Selected ${info.fileName} as ${info.category} input.`)
  }

  async function handleCompress() {
    if (!selectedPath) {
      setStatus('error')
      setStatusText('No file selected.')
      pushLog('error', 'Compression blocked because no file is selected.')
      return
    }

    setStatus('compressing')
    setStatusText('Running compression pipeline...')
    pushLog('info', `Compression started for ${selectedPath}.`)

    try {
      const result = await invoke<OperationResult>('compress_file', {
        path: selectedPath,
        request,
      })

      setStats(toStats(result))
      setStatus('success')
      setStatusText(result.message)
      setSelectedPath(result.outputPath)
      await refreshFileInfo(result.outputPath)
      pushLog(
        'success',
        `${result.method} finished in ${result.elapsedMs} ms. Ratio ${result.compressionRatio.toFixed(2)}%.`,
      )
    } catch (error) {
      handleError(error, 'Compression failed')
    }
  }

  async function handleDecompress() {
    if (!selectedPath) {
      setStatus('error')
      setStatusText('No file selected.')
      pushLog('error', 'Decompression blocked because no file is selected.')
      return
    }

    setStatus('decompressing')
    setStatusText('Decompressing lossless output...')
    pushLog('info', `Decompression started for ${selectedPath}.`)

    try {
      const result = await invoke<OperationResult>('decompress_file', {
        path: selectedPath,
      })

      setStats(toStats(result))
      setStatus(result.integrityMatch === false ? 'error' : 'success')
      setStatusText(result.message)
      setSelectedPath(result.outputPath)
      await refreshFileInfo(result.outputPath)
      pushLog(
        result.integrityMatch === false ? 'error' : 'success',
        `${result.method} restored ${result.outputPath}.`,
      )
    } catch (error) {
      handleError(error, 'Decompression failed')
    }
  }

  function handleClear() {
    setSelectedPath(null)
    setSelectedFile(null)
    setStats({})
    setStatus('idle')
    setStatusText('Selection cleared.')
    setRequest(DEFAULT_REQUEST)
    pushLog('info', 'Cleared file selection and statistics.')
  }

  return (
    <div className="app-shell">
      <div className="ambient-scene" aria-hidden="true" />
      <main className="dashboard">
        <header className="glass-panel app-topbar">
          <div className="brand-section">
            <span className="brand-mark">CS</span>
            <span className="brand-copy">
              <span className="brand-title">Compressa Studio</span>
              <span className="brand-subtitle">Liquid compression workspace</span>
            </span>
          </div>
          <div className="topbar-actions">
            <div className={`status-badge status-${status}`}>
              <span className="status-dot" />
              <span>{STATUS_LABELS[status]}</span>
            </div>
            <LogDropdown logs={logs} />
          </div>
        </header>

        <section className={`glass-panel status-strip status-${status}`}>
          <span className="status-text">{statusText}</span>
        </section>

        {!selectedFile && (
          <section className="glass-panel quick-start-section">
            <h2>Quick Start</h2>
            <p className="quick-start-subtitle">Select a compression mode or let us auto-detect the optimal pipeline</p>
            <div className="quick-start-grid">
              <button type="button" className="quick-start-card" onClick={() => handleChooseFile('auto')}>
                <span className="qs-icon">
                  {QUICK_START_ICONS.auto}
                </span>
                <span className="qs-title">Auto Detect</span>
              </button>
              <button type="button" className="quick-start-card" onClick={() => handleChooseFile('image')}>
                <span className="qs-icon">
                  {QUICK_START_ICONS.image}
                </span>
                <span className="qs-title">Image</span>
              </button>
              <button type="button" className="quick-start-card" onClick={() => handleChooseFile('video')}>
                <span className="qs-icon">
                  {QUICK_START_ICONS.video}
                </span>
                <span className="qs-title">Video</span>
              </button>
              <button type="button" className="quick-start-card" onClick={() => handleChooseFile('pdf')}>
                <span className="qs-icon">
                  {QUICK_START_ICONS.pdf}
                </span>
                <span className="qs-title">PDF</span>
              </button>
              <button type="button" className="quick-start-card" onClick={() => handleChooseFile('generic')}>
                <span className="qs-icon">
                  {QUICK_START_ICONS.generic}
                </span>
                <span className="qs-title">Generic</span>
              </button>
            </div>
          </section>
        )}

        <section className="dashboard-grid-2col" aria-label="Compression workspace panels">
          <div className="dashboard-column dashboard-left">
            <FilePicker
              selectedFile={selectedFile}
              selectedPath={selectedPath}
              activeCategory={category}
              onOverrideCategory={(overrideCategory) =>
                setRequest({ ...request, overrideCategory })
              }
            />
            {selectedFile && (
              <CompressionSettings
                category={category}
                request={request}
                tools={tools}
                disabled={isBusy}
                onChange={setRequest}
              />
            )}
            <ActionPanel
              canCompress={canCompress}
              canDecompress={canDecompress}
              isBusy={isBusy}
              onChooseFile={() => handleChooseFile()}
              onCompress={handleCompress}
              onDecompress={handleDecompress}
              onClear={handleClear}
            />
          </div>

          <div className="dashboard-column dashboard-right">
            {status === 'success' && stats.outputSize !== undefined && <StatsPanel stats={stats} />}
            <AlgorithmExplanation category={category} request={request} />
          </div>
        </section>

        {uploadModalMode && (
          <UploadModal 
            mode={uploadModalMode} 
            onClose={() => setUploadModalMode(null)} 
            onFileConfirmed={handleFileConfirmed} 
            fetchFileInfo={refreshFileInfo} 
          />
        )}
      </main>
    </div>
  )
}

function toStats(result: OperationResult): StatsSnapshot {
  return {
    method: result.method,
    lossless: result.lossless,
    originalSize: result.originalSize,
    outputSize: result.outputSize,
    savedSize: result.savedSize,
    compressionRatio: result.compressionRatio,
    spaceSaved: result.spaceSaved,
    elapsedMs: result.elapsedMs,
    sourceSha256: result.sourceSha256,
    outputSha256: result.outputSha256,
    expectedSha256: result.expectedSha256 ?? undefined,
    integrityMatch: result.integrityMatch ?? undefined,
    outputPath: result.outputPath,
  }
}

export default App
