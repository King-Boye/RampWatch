import { Pool } from 'pg';
import { AssetVerification, VerificationStatus, FxRate, FxRateRecord } from './types';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  max: 20,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

export async function initDatabase() {
  const client = await pool.connect();
  try {
    await client.query(`
      CREATE TABLE IF NOT EXISTS verified_assets (
        id SERIAL PRIMARY KEY,
        asset_code VARCHAR(12) NOT NULL,
        issuer VARCHAR(56) NOT NULL,
        status VARCHAR(20) NOT NULL,
        reputation_score INTEGER NOT NULL CHECK (reputation_score >= 0 AND reputation_score <= 100),
        last_verified TIMESTAMP NOT NULL DEFAULT NOW(),
        trustline_count BIGINT NOT NULL DEFAULT 0,
        has_toml BOOLEAN NOT NULL DEFAULT FALSE,
        stellar_expert_verified BOOLEAN DEFAULT FALSE,
        toml_data JSONB,
        community_reports INTEGER DEFAULT 0,
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
        UNIQUE(asset_code, issuer)
      );

      CREATE INDEX IF NOT EXISTS idx_asset_lookup ON verified_assets(asset_code, issuer);
      CREATE INDEX IF NOT EXISTS idx_status ON verified_assets(status);
      CREATE INDEX IF NOT EXISTS idx_last_verified ON verified_assets(last_verified);

      CREATE TABLE IF NOT EXISTS fx_rates (
        id SERIAL PRIMARY KEY,
        transaction_id VARCHAR(100) NOT NULL,
        rate DECIMAL(20, 8) NOT NULL,
        provider VARCHAR(100) NOT NULL,
        timestamp TIMESTAMP NOT NULL,
        from_currency VARCHAR(10) NOT NULL,
        to_currency VARCHAR(10) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        UNIQUE(transaction_id)
      );

      CREATE INDEX IF NOT EXISTS idx_fx_transaction ON fx_rates(transaction_id);
      CREATE INDEX IF NOT EXISTS idx_fx_timestamp ON fx_rates(timestamp);
      CREATE INDEX IF NOT EXISTS idx_fx_currencies ON fx_rates(from_currency, to_currency);
    `);
    console.log('Database initialized successfully');
  } finally {
    client.release();
  }
}

export async function saveAssetVerification(verification: AssetVerification): Promise<void> {
  const query = `
    INSERT INTO verified_assets (
      asset_code, issuer, status, reputation_score, last_verified,
      trustline_count, has_toml, stellar_expert_verified, toml_data, community_reports
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    ON CONFLICT (asset_code, issuer) 
    DO UPDATE SET
      status = EXCLUDED.status,
      reputation_score = EXCLUDED.reputation_score,
      last_verified = EXCLUDED.last_verified,
      trustline_count = EXCLUDED.trustline_count,
      has_toml = EXCLUDED.has_toml,
      stellar_expert_verified = EXCLUDED.stellar_expert_verified,
      toml_data = EXCLUDED.toml_data,
      community_reports = EXCLUDED.community_reports,
      updated_at = NOW()
  `;

  await pool.query(query, [
    verification.asset_code,
    verification.issuer,
    verification.status,
    verification.reputation_score,
    verification.last_verified,
    verification.trustline_count,
    verification.has_toml,
    verification.stellar_expert_verified || false,
    verification.toml_data ? JSON.stringify(verification.toml_data) : null,
    verification.community_reports || 0,
  ]);
}

export async function getAssetVerification(
  assetCode: string,
  issuer: string
): Promise<AssetVerification | null> {
  const query = `
    SELECT * FROM verified_assets 
    WHERE asset_code = $1 AND issuer = $2
  `;
  const result = await pool.query(query, [assetCode, issuer]);
  
  if (result.rows.length === 0) {
    return null;
  }

  const row = result.rows[0];
  return {
    asset_code: row.asset_code,
    issuer: row.issuer,
    status: row.status as VerificationStatus,
    reputation_score: row.reputation_score,
    last_verified: row.last_verified,
    trustline_count: parseInt(row.trustline_count),
    has_toml: row.has_toml,
    stellar_expert_verified: row.stellar_expert_verified,
    toml_data: row.toml_data,
    community_reports: row.community_reports,
  };
}

export async function getStaleAssets(hoursOld: number): Promise<AssetVerification[]> {
  const query = `
    SELECT * FROM verified_assets 
    WHERE last_verified < NOW() - INTERVAL '${hoursOld} hours'
    ORDER BY last_verified ASC
    LIMIT 100
  `;
  const result = await pool.query(query);
  
  return result.rows.map(row => ({
    asset_code: row.asset_code,
    issuer: row.issuer,
    status: row.status as VerificationStatus,
    reputation_score: row.reputation_score,
    last_verified: row.last_verified,
    trustline_count: parseInt(row.trustline_count),
    has_toml: row.has_toml,
    stellar_expert_verified: row.stellar_expert_verified,
    toml_data: row.toml_data,
    community_reports: row.community_reports,
  }));
}

export async function reportSuspiciousAsset(
  assetCode: string,
  issuer: string
): Promise<void> {
  const query = `
    UPDATE verified_assets 
    SET community_reports = community_reports + 1,
        updated_at = NOW()
    WHERE asset_code = $1 AND issuer = $2
  `;
  await pool.query(query, [assetCode, issuer]);
}

export async function getVerifiedAssets(limit: number = 100): Promise<AssetVerification[]> {
  const query = `
    SELECT * FROM verified_assets 
    WHERE status = 'verified'
    ORDER BY reputation_score DESC, trustline_count DESC
    LIMIT $1
  `;
  const result = await pool.query(query, [limit]);
  
  return result.rows.map(row => ({
    asset_code: row.asset_code,
    issuer: row.issuer,
    status: row.status as VerificationStatus,
    reputation_score: row.reputation_score,
    last_verified: row.last_verified,
    trustline_count: parseInt(row.trustline_count),
    has_toml: row.has_toml,
    stellar_expert_verified: row.stellar_expert_verified,
    toml_data: row.toml_data,
    community_reports: row.community_reports,
  }));
}

export async function saveFxRate(fxRate: FxRate): Promise<void> {
  const query = `
    INSERT INTO fx_rates (
      transaction_id, rate, provider, timestamp, from_currency, to_currency
    ) VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (transaction_id) DO NOTHING
  `;

  await pool.query(query, [
    fxRate.transaction_id,
    fxRate.rate,
    fxRate.provider,
    fxRate.timestamp,
    fxRate.from_currency,
    fxRate.to_currency,
  ]);
}

export async function getFxRate(transactionId: string): Promise<FxRateRecord | null> {
  const query = `
    SELECT * FROM fx_rates 
    WHERE transaction_id = $1
  `;
  const result = await pool.query(query, [transactionId]);
  
  if (result.rows.length === 0) {
    return null;
  }

  const row = result.rows[0];
  return {
    id: row.id,
    transaction_id: row.transaction_id,
    rate: parseFloat(row.rate),
    provider: row.provider,
    timestamp: row.timestamp,
    from_currency: row.from_currency,
    to_currency: row.to_currency,
    created_at: row.created_at,
  };
}

export { pool };
