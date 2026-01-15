import { useState, useEffect } from 'react'

const generateTransactionId = () => {
  const timestamp = new Date().getTime()
  return `tx_${timestamp}`
}

const scenarios = {
  normal: {
    amount: '50.00',
    userId: 'user_normal',
    deviceId: 'device_regular',
    label: '✅ Normal'
  },
  highAmount: {
    amount: '15000.00',
    userId: 'user_high_amount',
    deviceId: 'device_regular',
    label: '⚠️ High Amount'
  },
  suspicious: {
    amount: '500.00',
    userId: 'user_suspicious',
    deviceId: 'device_new',
    label: '🚨 Suspicious'
  },
  geoIssue: {
    amount: '200.00',
    userId: 'user_traveler',
    deviceId: 'device_regular',
    label: '🌍 Geo-Time'
  }
}

function TransactionForm({ onSubmit, loading }) {
  const [formData, setFormData] = useState({
    transactionId: generateTransactionId(),
    userId: 'user_123',
    merchantId: 'merchant_456',
    amount: '100.00',
    currency: 'USD',
    deviceId: 'device_789',
    ipAddress: '192.168.1.1'
  })

  const handleChange = (e) => {
    const { name, value } = e.target
    setFormData(prev => ({
      ...prev,
      [name]: value
    }))
  }

  const loadScenario = (scenarioKey) => {
    const scenario = scenarios[scenarioKey]
    setFormData(prev => ({
      ...prev,
      amount: scenario.amount,
      userId: scenario.userId,
      deviceId: scenario.deviceId
    }))
  }

  const handleSubmit = (e) => {
    e.preventDefault()
    
    const transactionData = {
      transaction_id: formData.transactionId,
      user_id: formData.userId,
      merchant_id: formData.merchantId,
      amount: parseFloat(formData.amount),
      currency: formData.currency,
      timestamp: new Date().toISOString(),
      device_id: formData.deviceId,
      ip_address: formData.ipAddress
    }

    onSubmit(transactionData)
    
    // Generate new transaction ID for next test
    setFormData(prev => ({
      ...prev,
      transactionId: generateTransactionId()
    }))
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className="quick-scenarios">
        {Object.entries(scenarios).map(([key, scenario]) => (
          <button
            key={key}
            type="button"
            className="btn btn-secondary"
            onClick={() => loadScenario(key)}
            disabled={loading}
          >
            {scenario.label}
          </button>
        ))}
      </div>

      <div className="form-grid">
        <div className="form-group">
          <label htmlFor="transactionId">Transaction ID</label>
          <input
            type="text"
            id="transactionId"
            name="transactionId"
            value={formData.transactionId}
            onChange={handleChange}
            required
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="userId">User ID</label>
          <input
            type="text"
            id="userId"
            name="userId"
            value={formData.userId}
            onChange={handleChange}
            required
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="merchantId">Merchant ID</label>
          <input
            type="text"
            id="merchantId"
            name="merchantId"
            value={formData.merchantId}
            onChange={handleChange}
            required
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="amount">Amount ($)</label>
          <input
            type="number"
            id="amount"
            name="amount"
            value={formData.amount}
            onChange={handleChange}
            step="0.01"
            min="0.01"
            required
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="currency">Currency</label>
          <select
            id="currency"
            name="currency"
            value={formData.currency}
            onChange={handleChange}
            required
            disabled={loading}
          >
            <option value="USD">USD</option>
            <option value="EUR">EUR</option>
            <option value="GBP">GBP</option>
            <option value="JPY">JPY</option>
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="deviceId">Device ID</label>
          <input
            type="text"
            id="deviceId"
            name="deviceId"
            value={formData.deviceId}
            onChange={handleChange}
            required
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="ipAddress">IP Address</label>
          <input
            type="text"
            id="ipAddress"
            name="ipAddress"
            value={formData.ipAddress}
            onChange={handleChange}
            required
            disabled={loading}
          />
        </div>
      </div>

      <button type="submit" className="btn btn-primary" disabled={loading}>
        🔍 Analyze Transaction
      </button>
    </form>
  )
}

export default TransactionForm
