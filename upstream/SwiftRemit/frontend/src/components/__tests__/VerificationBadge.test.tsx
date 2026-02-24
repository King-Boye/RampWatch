import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { VerificationBadge, VerificationStatus } from '../VerificationBadge';

describe('VerificationBadge', () => {
  const mockVerification = {
    asset_code: 'USDC',
    issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
    status: VerificationStatus.Verified,
    reputation_score: 95,
    last_verified: '2026-02-23T10:30:00Z',
    trustline_count: 15000,
    has_toml: true,
    stellar_expert_verified: true,
    community_reports: 0,
  };

  beforeEach(() => {
    global.fetch = vi.fn();
  });

  it('should render loading state initially', () => {
    render(<VerificationBadge assetCode="USDC" issuer="GXXX..." />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('should render verified badge', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockVerification,
    });

    render(<VerificationBadge assetCode="USDC" issuer="GXXX..." />);

    await waitFor(() => {
      expect(screen.getByText('Verified')).toBeInTheDocument();
      expect(screen.getByText('95/100')).toBeInTheDocument();
    });
  });

  it('should render unverified badge for 404', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      status: 404,
      ok: false,
    });

    render(<VerificationBadge assetCode="UNKNOWN" issuer="GXXX..." />);

    await waitFor(() => {
      expect(screen.getByText('Unverified')).toBeInTheDocument();
    });
  });

  it('should show modal on badge click', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockVerification,
    });

    render(<VerificationBadge assetCode="USDC" issuer="GXXX..." />);

    await waitFor(() => {
      expect(screen.getByText('Verified')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Verified'));

    await waitFor(() => {
      expect(screen.getByText('Asset Verification Details')).toBeInTheDocument();
    });
  });

  it('should trigger warning callback for suspicious assets', async () => {
    const onWarning = vi.fn();
    const suspiciousVerification = {
      ...mockVerification,
      status: VerificationStatus.Suspicious,
      reputation_score: 25,
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => suspiciousVerification,
    });

    render(
      <VerificationBadge
        assetCode="SCAM"
        issuer="GXXX..."
        onWarning={onWarning}
      />
    );

    await waitFor(() => {
      expect(onWarning).toHaveBeenCalledWith(suspiciousVerification);
    });
  });

  it('should show warning modal for suspicious assets', async () => {
    const suspiciousVerification = {
      ...mockVerification,
      status: VerificationStatus.Suspicious,
      reputation_score: 25,
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => suspiciousVerification,
    });

    render(<VerificationBadge assetCode="SCAM" issuer="GXXX..." />);

    await waitFor(() => {
      expect(screen.getByText('⚠ Asset Warning')).toBeInTheDocument();
    });
  });

  it('should handle report submission', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => mockVerification,
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ success: true }),
      });

    window.prompt = vi.fn().mockReturnValue('Suspicious activity');
    window.alert = vi.fn();

    render(<VerificationBadge assetCode="USDC" issuer="GXXX..." />);

    await waitFor(() => {
      expect(screen.getByText('Verified')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Verified'));

    await waitFor(() => {
      expect(screen.getByText('Report as Suspicious')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Report as Suspicious'));

    await waitFor(() => {
      expect(window.alert).toHaveBeenCalled();
    });
  });
});
