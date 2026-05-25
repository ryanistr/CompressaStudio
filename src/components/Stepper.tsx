interface StepperProps {
  id: string
  label: string
  value: number
  min: number
  max: number
  step?: number
  suffix?: string
  disabled?: boolean
  onChange: (value: number) => void
}

export function Stepper({
  id,
  label,
  value,
  min,
  max,
  step = 1,
  suffix,
  disabled = false,
  onChange,
}: StepperProps) {
  const normalizedValue = clamp(value, min, max)

  function update(nextValue: number) {
    onChange(clamp(nextValue, min, max))
  }

  return (
    <div className="stepper-field">
      <label className="field-label" htmlFor={id}>
        {label}
      </label>
      <div className="stepper-control">
        <button
          type="button"
          className="stepper-button"
          disabled={disabled || normalizedValue <= min}
          aria-label={`Decrease ${label}`}
          onClick={() => update(normalizedValue - step)}
        >
          -
        </button>
        <div className="stepper-value-wrap">
          <input
            id={id}
            className="stepper-value"
            type="text"
            inputMode="numeric"
            pattern="[0-9]*"
            value={normalizedValue}
            disabled={disabled}
            onChange={(event) => {
              const nextValue = Number(event.target.value)
              if (!Number.isNaN(nextValue)) {
                update(nextValue)
              }
            }}
            onBlur={() => update(normalizedValue)}
          />
          {suffix && <span className="stepper-suffix">{suffix}</span>}
        </div>
        <button
          type="button"
          className="stepper-button"
          disabled={disabled || normalizedValue >= max}
          aria-label={`Increase ${label}`}
          onClick={() => update(normalizedValue + step)}
        >
          +
        </button>
      </div>
    </div>
  )
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max)
}
