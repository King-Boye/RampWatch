import React, { useEffect, useMemo, useState } from 'react';
import './TransactionStatusTracker.css';

export type TransactionProgressStatus =
  | 'initiated'
  | 'submitted'
  | 'processing'
  | 'completed'
  | 'failed';

interface TransactionStatusTrackerProps {
  currentStatus: TransactionProgressStatus;
  onRefresh?: () => Promise<void> | void;
  autoRefresh?: boolean;
  refreshIntervalMs?: number;
  title?: string;
}

const TRACKER_STEPS: Array<{ key: TransactionProgressStatus; label: string }> = [
  { key: 'initiated', label: 'Initiated' },
  { key: 'submitted', label: 'Submitted' },
  { key: 'processing', label: 'Processing' },
  { key: 'completed', label: 'Completed' },
  { key: 'failed', label: 'Failed' },
];

export const TransactionStatusTracker: React.FC<TransactionStatusTrackerProps> = ({
  currentStatus,
  onRefresh,
  autoRefresh = true,
  refreshIntervalMs = 10000,
  title = 'Transaction Status',
}) => {
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [lastRefreshedAt, setLastRefreshedAt] = useState<Date | null>(null);

  const activeIndex = useMemo(() => {
    return TRACKER_STEPS.findIndex((step) => step.key === currentStatus);
  }, [currentStatus]);

  const refresh = async () => {
    if (!onRefresh || isRefreshing) return;
    setIsRefreshing(true);
    try {
      await onRefresh();
      setLastRefreshedAt(new Date());
    } finally {
      setIsRefreshing(false);
    }
  };

  useEffect(() => {
    if (!autoRefresh || !onRefresh) return;

    const timer = window.setInterval(() => {
      refresh();
    }, refreshIntervalMs);

    return () => window.clearInterval(timer);
  }, [autoRefresh, onRefresh, refreshIntervalMs]);

  return (
    <section className="transaction-tracker" aria-label="Transaction status tracker">
      <header className="transaction-tracker-header">
        <h2>{title}</h2>
        <div className="transaction-tracker-refresh">
          {lastRefreshedAt && (
            <span className="tracker-refresh-meta">
              Last refresh: {lastRefreshedAt.toLocaleTimeString()}
            </span>
          )}
          <button
            type="button"
            onClick={refresh}
            className="tracker-refresh-button"
            disabled={!onRefresh || isRefreshing}
          >
            {isRefreshing ? 'Refreshing...' : 'Refresh'}
          </button>
        </div>
      </header>

      <ol className="transaction-tracker-steps">
        {TRACKER_STEPS.map((step, index) => {
          const isActive = index === activeIndex;
          const isDone = index < activeIndex;
          const isFuture = index > activeIndex;
          const stepClass = isActive ? 'active' : isDone ? 'done' : isFuture ? 'future' : '';

          return (
            <li className={`transaction-tracker-step ${stepClass}`} key={step.key}>
              <span className="step-marker" aria-hidden="true" />
              <span className="step-label">{step.label}</span>
            </li>
          );
        })}
      </ol>
    </section>
  );
};
