import type { FileCategory } from '../types'

interface FileTypeBadgeProps {
  category: FileCategory
}

export function FileTypeBadge({ category }: FileTypeBadgeProps) {
  return <span className={`category-badge category-${category}`}>{label[category]}</span>
}

const label: Record<FileCategory, string> = {
  image: 'Image Compression',
  video: 'Video Compression',
  pdf: 'PDF Compression',
  generic: 'Generic Lossless Compression',
}
