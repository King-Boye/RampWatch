export enum VerificationStatus {
  Verified = 'verified',
  Unverified = 'unverified',
  Suspicious = 'suspicious',
}

export interface AssetVerification {
  asset_code: string;
  issuer: string;
  status: VerificationStatus;
  reputation_score: number;
  last_verified: Date;
  trustline_count: number;
  has_toml: boolean;
  stellar_expert_verified?: boolean;
  toml_data?: any;
  community_reports?: number;
}

export interface VerificationSource {
  name: string;
  verified: boolean;
  score: number;
  details?: any;
}

export interface VerificationResult {
  asset_code: string;
  issuer: string;
  status: VerificationStatus;
  reputation_score: number;
  sources: VerificationSource[];
  trustline_count: number;
  has_toml: boolean;
}

export interface FxRate {
  transaction_id: string;
  rate: number;
  provider: string;
  timestamp: Date;
  from_currency: string;
  to_currency: string;
}

export interface FxRateRecord {
  id: number;
  transaction_id: string;
  rate: number;
  provider: string;
  timestamp: Date;
  from_currency: string;
  to_currency: string;
  created_at: Date;
}
