import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import {
  LEGACY_RECIPIENTS_KEY,
  clearRecipients,
  loadRecipients,
  recipientsKey,
  saveRecipients,
} from '../recipientStorage';

const A = 'G' + 'A'.repeat(55);
const B = 'G' + 'B'.repeat(55);
const r = (name: string) => ({ id: name, name, address: A, country: 'NG' });

describe('recipientStorage', () => {
  beforeEach(() => localStorage.clear());
  afterEach(() => vi.restoreAllMocks());

  it('returns nothing and persists nothing without a wallet', () => {
    expect(loadRecipients(null)).toEqual({ recipients: [], ok: true });
    expect(saveRecipients(null, [r('x')])).toBe(false);
    expect(localStorage.length).toBe(0);
  });

  it('round-trips per wallet', () => {
    expect(saveRecipients(A, [r('a')])).toBe(true);
    saveRecipients(B, [r('b')]);
    expect(loadRecipients(A).recipients.map((x) => x.name)).toEqual(['a']);
    expect(loadRecipients(B).recipients.map((x) => x.name)).toEqual(['b']);
  });

  it('returns ok:false instead of throwing when storage is blocked', () => {
    vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => {
      throw new Error('blocked');
    });
    expect(loadRecipients(A)).toEqual({ recipients: [], ok: false });
  });

  it('returns false when a write fails', () => {
    vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
      throw new Error('quota');
    });
    expect(saveRecipients(A, [r('a')])).toBe(false);
  });

  it('treats invalid JSON as empty', () => {
    localStorage.setItem(recipientsKey(A), '{nope');
    expect(loadRecipients(A)).toEqual({ recipients: [], ok: true });
  });

  it('clears one wallet or everything', () => {
    saveRecipients(A, [r('a')]);
    saveRecipients(B, [r('b')]);
    localStorage.setItem(LEGACY_RECIPIENTS_KEY, '[]');
    localStorage.setItem('unrelated', '1');

    clearRecipients(A);
    expect(localStorage.getItem(recipientsKey(A))).toBeNull();
    expect(localStorage.getItem(recipientsKey(B))).not.toBeNull();

    clearRecipients();
    expect(localStorage.getItem(recipientsKey(B))).toBeNull();
    expect(localStorage.getItem(LEGACY_RECIPIENTS_KEY)).toBeNull();
    expect(localStorage.getItem('unrelated')).toBe('1');
  });
});
