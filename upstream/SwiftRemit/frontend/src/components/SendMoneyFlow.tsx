import React, { useMemo, useState } from 'react';
import './SendMoneyFlow.css';
import { signTransaction } from '@stellar/freighter-api';
import * as StellarSdk from '@stellar/stellar-sdk';

type FlowStep = 1 | 2 | 3 | 4 | 5;

interface ConfirmPayload {
  amount: number;
  asset: string;
  recipient: string;
  memo?: string;
}

interface SendMoneyFlowProps {
  assets?: string[];
  onConfirm?: (payload: ConfirmPayload) => Promise<void>;
  senderPublicKey?: string;
  network?: 'TESTNET' | 'PUBLIC';
}

const STEPS: Record<FlowStep, string> = {
  1: 'Enter amount',
  2: 'Select asset',
  3: 'Enter recipient',
  4: 'Review summary',
  5: 'Confirm transaction',
};
const STEP_SEQUENCE: FlowStep[] = [1, 2, 3, 4, 5];

const DEFAULT_ASSETS = ['XLM', 'USDC', 'EURC'];

const HORIZON_URLS: Record<string, string> = {
  TESTNET: 'https://horizon-testnet.stellar.org',
  PUBLIC: 'https://horizon.stellar.org',
};

const STELLAR_EXPERT_BASE: Record<string, string> = {
  TESTNET: 'https://stellar.expert/explorer/testnet/tx',
  PUBLIC: 'https://stellar.expert/explorer/public/tx',
};

function isValidRecipient(input: string): boolean {
  return /^G[A-Z2-7]{55}$/.test(input.trim());
}

async function buildAndSubmitTransaction(
  payload: ConfirmPayload,
  senderPublicKey: string,
  network: 'TESTNET' | 'PUBLIC'
): Promise<string> {
  const horizonUrl = HORIZON_URLS[network];
  const server = new StellarSdk.Horizon.Server(horizonUrl);
  const networkPassphrase =
    network === 'PUBLIC'
      ? StellarSdk.Networks.PUBLIC
      : StellarSdk.Networks.TESTNET;

  const account = await server.loadAccount(senderPublicKey);

  let asset: StellarSdk.Asset;
  if (payload.asset === 'XLM') {
    asset = StellarSdk.Asset.native();
  } else {
    // For non-native assets, use a well-known issuer placeholder;
    // in production this would come from the asset registry.
    asset = new StellarSdk.Asset(payload.asset, senderPublicKey);
  }

  const txBuilder = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase,
  })
    .addOperation(
      StellarSdk.Operation.payment({
        destination: payload.recipient,
        asset,
        amount: String(payload.amount),
      })
    )
    .setTimeout(30);

  if (payload.memo) {
    txBuilder.addMemo(StellarSdk.Memo.text(payload.memo));
  }

  const tx = txBuilder.build();
  const xdr = tx.toXDR();

  const signResult = await signTransaction(xdr, { networkPassphrase });
  if ('error' in signResult && signResult.error) {
    throw new Error(signResult.error.message || 'User rejected the transaction');
  }

  const signedXdr = 'signedTxXdr' in signResult ? signResult.signedTxXdr : (signResult as any);
  const signedTx = StellarSdk.TransactionBuilder.fromXDR(signedXdr, networkPassphrase);
  const result = await server.submitTransaction(signedTx);
  return result.hash;
}

