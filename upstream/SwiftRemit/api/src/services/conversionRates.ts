/**
 * Currency Conversion Rates Service — Issue #1545
 *
 * Provides on-demand exchange rate lookups between any two currencies
 * supported by the SwiftRemit currency config.
 *
 * ## Rate source
 * Exchange rates are fetched from the configured external provider
 * (EXCHANGE_RATE_API_URL).  When no external provider is configured the
 * service falls back to a set of static seed rates so the endpoint remains
 * functional in development and test environments.
 *
 * ## Caching
 * Rates are cached in memory for CACHE_TTL_MS (default: 5 minutes) to
 * avoid hammering the upstream provider on every request.  The cache key
 * is the "from:to" pair, normalised to upper case.  Cross-rates not
 * directly available from the provider are computed via USD as a pivot
 * currency.
 *
 * ## Precision
 * Rates are stored and returned as JavaScript numbers.  Six decimal places
 * are used when serialising to JSON so the response is deterministic.
 */

import axios from 'axios';
import { ConversionRate } from '../types';

const CACHE_TTL_MS = 5 * 60 * 1000; // 5 minutes

interface CacheEntry {
  rate: ConversionRate;
  expiresAt: number;
}

/**
 * Fallback static rates relative to USD.
 * Used only when EXCHANGE_RATE_API_URL is not configured.
 */
const STATIC_RATES_VS_USD: Record<string, number> = {
  USD: 1,
  EUR: 0.92,
  GBP: 0.79,
  JPY: 149.5,
  NGN: 1600,
  KES: 129,
  GHS: 15.7,
  ZAR: 18.3,
  INR: 83.5,
  PHP: 56.1,
  USDC: 1.0,
};

export class ConversionRateService {
  private cache = new Map<string, CacheEntry>();

  /**
   * Fetch the exchange rate from `fromCode` to `toCode`.
   *
   * @throws {Error} when either currency code is not in the static table and
   *                 no external provider is configured.
   */
  async getRate(fromCode: string, toCode: string): Promise<ConversionRate> {
    const from = fromCode.toUpperCase();
    const to = toCode.toUpperCase();
    const cacheKey = `${from}:${to}`;

    const cached = this.cache.get(cacheKey);
    if (cached && Date.now() < cached.expiresAt) {
      return cached.rate;
    }

    const rate = await this.fetchRate(from, to);
    this.cache.set(cacheKey, { rate, expiresAt: Date.now() + CACHE_TTL_MS });
    return rate;
  }

  /**
   * Fetch a rate, trying the external provider first and falling back to the
   * static pivot table.
   */
  private async fetchRate(from: string, to: string): Promise<ConversionRate> {
    if (from === to) {
      return {
        from,
        to,
        rate: 1,
        fetched_at: new Date().toISOString(),
      };
    }

    const externalUrl = process.env.EXCHANGE_RATE_API_URL;
    if (externalUrl) {
      try {
        return await this.fetchFromProvider(externalUrl, from, to);
      } catch (err) {
        console.warn(
          `[conversion-rates] External provider failed (${from}→${to}), falling back to static rates:`,
          err instanceof Error ? err.message : err,
        );
      }
    }

    return this.computeFromStaticRates(from, to);
  }

  /**
   * Fetch a rate from the configured external provider.
   *
   * The provider must expose a JSON endpoint at:
   *   GET {EXCHANGE_RATE_API_URL}?from={FROM}&to={TO}
   *
   * Expected response shape:
   *   { "rate": number }   — or —  { "result": number }
   *
   * This is intentionally minimal so the service can front any simple
   * rate-API (e.g. exchangerate.host, openexchangerates, a self-hosted proxy)
   * with only an env-var change.
   */
  private async fetchFromProvider(
    baseUrl: string,
    from: string,
    to: string,
  ): Promise<ConversionRate> {
    const response = await axios.get<{ rate?: number; result?: number }>(baseUrl, {
      params: { from, to },
      timeout: 5000,
    });

    const rawRate = response.data?.rate ?? response.data?.result;
    if (typeof rawRate !== 'number' || !isFinite(rawRate) || rawRate <= 0) {
      throw new Error(`Invalid rate received from provider: ${JSON.stringify(response.data)}`);
    }

    return {
      from,
      to,
      rate: parseFloat(rawRate.toFixed(6)),
      fetched_at: new Date().toISOString(),
    };
  }

  /**
   * Compute a cross-rate using USD as a pivot currency.
   *
   * @throws {Error} when `from` or `to` is not in STATIC_RATES_VS_USD.
   */
  private computeFromStaticRates(from: string, to: string): ConversionRate {
    const fromRate = STATIC_RATES_VS_USD[from];
    const toRate = STATIC_RATES_VS_USD[to];

    if (fromRate === undefined) {
      throw new Error(
        `Unsupported currency for conversion: ${from}. ` +
        `Configure EXCHANGE_RATE_API_URL for dynamic rate lookups.`,
      );
    }
    if (toRate === undefined) {
      throw new Error(
        `Unsupported currency for conversion: ${to}. ` +
        `Configure EXCHANGE_RATE_API_URL for dynamic rate lookups.`,
      );
    }

    // Cross-rate: 1 FROM = (toRate / fromRate) TO
    const crossRate = toRate / fromRate;

    return {
      from,
      to,
      rate: parseFloat(crossRate.toFixed(6)),
      fetched_at: new Date().toISOString(),
    };
  }

  /** Clears the in-memory cache. Primarily for testing. */
  clearCache(): void {
    this.cache.clear();
  }
}

// Singleton instance shared across requests
let instance: ConversionRateService | null = null;

export function getConversionRateService(): ConversionRateService {
  if (!instance) {
    instance = new ConversionRateService();
  }
  return instance;
}
