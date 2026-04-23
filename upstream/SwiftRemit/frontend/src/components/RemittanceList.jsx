import { useState, useEffect } from 'react'

export default function RemittanceList({ walletAddress, contractId }) {
  const [remittances, setRemittances] = useState([])
  const [loading, setLoading] = useState(false)

  // Mock data for demonstration
  useEffect(() => {
    if (contractId && walletAddress) {
      // In production, fetch from contract
      setRemittances([
        {
          id: 1,
          sender: walletAddress,
          agent: 'GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX',
          amount: 100.00,
          fee: 2.50,
          status: 'Pending',
          memo: null,
        }
      ])
    }
  }, [contractId, walletAddress])

  const getStatusColor = (status) => {
    switch (status) {
      case 'Pending': return '#ffa500'
      case 'Completed': return '#4caf50'
      case 'Cancelled': return '#f44336'
      default: return '#666'
    }
  }

  if (!contractId) {
    return null
  }

  return (
    <div className="panel remittance-list">
      <h2>Your Remittances</h2>
      
      {loading && <p>Loading...</p>}
      
      {!loading && remittances.length === 0 && (
        <p className="hint">No remittances found</p>
      )}

      {!loading && remittances.length > 0 && (
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>ID</th>
                <th>Agent</th>
                <th>Amount</th>
                <th>Fee</th>
                <th>Status</th>
                <th>Memo</th>
              </tr>
            </thead>
            <tbody>
              {remittances.map((r) => (
                <tr key={r.id}>
                  <td>{r.id}</td>
                  <td>{r.agent.slice(0, 8)}...{r.agent.slice(-8)}</td>
                  <td>${r.amount.toFixed(2)}</td>
                  <td>${r.fee.toFixed(2)}</td>
                  <td>
                    <span 
                      className="status-badge"
                      style={{ backgroundColor: getStatusColor(r.status) }}
                    >
                      {r.status}
                    </span>
                  </td>
                  <td>{r.memo || <span style={{ color: '#aaa' }}>—</span>}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
