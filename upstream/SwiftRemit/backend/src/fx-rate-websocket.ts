import { Server as HttpServer } from 'http';
import { Server as SocketIOServer, Socket, Namespace } from 'socket.io';
import { FxRateCache, FxRateResponse, getFxRateCache, setFxRateEventBus } from './fx-rate-cache';
import { EventEmitter } from 'events';
import { corsOptionsFromEnv } from '../../shared/src/cors-options';

export const fxRateEvents = new EventEmitter();
fxRateEvents.setMaxListeners(50);

export interface FxRateUpdate {
  pair: string;
  from: string;
  to: string;
  rate: number;
  timestamp: string;
  provider: string;
}

export interface SubscribeMessage {
  pairs: string[];
}

export interface UnsubscribeMessage {
  pairs: string[];
}

export const MAX_PAIRS_PER_MESSAGE = 50;
export const MAX_SUBSCRIPTIONS_PER_SOCKET = 100;

const ISO_4217_CODES = new Set<string>([
  'AED', 'AFN', 'ALL', 'AMD', 'ANG', 'AOA', 'ARS', 'AUD', 'AWG', 'AZN',
  'BAM', 'BBD', 'BDT', 'BGN', 'BHD', 'BIF', 'BMD', 'BND', 'BOB', 'BRL',
  'BSD', 'BTN', 'BWP', 'BYN', 'BZD', 'CAD', 'CDF', 'CHF', 'CLP', 'CNY',
  'COP', 'CRC', 'CUP', 'CVE', 'CZK', 'DJF', 'DKK', 'DOP', 'DZD', 'EGP',
  'ERN', 'ETB', 'EUR', 'FJD', 'FKP', 'GBP', 'GEL', 'GHS', 'GIP', 'GMD',
  'GNF', 'GTQ', 'GYD', 'HKD', 'HNL', 'HTG', 'HUF', 'IDR', 'ILS', 'INR',
  'IQD', 'IRR', 'ISK', 'JMD', 'JOD', 'JPY', 'KES', 'KGS', 'KHR', 'KMF',
  'KPW', 'KRW', 'KWD', 'KYD', 'KZT', 'LAK', 'LBP', 'LKR', 'LRD', 'LSL',
  'LYD', 'MAD', 'MDL', 'MGA', 'MKD', 'MMK', 'MNT', 'MOP', 'MRU', 'MUR',
  'MVR', 'MWK', 'MXN', 'MYR', 'MZN', 'NAD', 'NGN', 'NIO', 'NOK', 'NPR',
  'NZD', 'OMR', 'PAB', 'PEN', 'PGK', 'PHP', 'PKR', 'PLN', 'PYG', 'QAR',
  'RON', 'RSD', 'RUB', 'RWF', 'SAR', 'SBD', 'SCR', 'SDG', 'SEK', 'SGD',
  'SHP', 'SLE', 'SOS', 'SRD', 'SSP', 'STN', 'SVC', 'SYP', 'SZL', 'THB',
  'TJS', 'TMT', 'TND', 'TOP', 'TRY', 'TTD', 'TWD', 'TZS', 'UAH', 'UGX',
  'USD', 'UYU', 'UZS', 'VES', 'VND', 'VUV', 'WST', 'XAF', 'XCD', 'XOF',
  'XPF', 'YER', 'ZAR', 'ZMW', 'ZWL',
]);

export function isValidCurrencyCode(code: unknown): code is string {
  return typeof code === 'string' && /^[A-Z]{3}$/.test(code) && ISO_4217_CODES.has(code);
}

function pairKey(from: string, to: string): string {
  return `${from.toUpperCase()}/${to.toUpperCase()}`;
}

interface ParsedPair {
  from: string;
  to: string;
  key: string;
}

function parsePair(raw: unknown): ParsedPair | null {
  if (typeof raw !== 'string') return null;
  const parts = raw.toUpperCase().split('/');
  if (parts.length !== 2) return null;
  const [from, to] = parts;
  if (!isValidCurrencyCode(from) || !isValidCurrencyCode(to)) return null;
  return { from, to, key: pairKey(from, to) };
}
=======

export class FxRateWebSocketServer {
  private io: SocketIOServer;
  private ns: Namespace;
  private cache: FxRateCache;

