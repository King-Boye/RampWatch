export interface Currency {
  code: string;
  symbol: string;
  decimal_precision: number;
  name?: string;
  /**
   * #1547 — Currency aliases.
   * Alternative names or codes that resolve to this currency
   * (e.g. ["DOLLAR", "DOLLARS"] → "USD").  Aliases are matched
   * case-insensitively and must be unique across the config.
   */
  aliases?: string[];
}

export interface CurrencyConfig {
  currencies: Currency[];
}

// ─── #1545: Conversion rates ─────────────────────────────────────────────────

export interface ConversionRate {
  from: string;
  to: string;
  rate: number;
  /** ISO-8601 timestamp of when this rate was fetched / computed. */
  fetched_at: string;
}

export interface ConversionRateResponse {
  success: boolean;
  data: ConversionRate;
  timestamp: string;
}

export interface ConversionRateErrorResponse {
  success: false;
  error: { message: string; code: string };
  timestamp: string;
}

export interface CurrencyResponse {
  success: boolean;
  data: Currency[];
  count: number;
  total?: number;
  limit?: number;
  offset?: number;
  timestamp: string;
}

export interface ErrorResponse {
  success: false;
  error: {
    message: string;
    code?: string;
  };
  timestamp: string;
}

// Re-export from shared logger so all api service code imports from one place.
export { StructuredLogger, createLogger, redact } from '../../shared/src/logger';
