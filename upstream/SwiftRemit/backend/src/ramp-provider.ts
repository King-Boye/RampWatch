/**
 * Fiat on/off ramp provider interface and adapters.
 *
 * Normalises Transak and MoonPay webhook payloads into a canonical
 * RampOrderEvent so the rest of the system stays provider-agnostic.
 */

import crypto from 'crypto';

// ── Canonical types ────────────────────────────────────────────────

export type RampOrderStatus =
  | 'pending'
  | 'processing'
  | 'completed'
  | 'failed'
  | 'refunded'
  | 'cancelled';

export type RampDirection = 'on_ramp' | 'off_ramp';

export interface RampOrderEvent {
  provider: string;
  orderId: string;
  direction: RampDirection;
  status: RampOrderStatus;
  cryptoAmount?: number;
  fiatAmount?: number;
  fiatCurrency?: string;
  cryptoCurrency?: string;
  walletAddress?: string;
  /** Populated when the order carries a SwiftRemit remittance reference. */
  remittanceId?: string;
  /** Provider's event timestamp. */
  timestamp?: Date;
  /** Original provider payload for audit. */
  raw: unknown;
}

// ── Provider interface ─────────────────────────────────────────────

export interface RampProvider {
  readonly name: string;
  /** Verify the webhook signature and return true if authentic. */
  verifyWebhook(payload: string, headers: Record<string, string>): boolean;
  /** Normalise the raw provider payload into a canonical RampOrderEvent. */
  parseEvent(payload: unknown): RampOrderEvent;
}

// ── Transak adapter ────────────────────────────────────────────────

export interface TransakProviderOptions {
  /** Max allowed age (seconds) of a webhook event before it is rejected as a replay. */
  webhookToleranceSeconds?: number;
  /** Store used to track processed event IDs for idempotency. Defaults to an in-memory TTL store. */
  eventStore?: WebhookEventStore;
}

/**
 * Minimal idempotency store contract. Implementations must return true only the
 * first time a given key is seen (within the TTL window).
 */
export interface WebhookEventStore {
  /** Atomically mark `key` as seen. Returns true if newly seen, false if duplicate. */
  markSeen(key: string, ttlSeconds: number): boolean;
}

/** In-memory TTL-backed event store. Suitable for single-instance deployments. */
export class InMemoryWebhookEventStore implements WebhookEventStore {
  private readonly seen = new Map<string, number>();

  markSeen(key: string, ttlSeconds: number): boolean {
    const now = Date.now();
    // Opportunistic cleanup of expired entries.
    for (const [k, expiresAt] of this.seen) {
      if (expiresAt <= now) this.seen.delete(k);
    }
    if (this.seen.has(key)) return false;
    this.seen.set(key, now + ttlSeconds * 1000);
    return true;
  }
}

const DEFAULT_WEBHOOK_TOLERANCE_SECONDS = 5 * 60;

export class TransakProvider implements RampProvider {
  readonly name = 'transak';
  private readonly toleranceSeconds: number;
  private readonly eventStore: WebhookEventStore;

  constructor(private readonly apiSecret: string, options: TransakProviderOptions = {}) {
    this.toleranceSeconds = options.webhookToleranceSeconds ?? DEFAULT_WEBHOOK_TOLERANCE_SECONDS;
    this.eventStore = options.eventStore ?? new InMemoryWebhookEventStore();
  }

  verifyWebhook(payload: string, headers: Record<string, string>): boolean {
    const sig = headers['x-transak-signature'] ?? headers['X-Transak-Signature'];
    if (!sig) return false;
    const expected = crypto.createHmac('sha256', this.apiSecret).update(payload).digest('hex');
    let authentic: boolean;
    try {
      authentic = crypto.timingSafeEqual(Buffer.from(sig), Buffer.from(expected));
    } catch {
      return false;
    }
    if (!authentic) return false;

    // ── Replay protection ──────────────────────────────────────────
    let parsed: any;
    try {
      parsed = JSON.parse(payload);
    } catch {
      return false;
    }

    const data = parsed?.data ?? parsed;
    const event = parsed?.event ?? {};

    // Enforce an event-time window.
    const rawTimestamp =
      data?.createdAt ??
      data?.updatedAt ??
      event?.createdAt ??
      event?.timestamp ??
      headers['x-transak-timestamp'] ??
      headers['X-Transak-Timestamp'];
    if (rawTimestamp === undefined || rawTimestamp === null || rawTimestamp === '') return false;

    const eventTime = this.parseTimestamp(rawTimestamp);
    if (eventTime === null) return false;

    const ageSeconds = Math.abs(Date.now() - eventTime) / 1000;
    if (ageSeconds > this.toleranceSeconds) return false;

    // Idempotency: reject events we have already processed.
    const eventId = this.extractEventId(parsed, data, event);
    if (!eventId) return false;
    if (!this.eventStore.markSeen(eventId, this.toleranceSeconds)) return false;

    return true;
  }

