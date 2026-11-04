import { useState } from 'react'
import './App.css'
import TransactionForm from './components/TransactionForm'
import ResultDisplay from './components/ResultDisplay'
import MetricsDashboard from './components/MetricsDashboard'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000'
const API_KEY = import.meta.env.VITE_API_KEY || 'dev-api-key-change-in-production'

function App() {
  const [result, setResult] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const [activeTab, setActiveTab] = useState('test')

  const handleSubmit = async (transactionData) => {
    setLoading(true)
    setError(null)
    setResult(null)

    try {
      const response = await fetch(`${API_URL}/score`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-API-Key': API_KEY
        },
        body: JSON.stringify({ transaction: transactionData })
      })

      if (response.ok) {
        const data = await response.json()
        setResult(data)
      } else {
        const errorText = await response.text()
        setError(`Error: ${response.status} - ${errorText}`)
      }
    } catch (err) {
      setError(`Failed to connect to API. Make sure it's running at ${API_URL}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="app">
      <header className="header">
        <h1>🛡️ Fraud Detection System</h1>
        <p>Real-Time Transaction Analysis with AI</p>
      </header>

      <div className="container">
        <div className="tabs">
          <button 
            className={`tab ${activeTab === 'test' ? 'active' : ''}`}
            onClick={() => setActiveTab('test')}
          >
            Test Transaction
          </button>
          <button 
            className={`tab ${activeTab === 'metrics' ? 'active' : ''}`}
            onClick={() => setActiveTab('metrics')}
          >
            Metrics Dashboard
          </button>
        </div>

        {activeTab === 'test' ? (
          <div className="card">
            <h2>Test Transaction</h2>
            <p className="subtitle">Enter transaction details to analyze for fraud</p>

            <TransactionForm onSubmit={handleSubmit} loading={loading} />

            {loading && (
              <div className="loading">
                <div className="spinner"></div>
                <p>Analyzing transaction...</p>
              </div>
            )}

            {error && (
              <div className="error">
                {error}
              </div>
            )}

            {result && <ResultDisplay result={result} />}
          </div>
        ) : (
          <MetricsDashboard apiUrl={API_URL} apiKey={API_KEY} />
        )}
      </div>
    </div>
  )
}

export default App
