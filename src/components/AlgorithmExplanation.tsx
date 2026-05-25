import { GlassCard } from './GlassCard'
import type { CompressionRequest, FileCategory } from '../types'
import { ALGORITHM_EXPLANATIONS, type AlgorithmExplanationData } from '../constants'

/**
 * Component that displays educational information about the chosen compression algorithm.
 */

interface AlgorithmExplanationProps {
  category: FileCategory
  request: CompressionRequest
}

export function AlgorithmExplanation({ category, request }: AlgorithmExplanationProps) {
  const explanation = explanationFor(category, request)

  return (
    <GlassCard
      id="algorithm-explanation"
      title="Algorithm Explanation"
      subtitle="This panel mirrors the compression concepts emphasized in the Materi slides."
      className="explanation-panel"
    >
      <div className="explanation-copy">
        <p>{explanation.summary}</p>
        <ul className="explanation-list">
          {explanation.points.map((point, i) => (
            <li key={i}>{point}</li>
          ))}
        </ul>
      </div>
    </GlassCard>
  )
}

function explanationFor(category: FileCategory, request: CompressionRequest): AlgorithmExplanationData {
  if (category === 'generic') {
    return ALGORITHM_EXPLANATIONS[request.genericAlgorithm ?? 'zstd']
  }
  return ALGORITHM_EXPLANATIONS[category]
}
