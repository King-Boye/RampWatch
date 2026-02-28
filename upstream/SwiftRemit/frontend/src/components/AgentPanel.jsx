import { useState } from 'react'

export default function AgentPanel({ walletAddress, contractId }) {
  const [remittanceId, setRemittanceId] = useState('')
  const [loading, setLoading] = useState(false)
  const [result, setResult] = useState(null)
  const [error, setError] = useState(null)

  const handleConfirmPayout = async (e) => {
    e.preventDefault()
    setLoading(true)
    setError(null)
    setResult(null)

    try {
      if (!contractId) {
        throw new Error('Please enter a contract ID')
      }

      // Placeholder for actual contract interaction
      setResult({
        message: 'Payout confirmed successfully!',
        id: remittanceId
      })

      setRemittanceId('')
    } catch (err) {
      setError(err.message || 'Failed to confirm payout')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="panel">
      <h2>Agent Panel</h2>
      <p className="hint">Confirm fiat payouts after completing off-chain transfer</p>
      
      <form onSubmit={handleConfirmPayout}>
        <div className="form-group">
          <label>Remittance ID:</label>
          <input
            type="number"
            value={remittanceId}
            onChange={(e) => setRemittanceId(e.target.value)}
            placeholder="Enter remittance ID"
            required
          />
        </div>

        <button type="submit" disabled={loading} className="btn-primary">
          {loading ? 'Confirming...' : 'Confirm Payout'}
        </button>
      </form>

      {result && (
        <div className="success">
          <p>{result.message}</p>
        </div>
      )}

      {error && <div className="error">{error}</div>}
    </div>
  )
}
