/**
 * amount – strict parsing of user-entered Stellar amounts.
 *
 * `Number(amount)` accepts '1e3', ' 12 ', '0x10' and silently rounds
 * long decimals. These helpers accept only plain decimals with at most 7
 * fractional digits (Stellar's precision) and convert to integer stroops
 * with BigInt, so no floating-point rounding is involved.
 */

export const STROOPS_PER_UNIT = 10_000_000n;
export const STELLAR_DECIMALS = 7;
/** Largest amount Stellar can represent: int64 max in stroops. */
export const MAX_STROOPS = 9_223_372_036_854_775_807n;

/** Digits, optional single dot followed by 1-7 digits. No sign, exponent, spaces or leading zeros. */
const AMOUNT_RE = /^(0|[1-9]\d*)(\.\d{1,7})?$/;

export type AmountError = 'required' | 'format' | 'notPositive' | 'tooLarge';

export type AmountResult = { ok: true; stroops: bigint } | { ok: false; error: AmountError };

/** Parses `input` into stroops, or returns why it was rejected. */
export function parseAmount(input: string): AmountResult {
  if (input === '') return { ok: false, error: 'required' };
  // A minus sign on an otherwise valid number is a sign problem, not a format one.
  if (input.startsWith('-') && AMOUNT_RE.test(input.slice(1))) return { ok: false, error: 'notPositive' };
  if (!AMOUNT_RE.test(input)) return { ok: false, error: 'format' };

  const [whole, fraction = ''] = input.split('.');
  const stroops = BigInt(whole) * STROOPS_PER_UNIT + BigInt(fraction.padEnd(STELLAR_DECIMALS, '0'));

  if (stroops <= 0n) return { ok: false, error: 'notPositive' };
  if (stroops > MAX_STROOPS) return { ok: false, error: 'tooLarge' };
  return { ok: true, stroops };
}

/** Convenience wrapper: stroops or null. */
export function amountToStroops(input: string): bigint | null {
  const r = parseAmount(input);
  return r.ok ? r.stroops : null;
}

/**
 * Formats stroops as the decimal string Stellar operations expect,
 * e.g. 125000000n → "12.5". Never uses exponent notation.
 */
export function stroopsToAmount(stroops: bigint): string {
  const negative = stroops < 0n;
  const abs = negative ? -stroops : stroops;
  const whole = abs / STROOPS_PER_UNIT;
  const fraction = (abs % STROOPS_PER_UNIT).toString().padStart(STELLAR_DECIMALS, '0').replace(/0+$/, '');
  return `${negative ? '-' : ''}${whole}${fraction ? `.${fraction}` : ''}`;
}
