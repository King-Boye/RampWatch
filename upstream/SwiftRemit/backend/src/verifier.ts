import axios, { AxiosInstance } from 'axios';
import * as toml from 'toml';
import { VerificationStatus, VerificationResult, VerificationSource } from './types';

const STELLAR_EXPERT_API = 'https://api.stellar.expert/explorer/testnet';
const REQUEST_TIMEOUT = 5000;
const MAX_RETRIES = 3;
const POSITIVE_CACHE_TTL_MS = 30 * 60 * 1000; // 30 minutes for verified assets
const NEGATIVE_CACHE_TTL_MS = 5 * 60 * 1000;  // 5 minutes for unverified/suspicious assets

// ─── #1543: Reputation decay ─────────────────────────────────────────────────
// After a result has been cached and served, its effective reputation score
// decays linearly over time.  A "fresh" result (age = 0) keeps its full score.
// After DECAY_HALF_LIFE_MS the score is halved; after 2× the half-life it
// reaches zero.  Decay is applied only when reading from cache — the stored
// score is never mutated so the original result is preserved for audit purposes.
const DECAY_HALF_LIFE_MS = 24 * 60 * 60 * 1000; // 24 hours

/**
 * Apply linear reputation decay to a cached score based on how long ago the
 * result was computed.
 *
 * @param score       The reputation score at the time of verification (0–100).
 * @param verifiedAt  ISO-8601 string recorded when the result was produced.
 * @returns           The decayed score clamped to [0, 100].
 */
function applyReputationDecay(score: number, verifiedAt: string): number {
  const ageMs = Date.now() - new Date(verifiedAt).getTime();
  if (ageMs <= 0) return score;

  // Linear decay: score × max(0, 1 − age / (2 × half_life))
  const decayFactor = Math.max(0, 1 - ageMs / (2 * DECAY_HALF_LIFE_MS));
  return Math.round(score * decayFactor);
}

// ─── #1544: Source reliability weights ──────────────────────────────────────
// Each verification source is assigned a reliability weight that reflects how
// trustworthy and stable that data source is.  Weights influence the final
// weighted-average reputation score so that a high-quality source (e.g.
// Stellar Expert) has more impact than a secondary signal (e.g. raw
// transaction count).
//
// Weights are normalised internally so they do not need to sum to 1.0 — only
// the relative magnitudes matter.
const SOURCE_RELIABILITY_WEIGHTS: Record<string, number> = {
  'Stellar Expert':      0.40, // curated, rated by an established community index
  'Stellar TOML':        0.30, // issuer self-attests; valuable but not third-party
  'Trustline Analysis':  0.20, // on-chain adoption signal; gameable at low cost
  'Transaction History': 0.10, // activity proxy; least discriminating signal
};

/**
 * Compute the weighted-average reputation score from all verification sources.
 *
 * Only sources that were marked `verified` contribute to the score.  If no
 * source was verified, the result is 0.
 *
 * @param sources  Array of VerificationSource objects (each may carry a
 *                 `reliability_weight` override; falls back to the static
 *                 SOURCE_RELIABILITY_WEIGHTS table, then to 1.0).
 */
function computeWeightedScore(sources: VerificationSource[]): number {
  let weightedSum = 0;
  let totalWeight = 0;

  for (const source of sources) {
    if (!source.verified) continue;

    const weight =
      source.reliability_weight ??
      SOURCE_RELIABILITY_WEIGHTS[source.name] ??
      1.0;

    weightedSum += source.score * weight;
    totalWeight += weight;
  }

  return totalWeight > 0 ? Math.round(weightedSum / totalWeight) : 0;
}

// ─────────────────────────────────────────────────────────────────────────────

interface CacheEntry {
  result: VerificationResult;
  expiresAt: number;
}

export class AssetVerifier {
  private httpClient: AxiosInstance;
  private cache = new Map<string, CacheEntry>();

  constructor() {
    this.httpClient = axios.create({
      timeout: REQUEST_TIMEOUT,
      headers: {
        'User-Agent': 'SwiftRemit-Verifier/1.0',
      },
    });
  }

