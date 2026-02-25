import React, { useMemo, useState } from 'react';
import './SendMoneyFlow.css';

type FlowStep = 1 | 2 | 3 | 4 | 5;

interface ConfirmPayload {
  amount: number;
  asset: string;
  recipient: string;
}

interface SendMoneyFlowProps {
  assets?: string[];
  onConfirm?: (payload: ConfirmPayload) => Promise<void>;
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

function isValidRecipient(input: string): boolean {
  return /^G[A-Z2-7]{55}$/.test(input.trim());
}

export const SendMoneyFlow: React.FC<SendMoneyFlowProps> = ({
  assets = DEFAULT_ASSETS,
  onConfirm,
}) => {
  const [step, setStep] = useState<FlowStep>(1);
  const [amount, setAmount] = useState('');
  const [asset, setAsset] = useState('');
  const [recipient, setRecipient] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isComplete, setIsComplete] = useState(false);

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
      const payload = { amount: parsedAmount, asset, recipient: recipient.trim() };

      if (onConfirm) {
        await onConfirm(payload);
      } else {
        await new Promise((resolve) => setTimeout(resolve, 700));
      }

      setIsComplete(true);
    } catch (confirmError) {
      setError('Transaction failed. Please try again.');
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
        </dl>
      );
    }

    return null;
  };

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
        <p className="flow-success" role="status">
          Transaction confirmed successfully.
        </p>
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
