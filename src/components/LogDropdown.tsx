import { useState, useRef, useEffect } from 'react'
import type { LogEntry } from '../types'

interface LogDropdownProps {
  logs: LogEntry[]
}

export function LogDropdown({ logs }: LogDropdownProps) {
  const [isOpen, setIsOpen] = useState(false)
  const dropdownRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside)
    }
    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [isOpen])

  return (
    <div className="log-dropdown-container" ref={dropdownRef} style={{ position: 'relative' }}>
      <button 
        className="workflow-link" 
        onClick={() => setIsOpen(!isOpen)}
        style={{ margin: 0, height: '48px', minHeight: '48px', fontSize: '13px', background: isOpen ? 'rgba(var(--white-rgb), 0.7)' : undefined }}
      >
        Logs
      </button>

      {isOpen && (
        <div 
          className="glass-panel" 
          style={{ 
            position: 'absolute', 
            top: '100%', 
            right: 0, 
            marginTop: '8px', 
            width: '400px', 
            maxHeight: '400px', 
            overflowY: 'auto', 
            borderRadius: '12px',
            zIndex: 1000,
            padding: '16px',
            boxShadow: 'var(--glass-shadow)'
          }}
        >
          <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', color: 'var(--ink)' }}>Operation History</h3>
          {logs.length === 0 ? (
            <div className="empty-state" style={{ minHeight: '100px', padding: '16px' }}>
              <span>No logs yet.</span>
            </div>
          ) : (
            <ul className="log-list" style={{ margin: 0, padding: 0, listStyle: 'none', display: 'grid', gap: '8px' }}>
              {logs.map((log) => (
                <li key={log.id} className={`log-item log-${log.level}`} style={{ padding: '12px', borderRadius: '8px', fontSize: '13px' }}>
                  <div className="log-meta" style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px', fontSize: '11px', color: 'var(--muted)', fontWeight: 'bold' }}>
                    <span>{log.timestamp}</span>
                    <span>{log.level.toUpperCase()}</span>
                  </div>
                  <p style={{ margin: 0, color: 'var(--ink-soft)', lineHeight: 1.4, wordBreak: 'break-word' }}>{log.message}</p>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  )
}
