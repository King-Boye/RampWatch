/**
 * recipientStorage – guarded, wallet-scoped persistence for saved recipients.
 *
 * - Every storage access is wrapped in try/catch: `localStorage` itself can
 *   throw (private mode, disabled cookies, quota exceeded).
 * - Recipients are stored per wallet (`swiftremit_recipients:<G…>`) so one
 *   user's contacts aren't shown to another wallet on a shared device.
 * - Data is cleared when the wallet disconnects.
 * - Stored data is validated; malformed entries are dropped.
 */

export interface SavedRecipient {
  id: number | string;
  name: string;
  address: string;
  country: string;
  memo?: string;
}

export const LEGACY_RECIPIENTS_KEY = 'swiftremit_recipients';
const KEY_PREFIX = `${LEGACY_RECIPIENTS_KEY}:`;

export function recipientsKey(walletAddress: string): string {
  return `${KEY_PREFIX}${walletAddress}`;
}

function getStorage(): Storage | null {
  try {
    if (typeof window === 'undefined') return null;
    return window.localStorage ?? null;
  } catch {
    return null;
  }
}

function isRecipient(value: unknown): value is SavedRecipient {
  if (!value || typeof value !== 'object') return false;
  const r = value as Record<string, unknown>;
  return (
    (typeof r.id === 'number' || typeof r.id === 'string') &&
    typeof r.name === 'string' &&
    typeof r.address === 'string' &&
    typeof r.country === 'string' &&
    (r.memo === undefined || typeof r.memo === 'string')
  );
}

function parse(raw: string | null): SavedRecipient[] | null {
  if (raw === null) return null;
  try {
    const data: unknown = JSON.parse(raw);
    return Array.isArray(data) ? data.filter(isRecipient) : [];
  } catch {
    return [];
  }
}

export interface LoadResult {
  recipients: SavedRecipient[];
  /** False when storage was unavailable or unreadable. */
  ok: boolean;
}

/**
 * Loads the recipients saved for `walletAddress`. Data saved by older
 * versions under the unscoped key is moved to this wallet and the unscoped
 * copy is deleted.
 */
export function loadRecipients(walletAddress: string | null | undefined): LoadResult {
  if (!walletAddress) return { recipients: [], ok: true };
  const storage = getStorage();
  if (!storage) return { recipients: [], ok: false };
  try {
    const scoped = parse(storage.getItem(recipientsKey(walletAddress)));
    if (scoped) return { recipients: scoped, ok: true };

    const legacy = parse(storage.getItem(LEGACY_RECIPIENTS_KEY));
    if (legacy) {
      storage.setItem(recipientsKey(walletAddress), JSON.stringify(legacy));
      storage.removeItem(LEGACY_RECIPIENTS_KEY);
      return { recipients: legacy, ok: true };
    }
    return { recipients: [], ok: true };
  } catch {
    return { recipients: [], ok: false };
  }
}

/** Returns false if the data could not be persisted. */
export function saveRecipients(walletAddress: string | null | undefined, recipients: SavedRecipient[]): boolean {
  if (!walletAddress) return false;
  const storage = getStorage();
  if (!storage) return false;
  try {
    storage.setItem(recipientsKey(walletAddress), JSON.stringify(recipients));
    return true;
  } catch {
    return false;
  }
}

/**
 * Removes saved recipients. With a wallet, only that wallet's entry is
 * removed; without one, every recipient entry (including the legacy key) is.
 */
export function clearRecipients(walletAddress?: string | null): void {
  const storage = getStorage();
  if (!storage) return;
  try {
    if (walletAddress) {
      storage.removeItem(recipientsKey(walletAddress));
      return;
    }
    const keys: string[] = [];
    for (let i = 0; i < storage.length; i++) {
      const key = storage.key(i);
      if (key && (key === LEGACY_RECIPIENTS_KEY || key.startsWith(KEY_PREFIX))) keys.push(key);
    }
    keys.forEach((k) => storage.removeItem(k));
  } catch {
    // storage unavailable — nothing to clear
  }
}
