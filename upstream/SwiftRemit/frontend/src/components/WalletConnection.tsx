import React, { useMemo, useState } from 'react';
import './WalletConnection.css';

export type NetworkType = 'Testnet' | 'Mainnet';

interface WalletConnectionResult {
  publicKey: string;
  network?: NetworkType;
}

interface WalletConnectionProps {
  defaultNetwork?: NetworkType;
  onConnect?: () => Promise<WalletConnectionResult>;
  onDisconnect?: () => Promise<void> | void;
  onRequestSignature?: () => Promise<void>;
}

const DEFAULT_DEMO_KEY = 'GBZXN7PIRZGNMHGAU2LYGAZGQG4RYSQ3TB2T6O3COVGW6OLBDEQ2COFQ';

function truncatePublicKey(publicKey: string): string {
  if (publicKey.length <= 16) return publicKey;
  return `${publicKey.slice(0, 6)}...${publicKey.slice(-6)}`;
}

function isRejectedSignature(error: unknown): boolean {
  if (!error || typeof error !== 'object') {
    return false;
  }

  const message = 'message' in error ? String(error.message).toLowerCase() : '';
  const code = 'code' in error ? String(error.code) : '';

  return (
    code === '4001' ||
    message.includes('rejected') ||
    message.includes('denied') ||
    message.includes('declined')
  );
}

export const WalletConnection: React.FC<WalletConnectionProps> = ({
  defaultNetwork = 'Testnet',
  onConnect,
  onDisconnect,
  onRequestSignature,
}) => {
  const [connected, setConnected] = useState(false);
  const [publicKey, setPublicKey] = useState('');
  const [network, setNetwork] = useState<NetworkType>(defaultNetwork);
  const [isConnecting, setIsConnecting] = useState(false);
  const [isDisconnecting, setIsDisconnecting] = useState(false);
  const [isSigning, setIsSigning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const publicKeyText = useMemo(() => truncatePublicKey(publicKey), [publicKey]);

  const handleConnect = async () => {
    setError(null);
    setIsConnecting(true);

    try {
      const result = onConnect
        ? await onConnect()
        : { publicKey: DEFAULT_DEMO_KEY, network: defaultNetwork };

      setPublicKey(result.publicKey);
      setNetwork(result.network ?? defaultNetwork);
      setConnected(true);
    } catch (connectError) {
      setConnected(false);
      setError('Failed to connect wallet. Please try again.');
      console.error(connectError);
    } finally {
      setIsConnecting(false);
    }
  };

  const handleDisconnect = async () => {
    setError(null);
    setIsDisconnecting(true);

    try {
      if (onDisconnect) {
        await onDisconnect();
      }

      setConnected(false);
      setPublicKey('');
    } catch (disconnectError) {
      setError('Failed to disconnect wallet.');
      console.error(disconnectError);
    } finally {
      setIsDisconnecting(false);
    }
  };

  const handleSignature = async () => {
    setError(null);
    setIsSigning(true);

    try {
      if (onRequestSignature) {
        await onRequestSignature();
      }
    } catch (signatureError) {
      if (isRejectedSignature(signatureError)) {
        setError('Signature request was rejected.');
      } else {
        setError('Unable to complete signature request.');
      }
      console.error(signatureError);
    } finally {
      setIsSigning(false);
    }
  };

  return (
    <section className="wallet-card" aria-label="Wallet connection">
      <div className="wallet-row">
        <h2 className="wallet-title">Wallet</h2>
        <span
          className={`network-pill ${network === 'Mainnet' ? 'mainnet' : 'testnet'}`}
          aria-label={`Network: ${network}`}
        >
          {network}
        </span>
      </div>

      <div className="wallet-state" role="status">
        {connected ? (
          <>
            <p className="wallet-key">{publicKeyText}</p>
            <p className="wallet-meta">Connected public key</p>
          </>
        ) : (
          <>
            <p className="wallet-key">Not connected</p>
            <p className="wallet-meta">Connect a wallet to continue</p>
          </>
        )}
      </div>

      <div className="wallet-actions">
        {!connected ? (
          <button
            type="button"
            className="wallet-button primary"
            onClick={handleConnect}
            disabled={isConnecting}
          >
            {isConnecting ? 'Connecting...' : 'Connect'}
          </button>
        ) : (
          <>
            <button
              type="button"
              className="wallet-button secondary"
              onClick={handleSignature}
              disabled={isSigning}
            >
              {isSigning ? 'Waiting for signature...' : 'Sign Message'}
            </button>
            <button
              type="button"
              className="wallet-button danger"
              onClick={handleDisconnect}
              disabled={isDisconnecting}
            >
              {isDisconnecting ? 'Disconnecting...' : 'Disconnect'}
            </button>
          </>
        )}
      </div>

      {error && <p className="wallet-error">{error}</p>}
    </section>
  );
};
