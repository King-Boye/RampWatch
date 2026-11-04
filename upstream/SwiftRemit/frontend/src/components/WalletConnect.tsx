import { FC, useState } from 'react'
import { isConnected, getAddress, requestAccess } from '@stellar/freighter-api'
import { clearRecipients } from '../utils/recipientStorage'

interface WalletConnectProps {
  walletAddress: string | null
  setWalletAddress: (address: string | null) => void
}

type WalletErrorKind = 'notInstalled' | 'userRejected' | 'networkMismatch' | 'generic'

function classifyError(error: unknown): WalletErrorKind {
  const message = error instanceof Error ? error.message.toLowerCase() : ''
  if (message.includes('reject') || message.includes('denied')) return 'userRejected'
  if (message.includes('network') || message.includes('testnet') || message.includes('mainnet')) return 'networkMismatch'
  if (message.includes('not installed') || message.includes('freighter')) return 'notInstalled'
  return 'generic'
}

const WalletConnect: FC<WalletConnectProps> = ({ walletAddress, setWalletAddress }) => {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [errorKind, setErrorKind] = useState<WalletErrorKind | null>(null)

  const connectWallet = async (): Promise<void> => {
    setLoading(true)
    setError(null)
    setErrorKind(null)
    try {
      const connectedResponse = await isConnected()
      if (!connectedResponse.isConnected) {
        setErrorKind('notInstalled')
        setError('Freighter wallet not found. Please install it.')
        return
      }
      await requestAccess()
      const addressResponse = await getAddress()
      if (addressResponse.error) {
        throw new Error(addressResponse.error.message || 'Failed to get address')
      }
      setWalletAddress(addressResponse.address)
    } catch (err) {
      setErrorKind(classifyError(err))
      setError(err instanceof Error ? err.message : 'Failed to connect wallet')
    } finally {
      setLoading(false)
    }
  }

  const disconnectWallet = (): void => {
    // Preserve the legacy PII cleanup on wallet disconnect.
    clearRecipients(walletAddress)
    setWalletAddress(null)
  }

  if (walletAddress) {
    return (
      <div className="wallet-connected">
        <p>Connected: {walletAddress.slice(0, 8)}...{walletAddress.slice(-8)}</p>
        <button onClick={disconnectWallet} className="btn-secondary">Disconnect</button>
      </div>
    )
  }

  return (
    <div className="wallet-connect">
      <button onClick={connectWallet} disabled={loading} className="btn-primary" data-testid="connect-wallet-btn">
        {loading ? 'Connecting...' : 'Connect Freighter Wallet'}
      </button>
      {error && (
        <div className="error-container" role="alert">
          <p className="error">{errorKind === 'userRejected' ? 'Connection rejected. Please try again.' : error}</p>
          {errorKind === 'networkMismatch' && <p className="hint">Please switch your Freighter wallet to Testnet or Mainnet.</p>}
          {errorKind === 'notInstalled' && (
            <a href="https://www.freighter.app/" target="_blank" rel="noopener noreferrer" className="btn-link">Install Freighter Wallet</a>
          )}
          {(errorKind === 'userRejected' || errorKind === 'generic') && (
            <button onClick={connectWallet} className="btn-link retry-btn">
              {errorKind === 'userRejected' ? 'Retry Connection' : 'Retry'}
            </button>
          )}
        </div>
      )}
      {!error && (
        <p className="hint">
          Don't have Freighter? <a href="https://www.freighter.app/" target="_blank" rel="noopener noreferrer">Install it here</a>
        </p>
      )}
    </div>
  )
}

export default WalletConnect
