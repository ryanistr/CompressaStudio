import type { PropsWithChildren, ReactNode } from 'react'

interface GlassCardProps extends PropsWithChildren {
  id?: string
  title: string
  subtitle?: string
  className?: string
  aside?: ReactNode
}

/**
 * A reusable glassmorphic card container component.
 */
export function GlassCard({ id, title, subtitle, className, aside, children }: GlassCardProps) {
  return (
    <section id={id} className={`glass-panel panel ${className ?? ''}`.trim()}>
      <header className="panel-header">
        <div>
          <h2>{title}</h2>
          {subtitle && <p className="panel-subtitle">{subtitle}</p>}
        </div>
        {aside}
      </header>
      {children}
    </section>
  )
}
