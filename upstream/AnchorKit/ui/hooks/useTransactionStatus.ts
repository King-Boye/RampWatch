import { useState, useEffect, useRef } from 'react';
import { TxStatus } from '../components/TransactionTimeline';
import {
  TransactionStateTracker,
  TransactionSnapshot,
  TransactionTransition,
  FetchStatusFn,
} from './TransactionStateTracker';

export type { FetchStatusFn, TransactionTransition, TransactionSnapshot };

export interface UseTransactionStatusOptions {
  /** Function that fetches the current status for a given transaction ID. */
  fetchStatus: FetchStatusFn;
  /** Polling interval in milliseconds. Defaults to 5000. */
  interval?: number;
  /** Set to false to pause polling without unmounting. Defaults to true. */
  enabled?: boolean;
}

export interface UseTransactionStatusResult {
  /** Current transaction status, or null before the first poll completes. */
  status: TxStatus | null;
  /** Ordered list of recorded status transitions. */
  transitions: TransactionTransition[];
  /** True when status is "completed" or "failed" (polling has stopped). */
  isTerminal: boolean;
  /** Last fetch error, if any. Cleared on the next successful poll. */
  error: Error | null;
}

/**
 * Polls a TransactionStateTracker for the given txId and exposes a consistent
 * status interface. Polling starts automatically when txId is valid and
 * enabled is true, stops once a terminal state is reached, and cleans up
 * timers on unmount.
 *
 * @example
 * const { status, transitions, isTerminal } = useTransactionStatus(txId, {
 *   fetchStatus: (id) => api.getTransactionStatus(id),
 *   interval: 3000,
 * });
 */
export function useTransactionStatus(
  txId: string | null | undefined,
  options: UseTransactionStatusOptions
): UseTransactionStatusResult {
  const { interval = 5000, enabled = true } = options;

  // Ref so that an inline fetchStatus function doesn't restart the polling loop
  const fetchStatusRef = useRef<FetchStatusFn>(options.fetchStatus);
  fetchStatusRef.current = options.fetchStatus;

  const [snapshot, setSnapshot] = useState<TransactionSnapshot | null>(null);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!txId || !enabled) {
      setSnapshot(null);
      setError(null);
      return;
    }

    const tracker = new TransactionStateTracker((id) => fetchStatusRef.current(id));
    let cancelled = false;
    let timerId: ReturnType<typeof setTimeout> | null = null;

    const poll = async () => {
      try {
        const snap = await tracker.poll(txId);
        if (cancelled) return;
        setSnapshot(snap);
        setError(null);
        if (!snap.isTerminal) {
          timerId = setTimeout(poll, interval);
        }
      } catch (err) {
        if (cancelled) return;
        setError(err instanceof Error ? err : new Error(String(err)));
        timerId = setTimeout(poll, interval);
      }
    };

    poll();

    return () => {
      cancelled = true;
      if (timerId !== null) clearTimeout(timerId);
      tracker.reset();
    };
  }, [txId, enabled, interval]);

  return {
    status: snapshot?.status ?? null,
    transitions: snapshot?.transitions ?? [],
    isTerminal: snapshot?.isTerminal ?? false,
    error,
  };
}
