import { renderHook, act } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { nextDelay, usePolling } from '../usePolling';

function setVisibility(state: 'visible' | 'hidden'): void {
  Object.defineProperty(document, 'visibilityState', { configurable: true, get: () => state });
  document.dispatchEvent(new Event('visibilitychange'));
}

/** Advance fake timers and let awaited callbacks settle. */
async function advance(ms: number): Promise<void> {
  await act(async () => {
    await vi.advanceTimersByTimeAsync(ms);
  });
}

describe('nextDelay', () => {
  const opts = { intervalMs: 1000, backoffFactor: 2, maxIntervalMs: 5000 };

  it('resets to the base interval on success', () => {
    expect(nextDelay(4000, false, opts)).toBe(1000);
  });

  it('grows on failure up to the cap', () => {
    expect(nextDelay(1000, true, opts)).toBe(2000);
    expect(nextDelay(2000, true, opts)).toBe(4000);
    expect(nextDelay(4000, true, opts)).toBe(5000);
  });

  it('defaults the cap to 10x the interval', () => {
    expect(nextDelay(50_000, true, { intervalMs: 1000 })).toBe(10_000);
  });
});

describe('usePolling', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    setVisibility('visible');
  });

  afterEach(() => {
    vi.useRealTimers();
    setVisibility('visible');
  });

  it('polls on the interval while enabled', async () => {
    const cb = vi.fn();
    renderHook(() => usePolling(cb, { intervalMs: 1000 }));
    expect(cb).not.toHaveBeenCalled();
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(1);
    await advance(2000);
    expect(cb).toHaveBeenCalledTimes(3);
  });

  it('runs immediately when requested', async () => {
    const cb = vi.fn();
    renderHook(() => usePolling(cb, { intervalMs: 1000, immediate: true }));
    await advance(0);
    expect(cb).toHaveBeenCalledTimes(1);
  });

  it('does nothing when disabled', async () => {
    const cb = vi.fn();
    renderHook(() => usePolling(cb, { intervalMs: 1000, enabled: false }));
    await advance(5000);
    expect(cb).not.toHaveBeenCalled();
  });

  it('pauses while hidden and catches up when visible again', async () => {
    const cb = vi.fn();
    renderHook(() => usePolling(cb, { intervalMs: 1000 }));
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(1);

    setVisibility('hidden');
    await advance(10_000);
    expect(cb).toHaveBeenCalledTimes(1);

    setVisibility('visible');
    await advance(0);
    expect(cb).toHaveBeenCalledTimes(2);
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(3);
  });

  it('keeps polling while hidden when pauseWhenHidden is false', async () => {
    const cb = vi.fn();
    renderHook(() => usePolling(cb, { intervalMs: 1000, pauseWhenHidden: false }));
    setVisibility('hidden');
    await advance(3000);
    expect(cb).toHaveBeenCalledTimes(3);
  });

  it('backs off after failures and recovers after a success', async () => {
    let fail = true;
    const cb = vi.fn(() => (fail ? Promise.reject(new Error('boom')) : Promise.resolve()));
    renderHook(() => usePolling(cb, { intervalMs: 1000, backoffFactor: 2 }));

    await advance(1000); // fail → next in 2000
    expect(cb).toHaveBeenCalledTimes(1);
    await advance(1999);
    expect(cb).toHaveBeenCalledTimes(1);
    await advance(1); // fail → next in 4000
    expect(cb).toHaveBeenCalledTimes(2);

    fail = false;
    await advance(4000); // success → next in 1000
    expect(cb).toHaveBeenCalledTimes(3);
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(4);
  });

  it('treats a false return value as a failure', async () => {
    const cb = vi.fn(() => false);
    renderHook(() => usePolling(cb, { intervalMs: 1000 }));
    await advance(1000);
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(1);
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(2);
  });

  it('never overlaps a slow callback', async () => {
    let resolve: () => void = () => {};
    const cb = vi.fn(() => new Promise<void>((r) => (resolve = r)));
    renderHook(() => usePolling(cb, { intervalMs: 1000 }));
    await advance(1000);
    await advance(5000);
    expect(cb).toHaveBeenCalledTimes(1);
    await act(async () => resolve());
    await advance(1000);
    expect(cb).toHaveBeenCalledTimes(2);
  });

  it('stops on unmount', async () => {
    const cb = vi.fn();
    const { unmount } = renderHook(() => usePolling(cb, { intervalMs: 1000 }));
    unmount();
    await advance(5000);
    expect(cb).not.toHaveBeenCalled();
  });
});
