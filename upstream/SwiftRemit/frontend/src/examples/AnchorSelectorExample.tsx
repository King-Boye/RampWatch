import React, { useState } from 'react';
import { AnchorSelector, AnchorProvider } from '../components/AnchorSelector';

const mockAnchors: AnchorProvider[] = [
  {
    id: 'moneygram',
    name: 'MoneyGram',
    domain: 'moneygram.com',
    logo_url: '',
    description: 'Global money transfer service',
    status: 'active',
    fees: { deposit_fee_percent: 1.5, withdrawal_fee_percent: 1.0 },
    limits: { min_amount: 10, max_amount: 10000 },
    compliance: {
      kyc_required: true,
      kyc_level: 'basic',
      supported_countries: ['US', 'NG', 'DE'],
      restricted_countries: [],
      documents_required: ['ID'],
    },
    supported_currencies: ['USDC', 'NGN', 'EUR'],
    processing_time: '1-2 hours',
    verified: true,
  },
  {
    id: 'settle-network',
    name: 'Settle Network',
    domain: 'settlenetwork.com',
    logo_url: '',
    description: 'Crypto-native settlement layer',
    status: 'active',
    fees: { deposit_fee_percent: 0.5, withdrawal_fee_percent: 0.5 },
    limits: { min_amount: 1, max_amount: 50000 },
    compliance: {
      kyc_required: true,
      kyc_level: 'intermediate',
      supported_countries: ['US', 'EU'],
      restricted_countries: [],
      documents_required: ['ID', 'Proof of Address'],
    },
    supported_currencies: ['USDC', 'EUR'],
    processing_time: '< 30 minutes',
    verified: true,
  },
  {
    id: 'bitstamp',
    name: 'Bitstamp',
    domain: 'bitstamp.net',
    logo_url: '',
    description: 'Licensed European crypto exchange',
    status: 'active',
    fees: { deposit_fee_percent: 0.0, withdrawal_fee_percent: 0.9, withdrawal_fee_fixed: 0.9 },
    limits: { min_amount: 25, max_amount: 100000 },
    compliance: {
      kyc_required: true,
      kyc_level: 'advanced',
      supported_countries: ['US', 'EU', 'GB'],
      restricted_countries: [],
      documents_required: ['ID', 'Proof of Address', 'Source of Funds'],
    },
    supported_currencies: ['USDC', 'EUR', 'GBP'],
    processing_time: '1 business day',
    verified: true,
  },
];

// Intercept fetch so AnchorSelector works without a live API
const originalFetch = window.fetch;
window.fetch = (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
  const url = typeof input === 'string' ? input : input.toString();
  if (url.includes('/api/anchors')) {
    return Promise.resolve(
      new Response(JSON.stringify({ success: true, data: mockAnchors }), {
        headers: { 'Content-Type': 'application/json' },
      })
    );
  }
  return originalFetch(input, init);
};

export const AnchorSelectorExample: React.FC = () => {
  const [selectedAnchor, setSelectedAnchor] = useState<AnchorProvider | null>(null);

  return (
    <div style={{ padding: '24px', maxWidth: '560px' }}>
      <AnchorSelector onSelect={setSelectedAnchor} />
      <p style={{ marginTop: '16px', fontSize: '14px', color: '#555' }}>
        Selected: <strong>{selectedAnchor ? selectedAnchor.name : 'None'}</strong>
      </p>
    </div>
  );
};

export default AnchorSelectorExample;
