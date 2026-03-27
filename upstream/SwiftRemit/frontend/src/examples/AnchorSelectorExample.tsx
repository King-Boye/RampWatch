import React, { useState } from 'react';
import { AnchorSelector, AnchorProvider } from '../AnchorSelector';

/**
 * AnchorSelectorExample - Demonstrates how to use the AnchorSelector component
 * 
 * This example shows:
 * - Basic usage with onSelect callback
 * - Currency filtering
 * - Custom API URL configuration
 * - Handling selected anchor data
 */
export const AnchorSelectorExample: React.FC = () => {
  const [selectedAnchor, setSelectedAnchor] = useState<AnchorProvider | null>(null);
  const [selectedCurrency, setSelectedCurrency] = useState<string>('USD');

  // Handle anchor selection
  const handleAnchorSelect = (anchor: AnchorProvider) => {
    setSelectedAnchor(anchor);
    console.log('Selected anchor:', anchor);
  };

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <h1>Anchor Provider Selection Example</h1>

      {/* Currency Filter */}
      <div style={{ marginBottom: '20px' }}>
        <label htmlFor="currency-select" style={{ display: 'block', marginBottom: '8px' }}>
          Select Currency:
        </label>
        <select
          id="currency-select"
          value={selectedCurrency}
          onChange={(e) => setSelectedCurrency(e.target.value)}
          style={{
            padding: '8px',
            borderRadius: '4px',
            border: '1px solid #ccc',
            fontSize: '14px',
          }}
        >
          <option value="USD">USD - US Dollar</option>
          <option value="EUR">EUR - Euro</option>
          <option value="GBP">GBP - British Pound</option>
          <option value="MXN">MXN - Mexican Peso</option>
          <option value="NGN">NGN - Nigerian Naira</option>
        </select>
      </div>

      {/* AnchorSelector Component */}
      {/* 
        Props:
        - onSelect: Callback function triggered when user selects an anchor
        - currency: Filter anchors by supported currency
        - apiUrl: Custom API endpoint (defaults to http://localhost:3000)
        - selectedAnchorId: Pre-select an anchor by ID (optional)
      */}
      <AnchorSelector
        onSelect={handleAnchorSelect}
        currency={selectedCurrency}
        apiUrl="http://localhost:3000"
      />

      {/* Display Selected Anchor Information */}
      {selectedAnchor && (
        <div
          style={{
            marginTop: '30px',
            padding: '20px',
            backgroundColor: '#f5f5f5',
            borderRadius: '8px',
            border: '1px solid #ddd',
          }}
        >
          <h2>Selected Anchor Details</h2>

          <div style={{ marginBottom: '15px' }}>
            <strong>Name:</strong> {selectedAnchor.name}
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Domain:</strong> {selectedAnchor.domain}
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Status:</strong>{' '}
            <span
              style={{
                padding: '4px 8px',
                borderRadius: '4px',
                backgroundColor:
                  selectedAnchor.status === 'active' ? '#d4edda' : '#f8d7da',
                color: selectedAnchor.status === 'active' ? '#155724' : '#721c24',
              }}
            >
              {selectedAnchor.status}
            </span>
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Withdrawal Fee:</strong>{' '}
            {selectedAnchor.fees.withdrawal_fee_percent}%
            {selectedAnchor.fees.withdrawal_fee_fixed &&
              ` + $${selectedAnchor.fees.withdrawal_fee_fixed.toFixed(2)}`}
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Transaction Limits:</strong> $
            {selectedAnchor.limits.min_amount.toLocaleString()} - $
            {selectedAnchor.limits.max_amount.toLocaleString()}
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Processing Time:</strong> {selectedAnchor.processing_time}
          </div>

          {selectedAnchor.rating && (
            <div style={{ marginBottom: '15px' }}>
              <strong>Rating:</strong> ⭐ {selectedAnchor.rating.toFixed(1)}/5.0
            </div>
          )}

          <div style={{ marginBottom: '15px' }}>
            <strong>KYC Required:</strong>{' '}
            {selectedAnchor.compliance.kyc_required ? 'Yes' : 'No'}
          </div>

          <div style={{ marginBottom: '15px' }}>
            <strong>Supported Currencies:</strong>{' '}
            {selectedAnchor.supported_currencies.join(', ')}
          </div>

          {/* Action Button Example */}
          <button
            onClick={() => {
              console.log('Proceeding with anchor:', selectedAnchor.name);
              // Here you would typically proceed with the remittance flow
            }}
            style={{
              marginTop: '15px',
              padding: '10px 20px',
              backgroundColor: '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '14px',
            }}
          >
            Proceed with {selectedAnchor.name}
          </button>
        </div>
      )}

      {/* Help Text */}
      <div
        style={{
          marginTop: '30px',
          padding: '15px',
          backgroundColor: '#e7f3ff',
          borderRadius: '4px',
          border: '1px solid #b3d9ff',
          fontSize: '14px',
        }}
      >
        <strong>How to use:</strong>
        <ul style={{ marginTop: '10px', marginBottom: '0' }}>
          <li>Select a currency from the dropdown above</li>
          <li>Click on an anchor provider to view details</li>
          <li>The selected anchor information will appear below</li>
          <li>Click "Proceed" to continue with the remittance flow</li>
        </ul>
      </div>
    </div>
  );
};

export default AnchorSelectorExample;