export const SendMoneyFlow: React.FC<SendMoneyFlowProps> = ({
  assets = DEFAULT_ASSETS,
  onConfirm,
  senderPublicKey = '',
  network = 'TESTNET',
}) => {
  const [step, setStep] = useState<FlowStep>(1);
  const [amount, setAmount] = useState('');
  const [asset, setAsset] = useState('');
  const [recipient, setRecipient] = useState('');
  const [memo, setMemo] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isComplete, setIsComplete] = useState(false);
  const [txHash, setTxHash] = useState<string | null>(null);

  const parsedAmount = useMemo(() => Number(amount), [amount]);

  const validateCurrentStep = (): string | null => {
    if (step === 1) {
      if (!amount) return 'Amount is required.';
      if (!Number.isFinite(parsedAmount) || parsedAmount <= 0) {
        return 'Amount must be greater than zero.';
      }
    }

    if (step === 2 && !asset) {
      return 'Please select an asset.';
    }

    if (step === 3 && !isValidRecipient(recipient)) {
      return 'Recipient must be a valid Stellar public key.';
    }

    return null;
  };

  const nextStep = () => {
    const validation = validateCurrentStep();
    if (validation) {
      setError(validation);
      return;
    }

    setError(null);
    setStep((previous) => Math.min(previous + 1, 5) as FlowStep);
  };

  const previousStep = () => {
    setError(null);
    setStep((previous) => Math.max(previous - 1, 1) as FlowStep);
  };

  const confirmTransfer = async () => {
    if (!amount || !asset || !recipient) {
      setError('Transaction details are incomplete.');
      return;
    }

    setError(null);
    setIsSubmitting(true);

    try {
      const payload: ConfirmPayload = {
        amount: parsedAmount,
        asset,
        recipient: recipient.trim(),
        ...(memo.trim() ? { memo: memo.trim() } : {}),
      };

      if (onConfirm) {
        await onConfirm(payload);
        setIsComplete(true);
      } else if (senderPublicKey) {
        // Freighter signing flow
        const hash = await buildAndSubmitTransaction(payload, senderPublicKey, network);
        setTxHash(hash);
        setIsComplete(true);
      } else {
        await new Promise((resolve) => setTimeout(resolve, 700));
        setIsComplete(true);
      }
    } catch (confirmError) {
      const msg = confirmError instanceof Error ? confirmError.message : '';
      if (
        msg.toLowerCase().includes('rejected') ||
        msg.toLowerCase().includes('denied') ||
        msg.toLowerCase().includes('user rejected')
      ) {
        setError('Transaction was rejected by the wallet.');
      } else if (msg.toLowerCase().includes('not installed')) {
        setError('Freighter wallet is not installed. Please install it to continue.');
      } else {
        setError('Transaction failed. Please try again.');
      }
      console.error(confirmError);
    } finally {
      setIsSubmitting(false);
    }
  };

  const renderStepContent = () => {
    if (step === 1) {
      return (
        <label className="flow-field" htmlFor="amount">
          <span>Amount</span>
          <input
            id="amount"
            type="number"
            min="0"
            step="0.000001"
            value={amount}
            onChange={(event) => setAmount(event.target.value)}
            placeholder="0.00"
          />
        </label>
      );
    }

    if (step === 2) {
      return (
        <label className="flow-field" htmlFor="asset">
          <span>Asset</span>
          <select
            id="asset"
            value={asset}
            onChange={(event) => setAsset(event.target.value)}
          >
            <option value="">Choose an asset</option>
            {assets.map((assetCode) => (
              <option key={assetCode} value={assetCode}>
                {assetCode}
              </option>
            ))}
          </select>
        </label>
      );
    }

    if (step === 3) {
      return (
        <>
          <label className="flow-field" htmlFor="recipient">
            <span>Recipient</span>
            <input
              id="recipient"
              type="text"
              value={recipient}
              onChange={(event) => setRecipient(event.target.value)}
              placeholder="G..."
            />
          </label>
          <label className="flow-field" htmlFor="memo">
            <span>Memo <span className="flow-field-optional">(optional)</span></span>
            <input
              id="memo"
              type="text"
              value={memo}
              onChange={(event) => setMemo(event.target.value.slice(0, 100))}
              placeholder="e.g. Invoice #1234"
              maxLength={100}
              aria-describedby="memo-count"
            />
            <span id="memo-count" className="flow-char-count" aria-live="polite">
              {memo.length}/100
            </span>
          </label>
        </>
      );
    }

    if (step === 4 || step === 5) {
      return (
        <dl className="flow-review">
          <div>
            <dt>Amount</dt>
            <dd>{amount || '-'}</dd>
          </div>
          <div>
            <dt>Asset</dt>
            <dd>{asset || '-'}</dd>
          </div>
          <div>
            <dt>Recipient</dt>
            <dd>{recipient || '-'}</dd>
          </div>
          {memo.trim() && (
            <div>
              <dt>Memo</dt>
              <dd>{memo.trim()}</dd>
            </div>
          )}
        </dl>
      );
    }

    return null;
  };

  const stellarExpertUrl = txHash
    ? `${STELLAR_EXPERT_BASE[network]}/${txHash}`
    : null;

  return (
    <section className="send-flow-card" aria-label="Send money flow">
      <div className="send-flow-header">
        <h2>Send Money</h2>
        <p>Step {step} of 5: {STEPS[step]}</p>
      </div>

      <ol className="send-step-indicator" aria-label="Progress">
        {STEP_SEQUENCE.map((stepKey) => (
          <li key={stepKey} className={stepKey <= step ? 'active' : ''}>
            {stepKey}
          </li>
        ))}
      </ol>

      {isComplete ? (
        <div className="flow-success" role="status">
          <p>Transaction confirmed successfully.</p>
          {txHash && (
            <>
              <p className="flow-tx-hash">
                Transaction hash: <code>{txHash}</code>
              </p>
              {stellarExpertUrl && (
                <a
                  href={stellarExpertUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flow-expert-link"
                >
                  View on Stellar Expert ↗
                </a>
              )}
            </>
          )}
        </div>
      ) : (
        <>
          <div className="send-flow-body">{renderStepContent()}</div>

          {error && <p className="flow-error">{error}</p>}

          <div className="send-flow-actions">
            <button
              type="button"
              className="flow-button muted"
              onClick={previousStep}
              disabled={step === 1 || isSubmitting}
            >
              Back
            </button>

            {step < 5 ? (
              <button
                type="button"
                className="flow-button primary"
                onClick={nextStep}
                disabled={isSubmitting}
              >
                Continue
              </button>
            ) : (
              <button
                type="button"
                className="flow-button primary"
                onClick={confirmTransfer}
                disabled={isSubmitting}
              >
                {isSubmitting ? 'Confirming...' : 'Confirm transaction'}
              </button>
            )}
          </div>
        </>
      )}
    </section>
  );
};
