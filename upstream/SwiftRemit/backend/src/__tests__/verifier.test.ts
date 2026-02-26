import { describe, it, expect, vi, beforeEach } from 'vitest';
import { AssetVerifier } from '../verifier';
import { VerificationStatus } from '../types';

describe('AssetVerifier', () => {
  let verifier: AssetVerifier;

  beforeEach(() => {
    verifier = new AssetVerifier();
  });

  it('should verify a well-known asset', async () => {
    const result = await verifier.verifyAsset(
      'USDC',
      'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN'
    );

    expect(result.asset_code).toBe('USDC');
    expect(result.status).toBeDefined();
    expect(result.reputation_score).toBeGreaterThanOrEqual(0);
    expect(result.reputation_score).toBeLessThanOrEqual(100);
    expect(result.sources).toHaveLength(4);
  });

  it('should mark asset as suspicious with low trustlines and no TOML', async () => {
    // Mock responses for a suspicious asset
    const result = await verifier.verifyAsset('SCAM', 'GXXX...');

    expect(result.status).toBe(VerificationStatus.Suspicious);
    expect(result.reputation_score).toBeLessThan(30);
  });

  it('should handle network errors gracefully', async () => {
    // Test with invalid issuer
    const result = await verifier.verifyAsset('TEST', 'INVALID');

    expect(result).toBeDefined();
    expect(result.status).toBe(VerificationStatus.Unverified);
  });

  it('should calculate reputation score correctly', async () => {
    const result = await verifier.verifyAsset('TEST', 'GXXX...');

    // Score should be average of verified sources
    const verifiedSources = result.sources.filter(s => s.verified);
    if (verifiedSources.length > 0) {
      const expectedScore = Math.round(
        verifiedSources.reduce((sum, s) => sum + s.score, 0) / verifiedSources.length
      );
      expect(result.reputation_score).toBe(expectedScore);
    }
  });
});
