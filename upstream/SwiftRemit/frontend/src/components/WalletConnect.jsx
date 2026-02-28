import { useState } from 'react'
import { isConnected, getPublicKey, requestAccess } from '@stellar/freighter-api'

export default function WalletConnect({ walletAddress, setWalletAddress }) {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)

  const connectWallet = async () => {
    setLoading(true)
    setError(null)

    try {
      const connected = await isConnected()
      
      if (!connected) {
        setError('Freighter wallet not found. Please install it.')
        setLoading(false)
        return
      }

      await requestAccess()
      const publicKey = await getPublicKey()
      setWalletAddress(publicKey)
    } catch (err) {
      setError(err.message || 'Failed to connect wallet')
    } finally {
      setLoading(false)
    }
  }

  const disconnectWallet = () => {
    setWalletAddress(null)
  }

  if (walletAddress) {
    return (
      <div className="wallet-connected">
        <p>Connected: {walletAddress.slice(0, 8)}...{walletAddress.slice(-8)}</p>
        <button onClick={disconnectWallet} className="btn-secondary">
          Disconnect
        </button>
      </div>
    )
  }

  return (
    <div className="wallet-connect">
      <button 
        onClick={connectWallet} 
        disabled={loading}
        className="btn-primary"
      >
        {loading ? 'Connecting...' : 'Connect Freighter Wallet'}
      </button>
      {error && <p className="error">{error}</p>}
      {!error && (
        <p className="hint">
          Don't have Freighter? <a href="https://www.freighter.app/" target="_blank" rel="noopener noreferrer">Install it here</a>
        </p>
      )}
    </div>
  )
}
