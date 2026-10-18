/**
 * usePolling – shared, visibility-aware polling with error back-off.
 *
 * - Pauses while the document is hidden and polls once immediately when the
 *   tab becomes visible again.
 * - Uses a self-scheduling setTimeout chain, so a slow request never overlaps
 *   the next tick.
 * - When the callback throws or returns `false`, the delay grows by
 *   `backoffFactor` up to `maxIntervalMs`; a successful tick resets it.
 */
import { useEffect, useRef } from 'react';

export interface UsePollingOptions {
  /** Base delay between polls in ms. */
  intervalMs: number;
  /** Polling only runs while true (default true). */
  enabled?: boolean;
  /** Run the callback once as soon as polling starts (default false). */
  immediate?: boolean;
  /** Pause while `document.hidden` is true (default true). */
  pauseWhenHidden?: boolean;
  /** Delay multiplier after a failed tick (default 2). Use 1 to disable. */
  backoffFactor?: number;
  /** Upper bound for the backed-off delay (default 10 × intervalMs). */
  maxIntervalMs?: number;
}

/** A tick fails if the callback throws or resolves to `false`. */
export type PollCallback = () => unknown | Promise<unknown>;

export function nextDelay(
  current: number,
  failed: boolean,
  { intervalMs, backoffFactor = 2, maxIntervalMs = intervalMs * 10 }: UsePollingOptions,
): number {
  if (!failed) return intervalMs;
  return Math.min(Math.max(current, intervalMs) * backoffFactor, maxIntervalMs);
}

function isHidden(): boolean {
  return typeof document !== 'undefined' && document.visibilityState === 'hidden';
}

export function usePolling(callback: PollCallback, options: UsePollingOptions): void {
  const { intervalMs, enabled = true, immediate = false, pauseWhenHidden = true, backoffFactor, maxIntervalMs } =
    options;

  const callbackRef = useRef(callback);
  callbackRef.current = callback;

  useEffect(() => {
    if (!enabled || intervalMs <= 0) return;

    const opts: UsePollingOptions = { intervalMs, backoffFactor, maxIntervalMs };
    let timer: ReturnType<typeof setTimeout> | null = null;
    let delay = intervalMs;
    let cancelled = false;
    let running = false;

    const clear = (): void => {
      if (timer !== null) {
        clearTimeout(timer);
        timer = null;
      }
    };

    const schedule = (ms: number): void => {
      clear();
      if (cancelled || (pauseWhenHidden && isHidden())) return;
      timer = setTimeout(tick, ms);
    };

    async function tick(): Promise<void> {
      timer = null;
      if (cancelled || running) return;
      running = true;
      let failed = false;
      try {
        failed = (await callbackRef.current()) === false;
      } catch {
        failed = true;
      } finally {
        running = false;
      }
      if (cancelled) return;
      delay = nextDelay(delay, failed, opts);
      schedule(delay);
    }

    const onVisibilityChange = (): void => {
      if (isHidden()) {
        clear();
      } else if (timer === null && !running) {
        // Catch up right away instead of waiting a full interval.
        void tick();
      }
    };

    if (immediate && !(pauseWhenHidden && isHidden())) void tick();
    else schedule(delay);

    if (pauseWhenHidden) document.addEventListener('visibilitychange', onVisibilityChange);

    return () => {
      cancelled = true;
      clear();
      if (pauseWhenHidden) document.removeEventListener('visibilitychange', onVisibilityChange);
    };
  }, [enabled, intervalMs, immediate, pauseWhenHidden, backoffFactor, maxIntervalMs]);
}