  constructor(httpServer: HttpServer) {
    this.cache = getFxRateCache();
    setFxRateEventBus(fxRateEvents);

    const { origin } = corsOptionsFromEnv();

    this.io = new SocketIOServer(httpServer, {
      cors: { origin, methods: ['GET', 'POST'] },
      path: '/ws',
    });

    this.ns = this.io.of('/fx-rates');
    this.setupNamespace();
    this.listenForCacheRefreshes();
  }

  private setupNamespace(): void {
    this.ns.on('connection', (socket: Socket) => {
      const subscribedPairs = new Set<string>();

      socket.on('subscribe', async (msg: SubscribeMessage) => {
        if (!Array.isArray(msg?.pairs)) {
          socket.emit('subscribe_error', {
            code: 'INVALID_PAYLOAD',
            message: 'pairs must be an array',
          });
          return;
        }
        if (msg.pairs.length > MAX_PAIRS_PER_MESSAGE) {
          socket.emit('subscribe_error', {
            code: 'TOO_MANY_PAIRS',
            message: `at most ${MAX_PAIRS_PER_MESSAGE} pairs per message`,
          });
          return;
        }

        const invalid: string[] = [];
        for (const raw of msg.pairs) {
          const parsed = parsePair(raw);
          if (!parsed) {
            invalid.push(String(raw));
            continue;
          }
          const { from, to, key } = parsed;
          if (subscribedPairs.has(key)) continue;
          if (subscribedPairs.size >= MAX_SUBSCRIPTIONS_PER_SOCKET) {
            socket.emit('subscribe_error', {
              code: 'SUBSCRIPTION_LIMIT',
              message: `at most ${MAX_SUBSCRIPTIONS_PER_SOCKET} subscriptions per socket`,
            });
            break;
          }
          subscribedPairs.add(key);
          socket.join(`pair:${key}`);

          // Rate-replay: send last known rate immediately on subscribe
          try {
            const rate = await this.cache.getCurrentRate(from, to);
            const update = this.toUpdate(rate);
            socket.emit('fx_rate', update);
          } catch {
            // silently skip if rate unavailable
          }
        }

        if (invalid.length > 0) {
          socket.emit('subscribe_error', {
            code: 'INVALID_PAIR',
            message: 'invalid currency pair(s)',
            pairs: invalid,
          });
        }
      });

      socket.on('unsubscribe', (msg: UnsubscribeMessage) => {
        if (!Array.isArray(msg?.pairs)) {
          socket.emit('subscribe_error', {
            code: 'INVALID_PAYLOAD',
            message: 'pairs must be an array',
          });
          return;
        }
        if (msg.pairs.length > MAX_PAIRS_PER_MESSAGE) {
          socket.emit('subscribe_error', {
            code: 'TOO_MANY_PAIRS',
            message: `at most ${MAX_PAIRS_PER_MESSAGE} pairs per message`,
          });
          return;
        }

        const invalid: string[] = [];
        for (const raw of msg.pairs) {
          const parsed = parsePair(raw);
          if (!parsed) {
            invalid.push(String(raw));
            continue;
          }
          subscribedPairs.delete(parsed.key);
          socket.leave(`pair:${parsed.key}`);
        }

        if (invalid.length > 0) {
          socket.emit('subscribe_error', {
            code: 'INVALID_PAIR',
            message: 'invalid currency pair(s)',
            pairs: invalid,
          });
        }
      });

      socket.on('disconnect', () => {
        subscribedPairs.clear();
      });
    });
  }

  private listenForCacheRefreshes(): void {
    fxRateEvents.on('rate_updated', (rate: FxRateResponse) => {
      const key = pairKey(rate.from, rate.to);
      const update = this.toUpdate(rate);
      this.ns.to(`pair:${key}`).emit('fx_rate', update);
    });
  }

  private toUpdate(rate: FxRateResponse): FxRateUpdate {
    return {
      pair: pairKey(rate.from, rate.to),
      from: rate.from.toUpperCase(),
      to: rate.to.toUpperCase(),
      rate: rate.rate,
      timestamp: new Date(rate.timestamp).toISOString(),
      provider: rate.provider,
    };
  }

  close(): void {
    this.io.close();
  }
}
