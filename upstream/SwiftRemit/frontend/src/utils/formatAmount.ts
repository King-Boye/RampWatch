export const STROOPS_PER_USDC = 10_000_000n;

/**
 * Formats a BigInt stroop amount into a human-readable USDC display string.
 *
 * Uses BigInt division and remainder to avoid precision loss for amounts
 * above 2^53 stroops. The fractional part is zero-padded to 7 digits and
 * trailing zeros are trimmed.
 *
 * @param stroops - The amount in stroops as a BigInt.
 * @returns A display string such as "12.5" or "0.0000001".
 */
export function formatAmount(stroops: bigint): string {
  const negative = stroops < 0n;
  const abs = negative ? -stroops : stroops;

  const whole = abs / STROOPS_PER_USDC;
  const remainder = abs % STROOPS_PER_USDC;

  let result: string;
  if (remainder === 0n) {
    result = whole.toString();
  } else {
    const fraction = remainder.toString().padStart(7, "0").replace(/0+$/, "");
    result = `${whole.toString()}.${fraction}`;
  }

  return negative ? `-${result}` : result;
}

export default formatAmount;