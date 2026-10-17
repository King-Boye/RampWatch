import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import {
  InMemoryNonceStore,
  PostgresNonceStore,
  NonceStore,
} from './nonce-store';
import { WebhookVerifier } from '../webhook-verifier';

const DATABASE_URL = process.env.DATABASE_URL;

describe('InMemoryNonceStore', () => {
  let store: InMemoryNonceStore;

  beforeEach(() => {
    store = new InMemoryNonceStore();
  });

  afterEach(() => {
    store.clear();
  });

  it('accepts the first consume of a nonce', async () => {
    const accepted = await store.consume('nonce-1', 60_000);
    expect(accepted).toBe(true);
  });

  it('rejects a second consume of the same nonce', async () => {
    const first = await store.consume('nonce-1', 60_000);
    const second = await store.consume('nonce-1', 60_000);
    expect(first).toBe(true);
    expect(second).toBe(false);
  });

  it('accepts the nonce again after TTL expiry', async () => {
    const first = await store.consume('nonce-1', 10);
    expect(first).toBe(true);

    // Wait for the TTL to elapse.
    await new Promise((resolve) => setTimeout(resolve, 30));

    const afterExpiry = await store.consume('nonce-1', 10);
    expect(afterExpiry).toBe(true);
  });

  it('treats distinct nonces independently', async () => {
    expect(await store.consume('nonce-a', 60_000)).toBe(true);
    expect(await store.consume('nonce-b', 60_000)).toBe(true);
    expect(await store.consume('nonce-a', 60_000)).toBe(false);
    expect(await store.consume('nonce-b', 60_000)).toBe(false);
  });
});

describe.skipIf(!DATABASE_URL)('PostgresNonceStore (integration)', () => {
  let store: PostgresNonceStore;

  beforeEach(async () => {
    store = new PostgresNonceStore({ connectionString: DATABASE_URL });
    await store.init();
    await store.clear();
  });

  afterEach(async () => {
    await store.clear();
    await store.close();
  });

  it('rejects a replay of the same nonce', async () => {
    const first = await store.consume('pg-nonce-1', 60_000);
    const second = await store.consume('pg-nonce-1', 60_000);
    expect(first).toBe(true);
    expect(second).toBe(false);
  });

  it('accepts the nonce again after expiry', async () => {
    const first = await store.consume('pg-nonce-2', 10);
    expect(first).toBe(true);

    await new Promise((resolve) => setTimeout(resolve, 50));

    const afterExpiry = await store.consume('pg-nonce-2', 10);
    expect(afterExpiry).toBe(true);
  });
});

describe('WebhookVerifier cross-replica nonce protection', () => {
  let store: NonceStore;

  beforeEach(() => {
    store = new InMemoryNonceStore();
  });

  afterEach(() => {
    (store as InMemoryNonceStore).clear();
  });

  it('rejects a replay sent to a second verifier sharing the same store', async () => {
    const verifierA = new WebhookVerifier({ nonceStore: store });
    const verifierB = new WebhookVerifier({ nonceStore: store });

    const nonce = 'shared-nonce-1';

    const first = await verifierA.consumeNonce(nonce, 60_000);
    expect(first).toBe(true);

    // Replay to a different replica (verifier instance) sharing the store.
    const replay = await verifierB.consumeNonce(nonce, 60_000);
    expect(replay).toBe(false);
  });

  it('rejects a replay after a simulated restart against the same store', async () => {
    const nonce = 'restart-nonce-1';

    const verifierBeforeRestart = new WebhookVerifier({ nonceStore: store });
    const first = await verifierBeforeRestart.consumeNonce(nonce, 60_000);
    expect(first).toBe(true);

    // Simulate a process restart: construct a fresh verifier against the
    // same persistent store instance.
    const verifierAfterRestart = new WebhookVerifier({ nonceStore: store });
    const replay = await verifierAfterRestart.consumeNonce(nonce, 60_000);
    expect(replay).toBe(false);
  });
});