import type { FileCategory } from '../types'
import { FILE_CATEGORY_LABELS } from '../constants'

/**
 * Renders a badge indicating the selected file category.
 */
interface FileTypeBadgeProps {
  category: FileCategory
}

export function FileTypeBadge({ category }: FileTypeBadgeProps) {
  return <span className={`category-badge category-${category}`}>{FILE_CATEGORY_LABELS[category]}</span>
}
