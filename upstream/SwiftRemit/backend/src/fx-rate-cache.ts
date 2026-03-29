import NodeCache from 'node-cache';
import axios from 'axios';

export interface FxRateResponse {
  from: string;
  to: string;
  rate: number;
  timestamp: Date;
  provider: string;
  cached: boolean;
}

export interface FxRateCacheOptions {
  ttlSeconds?: number;
  checkPeriodSeconds?: number;
  refreshBeforeExpirySeconds?: number;
  externalApiUrl?: string;
  externalApiKey?: string;
}

export class FxRateCache {
  private cache: NodeCache;
  private ttlSeconds: number;
  private refreshBeforeExpirySeconds: number;
  private externalApiUrl: string;
  private externalApiKey: string;
  private refreshTimers: Map<string, NodeJS.Timeout>;

  constructor(options: FxRateCacheOptions = {}) {
    this.ttlSeconds = options.ttlSeconds || 60;
    this.refreshBeforeExpirySeconds = options.refreshBeforeExpirySeconds || 10;
    this.externalApiUrl = options.externalApiUrl || process.env.FX_API_URL || 'https://api.exchangerate-api.com/v4/latest';
    this.externalApiKey = options.externalApiKey || process.env.FX_API_KEY || '';
    this.refreshTimers = new Map();

    this.cache = new NodeCache({
      stdTTL: this.ttlSeconds,
      checkperiod: options.checkPeriodSeconds || 120,
      useClones: false,
    });

    // Listen for cache expiry events
    this.cache.on('expired', (key: string) => {
      this.clearRefreshTimer(key);
    });
  }

  /**
   * Get current FX rate with caching
   */
  async getCurrentRate(from: string, to: string): Promise<FxRateResponse> {
    // Normalize to uppercase
    const fromUpper = from.toUpperCase();
    const toUpper = to.toUpperCase();
    
    const cacheKey = this.getCacheKey(fromUpper, toUpper);

    // Try to get from cache first
    const cached = this.cache.get<FxRateResponse>(cacheKey);
    if (cached) {
      return { ...cached, cached: true };
    }

    // Cache miss - fetch from external API
    const rate = await this.fetchFromExternalApi(fromUpper, toUpper);
    
    // Store in cache
    this.cache.set(cacheKey, rate);

    // Schedule background refresh
    this.scheduleBackgroundRefresh(cacheKey, fromUpper, toUpper);

    return { ...rate, cached: false };
  }

  /**
   * Fetch rate from external FX provider
   */
  private async fetchFromExternalApi(from: string, to: string): Promise<FxRateResponse> {
    try {
      // Mock implementation - replace with actual API call
      const url = `${this.externalApiUrl}/${from}`;
      const headers: Record<string, string> = {};
      
      if (this.externalApiKey) {
        headers['Authorization'] = `Bearer ${this.externalApiKey}`;
      }

      const response = await axios.get(url, { 
        headers,
        timeout: 5000,
      });

      const rates = response.data.rates || {};
      const rate = rates[to];

      if (!rate) {
        throw new Error(`Rate not found for ${from}/${to}`);
      }

      return {
        from,
        to,
        rate: parseFloat(rate),
        timestamp: new Date(),
        provider: 'ExchangeRateAPI',
        cached: false,
      };
    } catch (error) {
      console.error(`Failed to fetch FX rate for ${from}/${to}:`, error);
      throw new Error(`Failed to fetch FX rate: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Schedule background refresh before cache expires
   */
  private scheduleBackgroundRefresh(cacheKey: string, from: string, to: string): void {
    // Clear any existing timer
    this.clearRefreshTimer(cacheKey);

    // Calculate when to refresh (TTL - refresh buffer)
    const refreshInMs = (this.ttlSeconds - this.refreshBeforeExpirySeconds) * 1000;

    if (refreshInMs > 0) {
      const timer = setTimeout(async () => {
        try {
          const rate = await this.fetchFromExternalApi(from, to);
          this.cache.set(cacheKey, rate);
          
          // Schedule next refresh
          this.scheduleBackgroundRefresh(cacheKey, from, to);
        } catch (error) {
          console.error(`Background refresh failed for ${cacheKey}:`, error);
          // Don't reschedule on error - let it expire naturally
        }
      }, refreshInMs);

      this.refreshTimers.set(cacheKey, timer);
    }
  }

  /**
   * Clear refresh timer for a cache key
   */
  private clearRefreshTimer(cacheKey: string): void {
    const timer = this.refreshTimers.get(cacheKey);
    if (timer) {
      clearTimeout(timer);
      this.refreshTimers.delete(cacheKey);
    }
  }

  /**
   * Generate cache key from currency pair
   */
  private getCacheKey(from: string, to: string): string {
    return `fx:${from.toUpperCase()}:${to.toUpperCase()}`;
  }

  /**
   * Manually invalidate cache for a currency pair
   */
  invalidate(from: string, to: string): void {
    const cacheKey = this.getCacheKey(from, to);
    this.cache.del(cacheKey);
    this.clearRefreshTimer(cacheKey);
  }

  /**
   * Clear all cached rates
   */
  clearAll(): void {
    this.cache.flushAll();
    this.refreshTimers.forEach(timer => clearTimeout(timer));
    this.refreshTimers.clear();
  }

  /**
   * Get cache statistics
   */
  getStats() {
    return this.cache.getStats();
  }

  /**
   * Close the cache and cleanup
   */
  close(): void {
    this.clearAll();
    this.cache.close();
  }
}

// Singleton instance
let fxRateCacheInstance: FxRateCache | null = null;

export function getFxRateCache(options?: FxRateCacheOptions): FxRateCache {
  if (!fxRateCacheInstance) {
    fxRateCacheInstance = new FxRateCache(options);
  }
  return fxRateCacheInstance;
}

export function resetFxRateCache(): void {
  if (fxRateCacheInstance) {
    fxRateCacheInstance.close();
    fxRateCacheInstance = null;
  }
}
