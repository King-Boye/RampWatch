import type { Pool } from 'pg';

/**
 * Shared, atomic, TTL-based nonce store used to prevent webhook replay attacks
 * across multiple replicas (issue #1703).
 *
 * Implementations MUST provide an atomic `consume` operation: concurrent calls
 * for the same nonce must result in exactly one `true` return.
 */
export interface NonceStore {
  /**
   * Atomically attempt to consume a nonce.
   *
   * @param nonce      The nonce to consume.
   * @param ttlSeconds How long the nonce should be remembered.
   * @returns `true` if the nonce was newly accepted, `false` if already seen.
   */
  consume(nonce: string, ttlSeconds: number): Promise<boolean>;
}

/**
 * Minimal Redis client surface required by {@link RedisNonceStore}.
 * Compatible with `ioredis` and `node-redis` v4 `set` signatures.
 */
export interface RedisLikeClient {
  set(
    key: string,
    value: string,
    mode: 'NX',
    expiryMode: 'EX',
    ttlSeconds: number,
  ): Promise<string | null>;
}

export interface NonceStoreConfig {
  /** Which backend to use. Defaults to `'memory'` when unset. */
  driver?: 'memory' | 'postgres' | 'redis';
  /** Redis client, required when `driver === 'redis'`. */
  redisClient?: RedisLikeClient;
  /** Key prefix for Redis keys. Defaults to `'webhook:nonce:'`. */
  redisKeyPrefix?: string;
  /** Anchor id used for Postgres rows. Defaults to `'default'`. */
  anchorId?: string;
}

const DEFAULT_REDIS_PREFIX = 'webhook:nonce:';
const DEFAULT_ANCHOR_ID = 'default';
const DEFAULT_CLEANUP_INTERVAL_MS = 60_000;

/**
 * In-memory nonce store.
 *
 * WARNING: This is a per-process store and is NOT shared across replicas.
 * It exists only as an explicit test/dev fallback. Do not use it in a
 * multi-instance production deployment.
 */
export class InMemoryNonceStore implements NonceStore {
  private readonly nonces = new Map<string, number>();
  private readonly cleanupTimer: ReturnType<typeof setInterval> | null;

  constructor(cleanupIntervalMs: number = DEFAULT_CLEANUP_INTERVAL_MS) {
    this.cleanupTimer = setInterval(() => this.cleanup(), cleanupIntervalMs);
    // Do not keep the process alive just for cleanup.
    if (typeof this.cleanupTimer.unref === 'function') {
      this.cleanupTimer.unref();
    }
  }

  async consume(nonce: string, ttlSeconds: number): Promise<boolean> {
    const now = Date.now();
    const existingExpiry = this.nonces.get(nonce);

    if (existingExpiry !== undefined && existingExpiry > now) {
      return false;
    }

    this.nonces.set(nonce, now + ttlSeconds * 1000);
    return true;
  }

  /** Remove all expired nonces. */
  cleanup(): void {
    const now = Date.now();
    for (const [nonce, expiry] of this.nonces) {
      if (expiry <= now) {
        this.nonces.delete(nonce);
      }
    }
  }

  /** Stop the periodic cleanup timer. */
  dispose(): void {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
    }
  }
}

/**
 * Postgres-backed nonce store. Uses `INSERT ... ON CONFLICT DO NOTHING` for
 * atomicity and a TTL column for expiry.
 *
 * Expects a table like:
 *   CREATE TABLE webhook_processed_nonces (
 *     nonce      TEXT NOT NULL,
 *     anchor_id  TEXT NOT NULL,
 *     expires_at TIMESTAMPTZ NOT NULL,
 *     PRIMARY KEY (nonce, anchor_id)
 *   );
 */
export class PostgresNonceStore implements NonceStore {
  constructor(
    private readonly pool: Pool,
    private readonly anchorId: string = DEFAULT_ANCHOR_ID,
  ) {}

  async consume(nonce: string, ttlSeconds: number): Promise<boolean> {
    const result = await this.pool.query(
      `INSERT INTO webhook_processed_nonces (nonce, anchor_id, expires_at)
       VALUES ($1, $2, NOW() + ($3 || ' seconds')::INTERVAL)
       ON CONFLICT (nonce, anchor_id) DO NOTHING
       RETURNING nonce`,
      [nonce, this.anchorId, String(ttlSeconds)],
    );
    return (result.rowCount ?? 0) > 0;
  }

  /** Delete expired nonce rows. Intended to be called periodically. */
  async purgeExpired(): Promise<void> {
    await this.pool.query(
      `DELETE FROM webhook_processed_nonces WHERE expires_at <= NOW()`,
    );
  }
}

/**
 * Redis-backed nonce store. Uses `SET key value NX EX ttl` for atomic,
 * TTL-based consumption.
 */
export class RedisNonceStore implements NonceStore {
  constructor(
    private readonly client: RedisLikeClient,
    private readonly keyPrefix: string = DEFAULT_REDIS_PREFIX,
  ) {}

  async consume(nonce: string, ttlSeconds: number): Promise<boolean> {
    const reply = await this.client.set(
      `${this.keyPrefix}${nonce}`,
      '1',
      'NX',
      'EX',
      ttlSeconds,
    );
    return reply === 'OK';
  }
}

/**
 * Create a {@link NonceStore} based on the supplied configuration.
 *
 * @param config Nonce store configuration.
 * @param pool   Postgres pool, required when `config.driver === 'postgres'`.
 */
export function createNonceStore(
  config: NonceStoreConfig,
  pool?: Pool,
): NonceStore {
  const driver = config.driver ?? 'memory';

  switch (driver) {
    case 'postgres': {
      if (!pool) {
        throw new Error(
          'createNonceStore: a Postgres Pool is required for the postgres driver',
        );
      }
      return new PostgresNonceStore(pool, config.anchorId ?? DEFAULT_ANCHOR_ID);
    }
    case 'redis': {
      if (!config.redisClient) {
        throw new Error(
          'createNonceStore: a redisClient is required for the redis driver',
        );
      }
      return new RedisNonceStore(
        config.redisClient,
        config.redisKeyPrefix ?? DEFAULT_REDIS_PREFIX,
      );
    }
    case 'memory':
      return new InMemoryNonceStore();
    default: {
      const exhaustive: never = driver;
      throw new Error(`createNonceStore: unknown driver "${exhaustive}"`);
    }
  }
}