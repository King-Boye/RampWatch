import { TxStatus } from '../components/TransactionTimeline';

export type FetchStatusFn = (txId: string) => Promise<TxStatus>;

export interface TransactionTransition {
  from: TxStatus | null;
  to: TxStatus;
  at: number;
}

export interface TransactionSnapshot {
  status: TxStatus;
  transitions: TransactionTransition[];
  isTerminal: boolean;
}

const TERMINAL_STATUSES = new Set<TxStatus>(['completed', 'failed']);

export function isTerminalStatus(status: TxStatus): boolean {
  return TERMINAL_STATUSES.has(status);
}

/**
 * Tracks a single transaction's status history by calling a fetch function
 * and recording each status transition. Designed to be used by useTransactionStatus.
 */
export class TransactionStateTracker {
  private _status: TxStatus | null = null;
  private _transitions: TransactionTransition[] = [];

  constructor(private readonly fetchStatus: FetchStatusFn) {}

  async poll(txId: string): Promise<TransactionSnapshot> {
    const newStatus = await this.fetchStatus(txId);
    if (newStatus !== this._status) {
      this._transitions = [
        ...this._transitions,
        { from: this._status, to: newStatus, at: Date.now() },
      ];
      this._status = newStatus;
    }
    return this.getSnapshot();
  }

  getSnapshot(): TransactionSnapshot {
    if (this._status === null) {
      throw new Error('No status polled yet');
    }
    return {
      status: this._status,
      transitions: [...this._transitions],
      isTerminal: isTerminalStatus(this._status),
    };
  }

  reset(): void {
    this._status = null;
    this._transitions = [];
  }
}