  private parseTimestamp(value: unknown): number | null {
    if (typeof value === 'number') {
      // Heuristic: treat small values as unix seconds, large as milliseconds.
      return value < 1e12 ? value * 1000 : value;
    }
    if (typeof value === 'string') {
      const asNumber = Number(value);
      if (!Number.isNaN(asNumber) && value.trim() !== '') {
        return asNumber < 1e12 ? asNumber * 1000 : asNumber;
      }
      const parsed = Date.parse(value);
      return Number.isNaN(parsed) ? null : parsed;
    }
    return null;
  }

  private extractEventId(parsed: any, data: any, event: any): string | null {
    const id =
      event?.id ??
      data?.eventId ??
      data?.webhookEventId ??
      (data?.id !== undefined && data?.status !== undefined
        ? `${data.id}:${typeof data.status === 'object' ? JSON.stringify(data.status) : data.status}`
        : undefined) ??
      data?.id;
    if (id === undefined || id === null || id === '') return null;
    return String(id);
  }

  parseEvent(payload: unknown): RampOrderEvent {
    const p = payload as any;
    let order = p?.data ?? p;
    if (typeof order.status === 'object' && order.status !== null) {
      order = order.status;
    }

    const statusMap: Record<string, RampOrderStatus> = {
      AWAITING_PAYMENT_FROM_USER: 'pending',
      PAYMENT_DONE_MARKED_BY_USER: 'processing',
      PROCESSING: 'processing',
      PENDING_DELIVERY_FROM_TRANSAK: 'processing',
      COMPLETED: 'completed',
      FAILED: 'failed',
      REFUNDED: 'refunded',
      CANCELLED: 'cancelled',
    };

    return {
      provider: this.name,
      orderId: String(order.id ?? order.orderId ?? ''),
      direction: order.isBuyOrSell === 'SELL' ? 'off_ramp' : 'on_ramp',
      status: statusMap[order.status] ?? 'pending',
      cryptoAmount: order.cryptoAmount,
      fiatAmount: order.fiatAmount,
      fiatCurrency: order.fiatCurrency,
      cryptoCurrency: order.cryptocurrency,
      walletAddress: order.walletAddress,
      remittanceId: order.partnerOrderId ?? order.externalId,
      timestamp: order.createdAt ? new Date(order.createdAt) : undefined,
      raw: payload,
    };
  }
}

// ── MoonPay adapter ────────────────────────────────────────────────

export class MoonPayProvider implements RampProvider {
  readonly name = 'moonpay';

  constructor(private readonly secretKey: string) {}

  verifyWebhook(payload: string, headers: Record<string, string>): boolean {
    const sig =
      headers['moonpay-signature-v2'] ??
      headers['MoonPay-Signature-V2'] ??
      headers['x-moonpay-signature'];
    if (!sig) return false;
    const expected = crypto.createHmac('sha256', this.secretKey).update(payload).digest('base64');
    try {
      return crypto.timingSafeEqual(Buffer.from(sig), Buffer.from(expected));
    } catch {
      return false;
    }
  }

  parseEvent(payload: unknown): RampOrderEvent {
    const p = payload as any;
    const tx = p?.data ?? p;

    const statusMap: Record<string, RampOrderStatus> = {
      waitingPayment: 'pending',
      pending: 'pending',
      waitingAuthorization: 'processing',
      processing: 'processing',
      completed: 'completed',
      failed: 'failed',
      refunded: 'refunded',
    };

    return {
      provider: this.name,
      orderId: String(tx.id ?? ''),
      direction: p.type === 'transaction_sell_updated' ? 'off_ramp' : 'on_ramp',
      status: statusMap[tx.status] ?? 'pending',
      cryptoAmount: tx.quoteCurrencyAmount,
      fiatAmount: tx.baseCurrencyAmount,
      fiatCurrency: tx.baseCurrencyCode,
      cryptoCurrency: tx.quoteCurrencyCode,
      walletAddress: tx.walletAddress,
      remittanceId: tx.externalTransactionId,
      timestamp: tx.updatedAt ? new Date(tx.updatedAt) : undefined,
      raw: payload,
    };
  }
}

// ── Provider registry ──────────────────────────────────────────────

const _providers = new Map<string, RampProvider>();

export function registerProvider(provider: RampProvider): void {
  _providers.set(provider.name, provider);
}

export function getProvider(name: string): RampProvider | undefined {
  return _providers.get(name);
}

export function listProviders(): string[] {
  return Array.from(_providers.keys());
}
