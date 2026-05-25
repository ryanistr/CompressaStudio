import { useEffect, useRef, useState, type CSSProperties, type WheelEvent } from 'react'
import { createPortal } from 'react-dom'
import type { LogEntry } from '../types'

interface LogDropdownProps {
  logs: LogEntry[]
}

const MENU_GAP = 14
const VIEWPORT_PADDING = 16
const MENU_MAX_WIDTH = 420
const MENU_MAX_HEIGHT = 420
const MENU_MIN_HEIGHT = 180

export function LogDropdown({ logs }: LogDropdownProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [menuGeometry, setMenuGeometry] = useState<MenuGeometry | null>(null)
  const dropdownRef = useRef<HTMLDivElement>(null)
  const triggerRef = useRef<HTMLButtonElement>(null)
  const menuRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!isOpen) {
      return
    }

    function handlePointerDown(event: PointerEvent) {
      const target = event.target as Node
      if (!dropdownRef.current?.contains(target) && !menuRef.current?.contains(target)) {
        setIsOpen(false)
        setMenuGeometry(null)
      }
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        setIsOpen(false)
        setMenuGeometry(null)
      }
    }

    document.addEventListener('pointerdown', handlePointerDown)
    document.addEventListener('keydown', handleKeyDown)

    return () => {
      document.removeEventListener('pointerdown', handlePointerDown)
      document.removeEventListener('keydown', handleKeyDown)
    }
  }, [isOpen])

  useEffect(() => {
    if (!isOpen) {
      return
    }

    const frameIds: number[] = []

    function updateMenuGeometry() {
      const trigger = triggerRef.current
      if (!trigger) {
        return
      }

      const rect = trigger.getBoundingClientRect()
      const width = Math.min(MENU_MAX_WIDTH, window.innerWidth - VIEWPORT_PADDING * 2)
      const maxHeight = Math.max(
        MENU_MIN_HEIGHT,
        Math.min(MENU_MAX_HEIGHT, window.innerHeight - rect.bottom - MENU_GAP - VIEWPORT_PADDING),
      )
      const left = Math.min(
        Math.max(VIEWPORT_PADDING, rect.right - width),
        window.innerWidth - width - VIEWPORT_PADDING,
      )
      const top = rect.bottom + MENU_GAP

      setMenuGeometry({
        left,
        maxHeight,
        top,
        width,
      })
    }

    updateMenuGeometry()
    frameIds.push(window.requestAnimationFrame(updateMenuGeometry))

    window.addEventListener('resize', updateMenuGeometry)
    window.addEventListener('scroll', updateMenuGeometry, true)

    return () => {
      frameIds.forEach((frameId) => window.cancelAnimationFrame(frameId))
      window.removeEventListener('resize', updateMenuGeometry)
      window.removeEventListener('scroll', updateMenuGeometry, true)
    }
  }, [isOpen])

  const menuStyle: CSSProperties | undefined = menuGeometry
    ? {
        left: menuGeometry.left,
        maxHeight: menuGeometry.maxHeight,
        top: menuGeometry.top,
        width: menuGeometry.width,
      }
    : undefined

  function handleLogWheel(event: WheelEvent<HTMLUListElement>) {
    event.stopPropagation()

    const list = event.currentTarget
    const cannotScroll = list.scrollHeight <= list.clientHeight
    const atTop = list.scrollTop <= 0
    const atBottom = Math.ceil(list.scrollTop + list.clientHeight) >= list.scrollHeight

    if (cannotScroll || (atTop && event.deltaY < 0) || (atBottom && event.deltaY > 0)) {
      event.preventDefault()
    }
  }

  const menu =
    isOpen && menuGeometry
      ? createPortal(
          <div
            className="glass-panel log-dropdown-menu"
            ref={menuRef}
            role="dialog"
            aria-label="Operation history"
            style={menuStyle}
          >
            <header className="log-dropdown-header">
              <div>
                <h3>Operation History</h3>
                <p>{logs.length} entries</p>
              </div>
            </header>

            {logs.length === 0 ? (
              <div className="log-dropdown-empty">
                <span>No logs yet.</span>
              </div>
            ) : (
              <ul className="log-list log-dropdown-list" onWheel={handleLogWheel}>
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
          </div>,
          document.body,
        )
      : null

  return (
    <div className="log-dropdown-container" ref={dropdownRef}>
      <button
        type="button"
        className="topbar-control log-trigger"
        ref={triggerRef}
        aria-expanded={isOpen}
        aria-haspopup="dialog"
        onClick={() => {
          if (isOpen) {
            setIsOpen(false)
            setMenuGeometry(null)
            return
          }
          setIsOpen(true)
        }}
      >
        Logs
        <span className="log-count">{logs.length}</span>
      </button>
      {menu}
    </div>
  )
}

interface MenuGeometry {
  left: number
  maxHeight: number
  top: number
  width: number
}
