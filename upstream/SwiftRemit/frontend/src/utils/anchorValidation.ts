/**
 * anchorValidation – strict validation for the AnchorManagement form.
 *
 * Replaces `parseFloat(x) || 0`, which silently turned empty or invalid input
 * into 0 (e.g. max_amount = 0 or a 0% fee).
 */

export interface AnchorNumericInput {
  deposit_fee_percent: string;
  withdrawal_fee_percent: string;
  min_amount: string;
  max_amount: string;
}

export type AnchorNumericField = keyof AnchorNumericInput;

export interface AnchorNumericValues {
  deposit_fee_percent: number;
  withdrawal_fee_percent: number;
  min_amount: number;
  max_amount: number;
}

export type AnchorFieldErrors = Partial<Record<AnchorNumericField | 'supported_currencies', string>>;

/** Plain non-negative decimal: "12", "0.5", "12.50". No exponents, signs, hex or spaces. */
const DECIMAL_RE = /^\d+(\.\d+)?$/;

/** Parses a strict decimal string, returning null for anything else. */
export function parseStrictDecimal(raw: string): number | null {
  const value = raw.trim();
  if (!DECIMAL_RE.test(value)) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}

const LABELS: Record<AnchorNumericField, string> = {
  deposit_fee_percent: 'Deposit fee',
  withdrawal_fee_percent: 'Withdrawal fee',
  min_amount: 'Minimum amount',
  max_amount: 'Maximum amount',
};

export interface AnchorValidationResult {
  valid: boolean;
  errors: AnchorFieldErrors;
  values: AnchorNumericValues | null;
}

export function validateAnchorNumbers(input: AnchorNumericInput): AnchorValidationResult {
  const errors: AnchorFieldErrors = {};
  const parsed: Partial<AnchorNumericValues> = {};

  (Object.keys(LABELS) as AnchorNumericField[]).forEach((field) => {
    const raw = input[field] ?? '';
    if (!raw.trim()) {
      errors[field] = `${LABELS[field]} is required.`;
      return;
    }
    const n = parseStrictDecimal(raw);
    if (n === null) {
      errors[field] = `${LABELS[field]} must be a number like 10 or 2.5.`;
      return;
    }
    parsed[field] = n;
  });

  for (const field of ['deposit_fee_percent', 'withdrawal_fee_percent'] as const) {
    if (parsed[field] !== undefined && parsed[field]! > 100) {
      errors[field] = `${LABELS[field]} must be between 0 and 100%.`;
    }
  }

  if (parsed.max_amount !== undefined && parsed.max_amount <= 0) {
    errors.max_amount = 'Maximum amount must be greater than 0.';
  }

  if (
    parsed.min_amount !== undefined &&
    parsed.max_amount !== undefined &&
    !errors.max_amount &&
    parsed.min_amount > parsed.max_amount
  ) {
    errors.max_amount = 'Maximum amount must be greater than or equal to the minimum amount.';
  }

  const valid = Object.keys(errors).length === 0;
  return { valid, errors, values: valid ? (parsed as AnchorNumericValues) : null };
}

/** Splits a comma-separated list, trimming and dropping empty entries. */
export function parseList(raw: string): string[] {
  return raw
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean);
}