  async verifyAsset(assetCode: string, issuer: string): Promise<VerificationResult> {
    const cacheKey = `${assetCode}:${issuer}`;
    const cached = this.cache.get(cacheKey);

    if (cached && Date.now() < cached.expiresAt) {
      // #1543: apply decay to the cached score before returning
      const decayedScore = applyReputationDecay(
        cached.result.reputation_score,
        cached.result.verified_at,
      );

      // Re-derive status from the decayed score so consumers always see a
      // consistent (score, status) pair even for cached results.
      const decayedStatus = this.deriveStatus(decayedScore, cached.result.sources);

      return {
        ...cached.result,
        reputation_score: decayedScore,
        status: decayedStatus,
      };
    }

    const sources: VerificationSource[] = [];

    // Check Stellar Expert
    const expertResult = await this.checkStellarExpert(assetCode, issuer);
    sources.push(expertResult);

    // Check stellar.toml
    const tomlResult = await this.checkStellarToml(issuer);
    sources.push(tomlResult);

    // Check trustline count
    const trustlineResult = await this.checkTrustlines(assetCode, issuer);
    sources.push(trustlineResult);

    // Check transaction history
    const txHistoryResult = await this.checkTransactionHistory(assetCode, issuer);
    sources.push(txHistoryResult);

    // #1544: compute weighted reputation score
    const reputationScore = computeWeightedScore(sources);

    // Determine status
    const verifiedSourceCount = sources.filter(s => s.verified).length;
    const status = this.deriveStatus(reputationScore, sources, verifiedSourceCount);

    const verifiedAt = new Date().toISOString();

    const result: VerificationResult = {
      asset_code: assetCode,
      issuer,
      status,
      reputation_score: reputationScore,
      verified_at: verifiedAt,
      sources,
      trustline_count: trustlineResult.details?.count || 0,
      has_toml: tomlResult.verified,
    };

    const ttl = status === VerificationStatus.Verified ? POSITIVE_CACHE_TTL_MS : NEGATIVE_CACHE_TTL_MS;
    this.cache.set(cacheKey, { result, expiresAt: Date.now() + ttl });

    return result;
  }

  /**
   * Derive a VerificationStatus from a (potentially decayed) score and sources.
   * Extracted so the same logic applies to both fresh and cached results.
   */
  private deriveStatus(
    score: number,
    sources: VerificationSource[],
    verifiedSourceCount?: number,
  ): VerificationStatus {
    const count =
      verifiedSourceCount ?? sources.filter(s => s.verified).length;

    if (score >= 70 && count >= 3) {
      return VerificationStatus.Verified;
    } else if (score < 30 || this.hasSuspiciousIndicators(sources)) {
      return VerificationStatus.Suspicious;
    }
    return VerificationStatus.Unverified;
  }

  private async checkStellarExpert(
    assetCode: string,
    issuer: string
  ): Promise<VerificationSource> {
    try {
      const response = await this.retryRequest(async () => {
        return await this.httpClient.get(
          `${STELLAR_EXPERT_API}/asset/${assetCode}-${issuer}`
        );
      });

      if (response.data && response.data.rating) {
        const rating = response.data.rating;
        return {
          name: 'Stellar Expert',
          verified: rating >= 3,
          score: Math.min(rating * 20, 100),
          reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar Expert'],
          details: { rating, age: response.data.age },
        };
      }

      return {
        name: 'Stellar Expert',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar Expert'],
      };
    } catch (error) {
      console.error('Stellar Expert check failed:', error);
      return {
        name: 'Stellar Expert',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar Expert'],
      };
    }
  }

  private async checkStellarToml(issuer: string): Promise<VerificationSource> {
    try {
      // Get issuer's home domain
      const accountResponse = await this.retryRequest(async () => {
        return await this.httpClient.get(
          `${process.env.HORIZON_URL}/accounts/${issuer}`
        );
      });

      const homeDomain = accountResponse.data.home_domain;
      if (!homeDomain) {
        return {
          name: 'Stellar TOML',
          verified: false,
          score: 0,
          reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar TOML'],
        };
      }

      // Fetch stellar.toml
      const tomlUrl = `https://${homeDomain}/.well-known/stellar.toml`;
      const tomlResponse = await this.retryRequest(async () => {
        return await this.httpClient.get(tomlUrl);
      });

      const tomlData = toml.parse(tomlResponse.data);

      // Validate TOML structure
      const hasValidStructure =
        tomlData.DOCUMENTATION ||
        tomlData.CURRENCIES ||
        tomlData.PRINCIPALS;

      if (hasValidStructure) {
        return {
          name: 'Stellar TOML',
          verified: true,
          score: 80,
          reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar TOML'],
          details: {
            domain: homeDomain,
            has_documentation: !!tomlData.DOCUMENTATION,
            has_currencies: !!tomlData.CURRENCIES,
          },
        };
      }

      return {
        name: 'Stellar TOML',
        verified: false,
        score: 30,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar TOML'],
      };
    } catch (error) {
      console.error('Stellar TOML check failed:', error);
      return {
        name: 'Stellar TOML',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Stellar TOML'],
      };
    }
  }

