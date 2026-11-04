import { useState, useEffect } from 'react'

function MetricsDashboard({ apiUrl, apiKey }) {
  const [metrics, setMetrics] = useState(null)
  const [alerts, setAlerts] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [alertFilter, setAlertFilter] = useState({ status: '', priority: '' })

  useEffect(() => {
    fetchMetrics()
    fetchAlerts()
  }, [])

  useEffect(() => {
    fetchAlerts()
  }, [alertFilter])

  const fetchMetrics = async () => {
    try {
      const response = await fetch(`${apiUrl}/metrics`, {
        headers: { 'X-API-Key': apiKey }
      })
      if (response.ok) {
        const data = await response.json()
        setMetrics(data)
      } else {
        setError('Failed to fetch metrics')
      }
    } catch (err) {
      setError('Failed to connect to API')
    } finally {
      setLoading(false)
    }
  }

  const fetchAlerts = async () => {
    try {
      const params = new URLSearchParams()
      if (alertFilter.status) params.append('status', alertFilter.status)
      if (alertFilter.priority) params.append('priority', alertFilter.priority)
      params.append('limit', '50')

      const response = await fetch(`${apiUrl}/alerts?${params}`, {
        headers: { 'X-API-Key': apiKey }
      })
      if (response.ok) {
        const data = await response.json()
        setAlerts(data.alerts)
      }
    } catch (err) {
      console.error('Failed to fetch alerts:', err)
    }
  }

  if (loading) {
    return (
      <div className="card">
        <div className="loading">
          <div className="spinner"></div>
          <p>Loading metrics...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="card">
        <div className="error">{error}</div>
      </div>
    )
  }

  return (
    <div className="metrics-dashboard">
      <div className="card">
        <h2>System Metrics</h2>
        
        {metrics && (
          <>
            <div className="metrics-grid">
              <div className="metric-card">
                <div className="metric-value">{metrics.alerts.total || 0}</div>
                <div className="metric-label">Total Alerts</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{metrics.alerts.pending || 0}</div>
                <div className="metric-label">Pending</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{metrics.alerts.reviewed || 0}</div>
                <div className="metric-label">Reviewed</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{metrics.alerts.resolved || 0}</div>
                <div className="metric-label">Resolved</div>
              </div>
            </div>

            <div className="thresholds">
              <h3>Decision Thresholds</h3>
              <div className="threshold-grid">
                <div className="threshold-item">
                  <span className="threshold-label">Approve Threshold:</span>
                  <span className="threshold-value">{(metrics.thresholds.approve * 100).toFixed(0)}%</span>
                </div>
                <div className="threshold-item">
                  <span className="threshold-label">Block Threshold:</span>
                  <span className="threshold-value">{(metrics.thresholds.block * 100).toFixed(0)}%</span>
                </div>
              </div>
            </div>
          </>
        )}
      </div>

      <div className="card">
        <h2>Recent Alerts</h2>
        
        <div className="alert-filters">
          <select 
            value={alertFilter.status} 
            onChange={(e) => setAlertFilter(prev => ({ ...prev, status: e.target.value }))}
          >
            <option value="">All Status</option>
            <option value="pending">Pending</option>
            <option value="reviewed">Reviewed</option>
            <option value="resolved">Resolved</option>
          </select>

          <select 
            value={alertFilter.priority} 
            onChange={(e) => setAlertFilter(prev => ({ ...prev, priority: e.target.value }))}
          >
            <option value="">All Priority</option>
            <option value="high">High</option>
            <option value="medium">Medium</option>
            <option value="low">Low</option>
          </select>
        </div>

        <div className="alerts-list">
          {alerts.length === 0 ? (
            <p className="no-alerts">No alerts found</p>
          ) : (
            alerts.map(alert => (
              <div key={alert.alert_id} className={`alert-item priority-${alert.priority}`}>
                <div className="alert-header">
                  <span className="alert-id">{alert.alert_id}</span>
                  <span className={`alert-priority ${alert.priority}`}>
                    {alert.priority.toUpperCase()}
                  </span>
                  <span className={`alert-status ${alert.status}`}>
                    {alert.status}
                  </span>
                </div>
                <div className="alert-body">
                  <p><strong>Transaction:</strong> {alert.transaction_id}</p>
                  <p><strong>Explanation:</strong> {alert.explanation}</p>
                  <p className="alert-time">
                    Created: {new Date(alert.created_at).toLocaleString()}
                  </p>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  )
}

export default MetricsDashboard
