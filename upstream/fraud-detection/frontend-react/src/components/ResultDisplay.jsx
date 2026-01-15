function ResultDisplay({ result }) {
  const { fraud_score, decision, explanation, processing_time_ms } = result

  const getScoreClass = (score) => {
    if (score < 0.5) return 'score-low'
    if (score < 0.85) return 'score-medium'
    return 'score-high'
  }

  const getDecisionClass = (decision) => {
    if (decision === 'approve') return 'result-approve'
    if (decision === 'review') return 'result-review'
    return 'result-block'
  }

  const getDecisionIcon = (decision) => {
    if (decision === 'approve') return '✅'
    if (decision === 'review') return '⚠️'
    return '🚨'
  }

  const scoreClass = getScoreClass(fraud_score)
  const decisionClass = getDecisionClass(decision)
  const decisionIcon = getDecisionIcon(decision)

  return (
    <div className={`result-card ${decisionClass}`}>
      <h3>{decisionIcon} Decision: {decision.toUpperCase()}</h3>
      
      <div className="fraud-score">
        <div className={`score-circle ${scoreClass}`}>
          {(fraud_score * 100).toFixed(1)}%
        </div>
        <p className="score-label">Fraud Score</p>
      </div>

      <div className="metrics">
        <div className="metric-box">
          <div className="metric-value">{(fraud_score * 100).toFixed(1)}%</div>
          <div className="metric-label">Fraud Score</div>
        </div>
        <div className="metric-box">
          <div className="metric-value">{decision}</div>
          <div className="metric-label">Decision</div>
        </div>
        <div className="metric-box">
          <div className="metric-value">{processing_time_ms.toFixed(2)}ms</div>
          <div className="metric-label">Processing Time</div>
        </div>
      </div>

      <div className="explanation">
        <h4>📝 Explanation</h4>
        <p>{explanation}</p>
      </div>
    </div>
  )
}

export default ResultDisplay