  private async checkTrustlines(
    assetCode: string,
    issuer: string
  ): Promise<VerificationSource> {
    try {
      const response = await this.retryRequest(async () => {
        return await this.httpClient.get(
          `${process.env.HORIZON_URL}/assets`,
          {
            params: {
              asset_code: assetCode,
              asset_issuer: issuer,
            },
          }
        );
      });

      if (response.data._embedded?.records?.length > 0) {
        const asset = response.data._embedded.records[0];
        const trustlineCount = parseInt(asset.num_accounts || '0');

        let score = 0;
        if (trustlineCount >= 10000) score = 100;
        else if (trustlineCount >= 1000) score = 80;
        else if (trustlineCount >= 100) score = 60;
        else if (trustlineCount >= 10) score = 40;
        else score = 20;

        return {
          name: 'Trustline Analysis',
          verified: trustlineCount >= parseInt(process.env.MIN_TRUSTLINE_COUNT || '10'),
          score,
          reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Trustline Analysis'],
          details: { count: trustlineCount },
        };
      }

      return {
        name: 'Trustline Analysis',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Trustline Analysis'],
        details: { count: 0 },
      };
    } catch (error) {
      console.error('Trustline check failed:', error);
      return {
        name: 'Trustline Analysis',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Trustline Analysis'],
        details: { count: 0 },
      };
    }
  }

  private async checkTransactionHistory(
    assetCode: string,
    issuer: string
  ): Promise<VerificationSource> {
    try {
      const response = await this.retryRequest(async () => {
        return await this.httpClient.get(
          `${process.env.HORIZON_URL}/accounts/${issuer}/transactions`,
          {
            params: { limit: 200 },
          }
        );
      });

      const transactions = response.data._embedded?.records || [];
      const txCount = transactions.length;

      // Check for suspicious patterns
      const recentTxs = transactions.filter((tx: any) => {
        const txDate = new Date(tx.created_at);
        const daysSince = (Date.now() - txDate.getTime()) / (1000 * 60 * 60 * 24);
        return daysSince <= 30;
      });

      const hasRecentActivity = recentTxs.length > 0;
      const hasHistoricalActivity = txCount > 10;

      let score = 0;
      if (hasRecentActivity && hasHistoricalActivity) score = 70;
      else if (hasHistoricalActivity) score = 50;
      else if (hasRecentActivity) score = 30;

      return {
        name: 'Transaction History',
        verified: hasRecentActivity && hasHistoricalActivity,
        score,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Transaction History'],
        details: {
          total_transactions: txCount,
          recent_transactions: recentTxs.length,
        },
      };
    } catch (error) {
      console.error('Transaction history check failed:', error);
      return {
        name: 'Transaction History',
        verified: false,
        score: 0,
        reliability_weight: SOURCE_RELIABILITY_WEIGHTS['Transaction History'],
      };
    }
  }

  private hasSuspiciousIndicators(sources: VerificationSource[]): boolean {
    // Check for red flags
    const hasNoToml = !sources.find(s => s.name === 'Stellar TOML')?.verified;
    const hasLowTrustlines = (sources.find(s => s.name === 'Trustline Analysis')?.details?.count || 0) < 5;
    const hasNoHistory = !sources.find(s => s.name === 'Transaction History')?.verified;

    return hasNoToml && hasLowTrustlines && hasNoHistory;
  }

  private async retryRequest<T>(
    requestFn: () => Promise<T>,
    retries: number = MAX_RETRIES
  ): Promise<T> {
    for (let i = 0; i < retries; i++) {
      try {
        return await requestFn();
      } catch (error) {
        if (i === retries - 1) throw error;
        await this.delay(1000 * (i + 1)); // Exponential backoff
      }
    }
    throw new Error('Max retries exceeded');
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}
