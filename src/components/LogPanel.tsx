import { GlassCard } from './GlassCard'
import type { LogEntry } from '../types'

interface LogPanelProps {
  logs: LogEntry[]
}

export function LogPanel({ logs }: LogPanelProps) {
  return (
    <GlassCard
      id="log-panel"
      title="Log Panel"
      subtitle="Operation history, backend messages, and error traces are collected here."
      className="log-panel"
    >
      {logs.length === 0 ? (
        <div className="empty-state">
          <span>No logs yet.</span>
        </div>
      ) : (
        <ul className="log-list">
          {logs.map((log) => (
            <li key={log.id} className={`log-item log-${log.level}`}>
              <div className="log-meta">
                <span>{log.timestamp}</span>
                <span>{log.level.toUpperCase()}</span>
              </div>
              <p>{log.message}</p>
            </li>
          ))}
        </ul>
      )}
    </GlassCard>
  )
}
