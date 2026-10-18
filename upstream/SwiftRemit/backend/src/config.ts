import { z } from 'zod';

const nodeEnv = process.env.NODE_ENV ?? 'development';

const nonceStoreSchema = z.enum(['postgres', 'redis', 'memory']);

function resolveNonceStore(): 'postgres' | 'redis' | 'memory' {
  const raw = process.env.WEBHOOK_NONCE_STORE;
  if (raw !== undefined && raw !== '') {
    const parsed = nonceStoreSchema.safeParse(raw);
    if (!parsed.success) {
      throw new Error(
        `Invalid WEBHOOK_NONCE_STORE value "${raw}". Expected one of: postgres, redis, memory.`,
      );
    }
    return parsed.data;
  }
  if (nodeEnv === 'test') {
    return 'memory';
  }
  if (nodeEnv === 'production') {
    return 'postgres';
  }
  return 'memory';
}

function resolveNonceTtlSeconds(): number {
  const raw = process.env.WEBHOOK_NONCE_TTL_SECONDS;
  if (raw === undefined || raw === '') {
    return 86400;
  }
  const parsed = Number(raw);
  if (!Number.isFinite(parsed) || !Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(
      `Invalid WEBHOOK_NONCE_TTL_SECONDS value "${raw}". Expected a positive integer number of seconds.`,
    );
  }
  return parsed;
}

const webhookNonceStoreBackend = resolveNonceStore();
const webhookNonceTtlSeconds = resolveNonceTtlSeconds();
const redisUrl = process.env.REDIS_URL;

if (webhookNonceStoreBackend === 'redis' && (redisUrl === undefined || redisUrl === '')) {
  throw new Error(
    'WEBHOOK_NONCE_STORE is set to "redis" but REDIS_URL is not configured. ' +
      'Set REDIS_URL to a valid Redis connection string, or choose a different WEBHOOK_NONCE_STORE backend.',
  );
}

export interface WebhookNonceStoreConfig {
  backend: 'postgres' | 'redis' | 'memory';
  ttlSeconds: number;
  redisUrl?: string;
}

export const webhookNonceStore: WebhookNonceStoreConfig = {
  backend: webhookNonceStoreBackend,
  ttlSeconds: webhookNonceTtlSeconds,
  ...(webhookNonceStoreBackend === 'redis' ? { redisUrl } : {}),
};