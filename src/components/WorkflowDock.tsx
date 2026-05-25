interface WorkflowDockProps {
  activeCategory: string
}

const workflowItems = [
  { href: '#file-selection', label: 'Source' },
  { href: '#compression-mode', label: 'Mode' },
  { href: '#actions', label: 'Run' },
  { href: '#statistics', label: 'Stats' },
  { href: '#algorithm-explanation', label: 'Learn' },
  { href: '#log-panel', label: 'Logs' },
]

export function WorkflowDock({ activeCategory }: WorkflowDockProps) {
  return (
    <nav className="workflow-dock glass-panel" aria-label="Compression workflow">
      <div className="workflow-context">
        <span className="workflow-context-label">Current route</span>
        <span className="workflow-context-value">{activeCategory}</span>
      </div>
      <div className="workflow-links">
        {workflowItems.map((item) => (
          <a className="workflow-link" href={item.href} key={item.href}>
            {item.label}
          </a>
        ))}
      </div>
    </nav>
  )
}
