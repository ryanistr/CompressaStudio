import { useEffect, useId, useRef, useState, type CSSProperties } from 'react'
import { createPortal } from 'react-dom'

interface SelectOption<TValue extends string> {
  value: TValue
  label: string
}

const VIEWPORT_PADDING = 12
const MENU_GAP = 10
const MAX_MENU_HEIGHT = 320
const ITEM_HEIGHT = 49
const MENU_PADDING = 16
const MIN_DROPDOWN_HEIGHT = 220
const MIN_WIDTH = 140

interface MenuGeometry {
  left: number
  maxHeight: number
  opensAbove: boolean
  top: number
  width: number
}

interface CustomSelectProps<TValue extends string> {
  id?: string
  label: string
  value: TValue
  options: Array<SelectOption<TValue>>
  disabled?: boolean
  onChange: (value: TValue) => void
}

/**
 * A custom select dropdown component mimicking native select behavior but with custom styling.
 */
export function CustomSelect<TValue extends string>({
  id,
  label,
  value,
  options,
  disabled = false,
  onChange,
}: CustomSelectProps<TValue>) {
  const generatedId = useId()
  const controlId = id ?? generatedId
  const listboxId = `${controlId}-listbox`
  const [open, setOpen] = useState(false)
  const [menuGeometry, setMenuGeometry] = useState<MenuGeometry | null>(null)
  const rootRef = useRef<HTMLDivElement>(null)
  const triggerRef = useRef<HTMLButtonElement>(null)
  const menuRef = useRef<HTMLDivElement>(null)
  const selectedOption = options.find((option) => option.value === value) ?? options[0]
  const isOpen = open && !disabled

  useEffect(() => {
    if (!isOpen) {
      return
    }

    function handlePointerDown(event: PointerEvent) {
      const target = event.target as Node
      if (!rootRef.current?.contains(target) && !menuRef.current?.contains(target)) {
        setOpen(false)
        setMenuGeometry(null)
      }
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        setOpen(false)
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
      const estimatedHeight = Math.min(MAX_MENU_HEIGHT, options.length * ITEM_HEIGHT + MENU_PADDING)
      const menuHeight = menuRef.current?.offsetHeight ?? estimatedHeight
      const spaceBelow = window.innerHeight - rect.bottom - VIEWPORT_PADDING
      const spaceAbove = rect.top - VIEWPORT_PADDING
      const opensAbove = spaceBelow < Math.min(menuHeight, MIN_DROPDOWN_HEIGHT) && spaceAbove > spaceBelow
      const availableSpace = opensAbove ? spaceAbove : spaceBelow
      const maxHeight = Math.max(MIN_WIDTH, Math.min(availableSpace - MENU_GAP, MAX_MENU_HEIGHT))
      const renderedHeight = Math.min(menuHeight, maxHeight)
      const maxLeft = window.innerWidth - rect.width - VIEWPORT_PADDING
      const left = Math.min(Math.max(VIEWPORT_PADDING, rect.left), Math.max(VIEWPORT_PADDING, maxLeft))
      const top = opensAbove
        ? Math.max(VIEWPORT_PADDING, rect.top - MENU_GAP - renderedHeight)
        : Math.min(rect.bottom + MENU_GAP, window.innerHeight - VIEWPORT_PADDING - renderedHeight)

      setMenuGeometry({
        left,
        maxHeight,
        opensAbove,
        top,
        width: rect.width,
      })
    }

    updateMenuGeometry()
    frameIds.push(window.requestAnimationFrame(updateMenuGeometry))

    window.addEventListener('scroll', updateMenuGeometry, true)
    window.addEventListener('resize', updateMenuGeometry)

    return () => {
      frameIds.forEach((frameId) => window.cancelAnimationFrame(frameId))
      window.removeEventListener('scroll', updateMenuGeometry, true)
      window.removeEventListener('resize', updateMenuGeometry)
    }
  }, [isOpen, options.length])

  const menuStyle: CSSProperties | undefined = menuGeometry
    ? {
        left: menuGeometry.left,
        maxHeight: menuGeometry.maxHeight,
        top: menuGeometry.top,
        width: menuGeometry.width,
      }
    : undefined

  const menu =
    isOpen && menuGeometry
      ? createPortal(
          <div
            className={`select-menu select-menu-open ${
              menuGeometry.opensAbove ? 'select-menu-above' : ''
            }`.trim()}
            id={listboxId}
            ref={menuRef}
            role="listbox"
            aria-labelledby={`${controlId}-label`}
            style={menuStyle}
          >
            {options.map((option) => (
              <button
                type="button"
                className={`select-option ${
                  option.value === value ? 'select-option-active' : ''
                }`.trim()}
                key={option.value}
                role="option"
                aria-selected={option.value === value}
                onClick={() => {
                  onChange(option.value)
                  setOpen(false)
                  setMenuGeometry(null)
                }}
              >
                {option.label}
              </button>
            ))}
          </div>,
          document.body,
        )
      : null

  return (
    <div className={`custom-select ${isOpen ? 'custom-select-open' : ''}`.trim()} ref={rootRef}>
      <label className="field-label" id={`${controlId}-label`}>
        {label}
      </label>
      <button
        type="button"
        className="select-trigger"
        ref={triggerRef}
        disabled={disabled}
        aria-controls={listboxId}
        aria-haspopup="listbox"
        aria-expanded={isOpen}
        aria-labelledby={`${controlId}-label ${controlId}-value`}
        onClick={() => {
          if (isOpen) {
            setOpen(false)
            setMenuGeometry(null)
            return
          }

          setOpen(true)
        }}
      >
        <span id={`${controlId}-value`}>{selectedOption.label}</span>
        <span className="select-chevron" aria-hidden="true" />
      </button>
      {menu}
    </div>
  )
}
