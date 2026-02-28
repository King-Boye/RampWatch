import { useState } from 'react'
import { signTransaction } from '@stellar/freighter-api'
import * as StellarSdk from '@stellar/stellar-sdk'

export default function CreateRemittance({ walletAddress, contractId }) {
  const [agentAddress, setAgentAddress] = useState('')
  const [amount, setAmount] = useState('')
  const [loading, setLoading] = useState(false)
  const [result, setResult] = useState(null)
  const [error, setError] = useState(null)

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)
    setError(null)
    setResult(null)

    try {
      if (!contractId) {
        throw new Error('Please enter a contract ID')
      }

      // Convert amount to stroops (7 decimals for USDC)
      const amountInStroops = Math.floor(parseFloat(amount) * 10000000)

      // This is a placeholder - you'll need to implement actual contract interaction
      // using Stellar SDK and the contract's WASM interface
      
      setResult({
        message: 'Remittance created successfully!',
        id: Math.floor(Math.random() * 1000), // Mock ID
        amount: amount,
        agent: agentAddress
      })

      // Reset form
      setAgentAddress('')
      setAmount('')
    } catch (err) {
      setError(err.message || 'Failed to create remittance')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="panel">
      <h2>Create Remittance</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label>Agent Address:</label>
          <input
            type="text"
            value={agentAddress}
            onChange={(e) => setAgentAddress(e.target.value)}
            placeholder="GXXXXXXX..."
            required
          />
        </div>

        <div className="form-group">
          <label>Amount (USDC):</label>
          <input
            type="number"
            step="0.01"
            min="0.01"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="100.00"
            required
          />
        </div>

        <button type="submit" disabled={loading} className="btn-primary">
          {loading ? 'Creating...' : 'Create Remittance'}
        </button>
      </form>

      {result && (
        <div className="success">
          <p>{result.message}</p>
          <p>Remittance ID: {result.id}</p>
        </div>
      )}

      {error && <div className="error">{error}</div>}
    </div>
  )
}
